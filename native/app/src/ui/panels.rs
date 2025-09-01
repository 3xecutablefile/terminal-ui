use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub struct Panels {
    sys: System,
    pub cpu_use: f32,
    pub mem_use: f32,
}

impl Panels {
    pub fn new() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::new().with_cpu_usage())
                .with_memory(MemoryRefreshKind::new()),
        );
        sys.refresh_cpu();
        sys.refresh_memory();
        let mut p = Panels {
            sys,
            cpu_use: 0.0,
            mem_use: 0.0,
        };
        p.update();
        p
    }

    pub fn update(&mut self) {
        self.sys.refresh_cpu();
        self.sys.refresh_memory();
        self.cpu_use = self.sys.global_cpu_info().cpu_usage() / 100.0;
        let total = self.sys.total_memory() as f32;
        self.mem_use = if total > 0.0 {
            self.sys.used_memory() as f32 / total
        } else {
            0.0
        };
    }
}
