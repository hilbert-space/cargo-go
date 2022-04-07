macro_rules! run(
    ($name:expr, $path:expr) => ({
        if ::std::process::Command::new($name).arg($path).spawn().is_ok() {
            return true;
        }
    });
);

pub fn open(path: &str) -> Result<(), String> {
    if run(path) {
        return Ok(());
    } else {
        raise!("cannot go to {:?}", path);
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn run(path: &str) -> bool {
    if let Ok(name) = ::std::env::var("BROWSER") {
        run!(name, path);
    }
    for name in ["xdg-open", "gnome-open", "kde-open"].iter() {
        run!(name, path);
    }
    false
}

#[cfg(target_os = "macos")]
fn run(path: &str) -> bool {
    run!("open", path);
    false
}

#[cfg(target_os = "windows")]
fn run(path: &str) -> bool {
    run!("explorer", path);
    false
}
