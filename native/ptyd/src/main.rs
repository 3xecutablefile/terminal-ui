use std::io::{self, BufRead, Read, Write};
use std::sync::{Arc, Mutex};

use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use serde::{Deserialize, Serialize};


=======
#[cfg(windows)]
use which::which;


#[derive(Deserialize)]
#[serde(tag = "t")]
enum ToPty {
    #[serde(rename = "i")]
    Input { data: String },
    #[serde(rename = "r")]
    Resize { cols: u16, rows: u16 },
    #[serde(rename = "s")]
    Signal { sig: String },
}

#[derive(Serialize)]
#[serde(tag = "t")]
enum FromPty {
    #[serde(rename = "o")]
    Output { data: String, seq: u64 },
    #[serde(rename = "x")]
    Exit { code: i32 },
}

fn main() -> Result<()> {
    let pty_system = NativePtySystem::default();
    let mut size = PtySize {
        cols: 80,
        rows: 24,
        pixel_width: 0,
        pixel_height: 0,
    };
    let pair = pty_system.openpty(size)?;

    let mut cmd = build_shell_command()?;
    #[cfg(unix)]
    {
        cmd.env("TERM", "xterm-256color");
    }

    let mut child = pair.slave.spawn_command(cmd)?;
    drop(pair.slave);
    let master = pair.master;
    let mut reader = master.try_clone_reader()?;
    let mut writer = master.take_writer()?;

    let stdout = Arc::new(Mutex::new(io::stdout()));
    let out = stdout.clone();
    std::thread::spawn(move || -> Result<()> {
        let mut buf = vec![0u8; 64 * 1024];
        let mut seq = 0u64;
        loop {
            let n = reader.read(&mut buf).context("read pty")?;
            if n == 0 {
                break;
            }
            let msg = FromPty::Output {
                data: B64.encode(&buf[..n]),
                seq,
            };
            seq += 1;
            let mut w = out.lock().unwrap();
            serde_json::to_writer(&mut *w, &msg)?;
            w.write_all(b"\n")?;
            w.flush()?;
        }
        Ok(())
    });

    for line in io::stdin().lock().lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let msg: ToPty = serde_json::from_str(&line)?;
        match msg {
            ToPty::Input { data } => {
                let bytes = B64.decode(data)?;
                writer.write_all(&bytes)?;
            }
            ToPty::Resize { cols, rows } => {
                size.cols = cols;
                size.rows = rows;
                master.resize(size)?;
            }
            ToPty::Signal { sig } => {
                forward_signal(&mut *child, &sig)?;
            }
        }
    }

    let status = child.wait()?;
    let code = status.exit_code() as i32;
    let mut w = stdout.lock().unwrap();
    let msg = FromPty::Exit { code };
    serde_json::to_writer(&mut *w, &msg)?;
    w.write_all(b"\n")?;
    w.flush()?;
    Ok(())
}

#[cfg(unix)]
fn build_shell_command() -> Result<CommandBuilder> {
    use std::env;
    let sh = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into());
    let mut c = CommandBuilder::new(sh.clone());
    let name = std::path::Path::new(&sh)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    if matches!(name, "bash" | "zsh") {
        c.arg("-l");
        c.arg("-i");
    } else if name == "fish" {
        c.arg("-l");
    } else {
=======
    c.arg(match name {
        "bash" | "zsh" => "-l",
        "fish" => "-l",
        _ => "-i",
    });
    if matches!(name, "bash" | "zsh") {

        c.arg("-i");
    }
    Ok(c)
}

#[cfg(windows)]
fn build_shell_command() -> Result<CommandBuilder> {

    let exe = if which::which("pwsh.exe").is_ok() {
        "pwsh.exe"
    } else if which::which("powershell.exe").is_ok() {
=======
    let choice = if which("pwsh.exe").is_ok() {
        "pwsh.exe"
    } else if which("powershell.exe").is_ok() {

        "powershell.exe"
    } else {
        std::env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".into())
    };

    Ok(CommandBuilder::new(exe))
}

#[cfg(unix)]
fn forward_signal(child: &mut dyn portable_pty::Child, sig: &str) -> Result<()> {
    use nix::sys::signal::{kill, Signal};
    use nix::unistd::Pid;
    let pid = child.process_id().ok_or_else(|| anyhow!("no child pid"))? as i32;
    let signo = match sig {
        "INT" => Signal::SIGINT,
        "TERM" => Signal::SIGTERM,
        "QUIT" => Signal::SIGQUIT,
        _ => Signal::SIGINT,
    };
    kill(Pid::from_raw(pid), signo).ok();
    Ok(())
}

#[cfg(windows)]
fn forward_signal(_child: &mut dyn portable_pty::Child, _sig: &str) -> Result<()> {
    // ConPTY surfaces Ctrl-C to many apps; no explicit signal handling needed.
=======
    Ok(CommandBuilder::new(choice))
}

fn forward_signal(child: &mut dyn portable_pty::Child, sig: &str) -> Result<()> {
    #[cfg(unix)]
    {
        use nix::sys::signal::{kill, Signal};
        use nix::unistd::Pid;
        let pid = child.process_id().ok_or_else(|| anyhow!("no child pid"))? as i32;
        let signo = match sig {
            "INT" => Signal::SIGINT,
            "TERM" => Signal::SIGTERM,
            "QUIT" => Signal::SIGQUIT,
            _ => Signal::SIGINT,
        };
        kill(Pid::from_raw(pid), signo).ok();
    }
    #[cfg(windows)]
    {
        let _ = sig; // ConPTY handles Ctrl-C for many apps; no-op
    }

    Ok(())
}
