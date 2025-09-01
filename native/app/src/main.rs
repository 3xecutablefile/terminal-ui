use anyhow::{Context, Result};
use crossbeam_channel::{unbounded, Receiver};
use ptycore::{spawn_shell, ShellPrefs};
use std::io::Read;
use std::sync::{Arc, Mutex};
use wgpu::SurfaceError;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

mod gfx;
mod term;
mod theme;
mod ui;
use gfx::Renderer;
use term::Emu;
use ui::panels::Panels;
use ui::theme_switcher::{Action as TSAction, Key as TKey, Page as TPage, ThemeSwitcher};

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    _pty: Arc<Mutex<ptycore::PtyHandle>>,
    rx: Receiver<Vec<u8>>,
    emu: Emu,
    renderer: Renderer,
    theme: theme::Theme,
    switcher: ThemeSwitcher,
    panels: Panels,
    cell_width: f64,
    cell_height: f64,
    scale_factor: f64,
}

impl State {
    async fn new(window: &winit::window::Window) -> Result<Self> {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();
        let surface = unsafe { instance.create_surface(window)? };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .context("request adapter")?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await?;
        let caps = surface.get_capabilities(&adapter);
        let format = caps.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let cols = 80;
        let rows = 24;
        let mut handle = spawn_shell(cols, rows, ShellPrefs::default())?;
        let mut reader = handle.take_reader();
        let (tx, rx) = unbounded();
        std::thread::spawn(move || {
            let mut buf = [0u8; 4096];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) | Err(_) => break,
                    Ok(n) => {
                        let _ = tx.send(buf[..n].to_vec());
                    }
                }
            }
        });
        let pty = Arc::new(Mutex::new(handle));

        let emu = Emu::new(cols as usize, rows as usize);
        let renderer = Renderer::new();
        let theme = theme::load_theme("tron")?;
        let switcher = ThemeSwitcher::new();
        let panels = Panels::new();

        let scale_factor = window.scale_factor();
        let cell_width = size.width as f64 / cols as f64;
        let cell_height = size.height as f64 / rows as f64;

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size,
            _pty: pty,
            rx,
            emu,
            renderer,
            theme,
            switcher,
            panels,
            cell_width,
            cell_height,
            scale_factor,
        })
    }

    fn resize(
        &mut self,
        new_size: winit::dpi::PhysicalSize<u32>,
        scale_factor: Option<f64>,
    ) {
        if new_size.width > 0 && new_size.height > 0 {
            if let Some(sf) = scale_factor {
                let ratio = sf / self.scale_factor;
                self.scale_factor = sf;
                self.cell_width *= ratio;
                self.cell_height *= ratio;
            }
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.renderer.resize(new_size.width, new_size.height);
            let cols = (new_size.width as f64 / self.cell_width).floor().max(1.0) as u16;
            let rows = (new_size.height as f64 / self.cell_height).floor().max(1.0) as u16;
            if let Ok(mut pty) = self._pty.lock() {
                let _ = pty.resize(cols, rows);
            }
            self.emu.resize(cols as usize, rows as usize);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if input.state == ElementState::Pressed {
                    #[allow(deprecated)]
                    if is_toggle_theme(input.modifiers, input.virtual_keycode) {
                        self.switcher.toggle(theme::list_themes);
                        return true;
                    }
                    if self.switcher.is_open() {
                        let act = match input.virtual_keycode {
                            Some(VirtualKeyCode::Up) => self.switcher.handle_key(TKey::Up),
                            Some(VirtualKeyCode::Down) => self.switcher.handle_key(TKey::Down),
                            Some(VirtualKeyCode::Escape) => self.switcher.handle_key(TKey::Escape),
                            Some(VirtualKeyCode::Return) | Some(VirtualKeyCode::NumpadEnter) => {
                                self.switcher.handle_key(TKey::Enter)
                            }
                            Some(VirtualKeyCode::PageUp) => {
                                self.switcher.handle_key_page(TPage::Up)
                            }
                            Some(VirtualKeyCode::PageDown) => {
                                self.switcher.handle_key_page(TPage::Down)
                            }
                            _ => TSAction::None,
                        };
                        if let TSAction::Apply(name) = act {
                            if let Ok(th) = theme::load_theme(&name) {
                                self.theme = th;
                            }
                        }
                        return true;
                    }
                    if input.virtual_keycode == Some(VirtualKeyCode::F1) {
                        if let Ok(mut pty) = self._pty.lock() {
                            let _ = pty.write(b"nmap --version\n");
                        }
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }

    fn update(&mut self) {
        while let Ok(bytes) = self.rx.try_recv() {
            self.emu.on_bytes(&bytes);
        }
        self.panels.tick();
    }

    fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(theme::parse_color(
                            &self.theme.terminal.background,
                        )),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }
        self.renderer
            .draw_neon_grid(&mut encoder, self.size.width, self.size.height, &self.theme);
        self.renderer
            .draw_scanlines(&mut encoder, self.size.width, self.size.height, &self.theme);
        let pw = 320.0;
        let px = self.size.width as f32 - pw - 24.0;
        self.renderer
            .draw_side_panel(&mut encoder, px, pw, self.size.height as f32, &self.theme);
        self.renderer.draw_bar(
            &mut encoder,
            px + 24.0,
            64.0,
            pw - 48.0,
            16.0,
            self.panels.cpu_percent,
            "CPU",
            &self.theme,
        );
        self.renderer.draw_bar(
            &mut encoder,
            px + 24.0,
            112.0,
            pw - 48.0,
            16.0,
            self.panels.mem_percent,
            "RAM",
            &self.theme,
        );
        if self.switcher.is_open() {
            let (w, h) = (self.size.width as f32, self.size.height as f32);
            let layout = self.switcher.layout(w, h);
            let (panel, rows) = self.switcher.render_items(&layout, w, h);
            self.renderer
                .draw_theme_overlay_begin(&mut encoder, panel, &self.theme);
            self.renderer
                .draw_theme_overlay_rows(&mut encoder, &rows, &self.theme);
        }
        self.queue.submit(Some(encoder.finish()));
        output.present();
        let _ = self.renderer.draw();
        Ok(())
    }
}

fn is_toggle_theme(mods: winit::event::ModifiersState, key: Option<VirtualKeyCode>) -> bool {
    key == Some(VirtualKeyCode::T) && (mods.ctrl() || mods.logo()) && mods.shift()
}

fn main() -> Result<()> {
    if std::env::args().any(|a| a == "--version") {
        println!("{} ({})", env!("CARGO_PKG_VERSION"), env!("GIT_SHA"));
        return Ok(());
    }
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_decorations(false)
        .build(&event_loop)?;
    let mut state = pollster::block_on(State::new(&window))?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                if !state.input(&event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(size) => state.resize(size, None),
                        WindowEvent::ScaleFactorChanged { new_inner_size, scale_factor, .. } => {
                            state.resize(*new_inner_size, Some(scale_factor))
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(_) => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size, None),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("render error: {e:?}"),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
