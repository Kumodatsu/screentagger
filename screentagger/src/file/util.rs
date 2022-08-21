#[cfg(target_os = "linux")]
pub fn reveal_file(path: &str) {
  std::process::Command::new("xdg-open")
    .arg(path)
    .spawn()
    .expect("Failed to reveal file.");
}

#[cfg(target_os = "windows")]
pub fn reveal_file(path: &str) {
  std::process::Command::new("explorer")
    .arg("/select,")
    .arg(path.replace("/", "\\"))
    .spawn()
    .expect("Failed to reveal file.");
}

#[cfg(target_os = "macos")]
pub fn reveal_file(path: &str) {
  std::process::Command::new("open")
    .arg(path)
    .spawn()
    .expect("Failed to reveal file.");
}
