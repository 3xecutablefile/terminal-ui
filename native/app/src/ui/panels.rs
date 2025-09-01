use std::time::{Duration, Instant};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub struct Panels {
    sys: System,
    pub cpu_percent: f32,
    pub mem_percent: f32,
    last_sample: Instant,
    cadence: Duration,
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
            cpu_percent: 0.0,
            mem_percent: 0.0,
            last_sample: Instant::now() - Duration::from_millis(1000),
            cadence: Duration::from_millis(350),
        };
        p.tick();
        p
    }

    pub fn tick(&mut self) {
        // Avoid clippy float-eq lint via checked elapsed
        if self.last_sample.elapsed() < self.cadence {
            return;
        }
        self.last_sample = Instant::now();
        self.sys.refresh_cpu();
        self.sys.refresh_memory();
        self.cpu_percent = self.sys.global_cpu_info().cpu_usage();
        let total = self.sys.total_memory() as f32;
        self.mem_percent = if total > 0.0 {
            (self.sys.used_memory() as f32 / total) * 100.0
        } else {
            0.0
        };
    }
}
