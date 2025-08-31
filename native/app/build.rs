use std::process::Command;

fn main() {
    if let Ok(output) = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
    {
        let hash = String::from_utf8_lossy(&output.stdout);
        println!("cargo:rustc-env=GIT_SHA={}", hash.trim());
    }
}
