use ptycore::{spawn_shell, ShellPrefs};
use std::io::Read;

#[test]
fn spawn_prints_shell() {
    let mut handle = spawn_shell(80, 24, ShellPrefs::default()).expect("spawn shell");
    #[cfg(unix)]
    let cmd = b"echo $SHELL\nexit\n".to_vec();
    #[cfg(windows)]
    let cmd = b"echo $PSVersionTable.PSVersion\nexit\n".to_vec();
    handle.write(&cmd).expect("write");
    handle.close();
    let mut reader = handle.take_reader();
    let mut out = Vec::new();
    reader.read_to_end(&mut out).expect("read");
    handle.wait().ok();
    let output = String::from_utf8_lossy(&out);
    #[cfg(unix)]
    assert!(output.contains(&std::env::var("SHELL").unwrap_or_default()));
    #[cfg(windows)]
    assert!(output.contains("PSVersion"));
}
