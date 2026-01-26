slint::include_modules!();
mod services;
mod util;
fn main() {
    let app = MainWindow::new().unwrap();
    app.set_app_version(slint::SharedString::from(env!("CARGO_PKG_VERSION")));
    app.run();
}

