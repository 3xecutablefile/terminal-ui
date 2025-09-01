
#[cfg(target_os = "windows")]
compile_error!("Windows is not supported in this fork. Build on Linux or macOS.");

=======

use std::io::{self, Read, Write};

use anyhow::{Context, Result};
use portable_pty::{CommandBuilder, ExitStatus, NativePtySystem, PtySize, PtySystem};


pub struct ShellPrefs {
=======
use which::which;

pub struct ShellPrefs {
    pub prefer_pwsh: bool,

    pub login: bool,
}

impl Default for ShellPrefs {
    fn default() -> Self {

        Self { login: true }
=======
        Self {
            prefer_pwsh: true,
            login: true,
        }

    }
}

pub enum Sig {
    Int,
    Term,
    Quit,
}

pub struct PtyHandle {
    reader: Option<Box<dyn Read + Send>>,
    writer: Option<Box<dyn Write + Send>>,
    master: Box<dyn portable_pty::MasterPty + Send>,
    child: Box<dyn portable_pty::Child + Send>,
}

impl PtyHandle {
    pub fn take_reader(&mut self) -> Box<dyn Read + Send> {
        self.reader.take().unwrap()
    }
    pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
        if let Some(w) = &mut self.writer {
            w.write_all(data)
        } else {
            Ok(())
        }
    }
    pub fn close(&mut self) {
        self.writer.take();
    }
    pub fn resize(&mut self, cols: u16, rows: u16) -> Result<()> {
        self.master
            .resize(PtySize {
                cols,
                rows,
                pixel_width: 0,
                pixel_height: 0,
            })
            .context("resize pty")?;
        Ok(())
    }
    pub fn signal(&mut self, sig: Sig) -> Result<()> {
        if let Some(pid) = self.child.process_id() {
            send_signal(pid, sig)?;
        }
        Ok(())
    }
    pub fn wait(&mut self) -> Result<ExitStatus> {
        let status = self.child.wait().context("wait child")?;
        Ok(status)
    }
}

pub fn spawn_shell(cols: u16, rows: u16, prefs: ShellPrefs) -> Result<PtyHandle> {
    let pty_system = NativePtySystem::default();
    let pair = pty_system
        .openpty(PtySize {
            cols,
            rows,
            pixel_width: 0,
            pixel_height: 0,
        })
        .context("open pty")?;

    let mut cmd = build_shell_command(&prefs)?;
    cmd.env("TERM", "xterm-256color");

    let child = pair.slave.spawn_command(cmd).context("spawn command")?;
    let reader = Some(pair.master.try_clone_reader().context("clone reader")?);
    let writer = Some(pair.master.take_writer().context("take writer")?);
    Ok(PtyHandle {
        reader,
        writer,
        master: pair.master,
        child,
    })
}

fn build_shell_command(prefs: &ShellPrefs) -> Result<CommandBuilder> {

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into());
    let mut cmd = CommandBuilder::new(shell.clone());
    if prefs.login {
        if let Some(name) = std::path::Path::new(&shell)
            .file_name()
            .and_then(|s| s.to_str())
        {
            match name {
                "bash" | "zsh" => {
                    cmd.arg("-l");
                    cmd.arg("-i");
                }
                "fish" => {
                    cmd.arg("-l");
                }
                _ => {}
            }
        }
    }
    Ok(cmd)
}

=======
    if cfg!(windows) {
        let mut shells = vec!["cmd.exe"]; // default fallback
        if prefs.prefer_pwsh {
            shells.insert(0, "pwsh.exe");
            shells.insert(1, "powershell.exe");
        } else {
            shells.insert(0, "powershell.exe");
            shells.insert(1, "pwsh.exe");
        }
        let shell = shells
            .into_iter()
            .find(|s| which(s).is_ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".into()));
        Ok(CommandBuilder::new(shell))
    } else {
        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into());
        let mut cmd = CommandBuilder::new(shell.clone());
        if prefs.login {
            if let Some(name) = std::path::Path::new(&shell)
                .file_name()
                .and_then(|s| s.to_str())
            {
                match name {
                    "bash" | "zsh" => {
                        cmd.arg("-l");
                        cmd.arg("-i");
                    }
                    "fish" => {
                        cmd.arg("-l");
                    }
                    _ => {}
                }
            }
        }
        Ok(cmd)
    }
}

#[cfg(unix)]

fn send_signal(pid: u32, sig: Sig) -> Result<()> {
    use nix::sys::signal::{killpg, Signal};
    use nix::unistd::Pid;
    let signo = match sig {
        Sig::Int => Signal::SIGINT,
        Sig::Term => Signal::SIGTERM,
        Sig::Quit => Signal::SIGQUIT,
    };
    killpg(Pid::from_raw(pid as i32), signo).context("killpg")?;
    Ok(())
}

=======

#[cfg(windows)]
fn send_signal(pid: u32, sig: Sig) -> Result<()> {
    use windows_sys::Win32::System::Console::{
        GenerateConsoleCtrlEvent, CTRL_BREAK_EVENT, CTRL_C_EVENT,
    };
    let evt = match sig {
        Sig::Int => CTRL_C_EVENT,
        Sig::Term | Sig::Quit => CTRL_BREAK_EVENT,
    };
    unsafe {
        GenerateConsoleCtrlEvent(evt, pid);
    }
    Ok(())
}

