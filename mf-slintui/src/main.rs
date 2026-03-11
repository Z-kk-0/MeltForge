slint::include_modules!();
mod services;
mod util;
use slint::{SharedString, VecModel};
use util::filechooser;
fn main() {
    let app = MainWindow::new().unwrap();
    app.set_app_version(slint::SharedString::from(env!("CARGO_PKG_VERSION")));
    let files_model = Rc::new(VecModel::<SharedString>::default());
    app.on_add_file_clicked(move || {
        let OK(Some(input_file)) = filechooser::open_dialog().await {
            
        }
    });
    app.run();
}

