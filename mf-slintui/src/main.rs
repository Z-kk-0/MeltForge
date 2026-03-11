slint::include_modules!();
mod services;
mod util;
use slint::{ModelRc, SharedString, VecModel};
use std::rc::Rc;
use util::filechooser;

fn main() {
    let app = MainWindow::new().unwrap();
    app.set_app_version(SharedString::from(env!("CARGO_PKG_VERSION")));

    let files_model = Rc::new(VecModel::<SharedString>::default());
    app.set_selected_files(ModelRc::from(files_model.clone()));

    app.on_add_file_clicked(move || {
        let files_model = files_model.clone();
        slint::spawn_local(async move {
            if let Ok(Some(input_file)) = filechooser::open_dialog().await {
                files_model.push(SharedString::from(input_file.to_string_lossy().as_ref()));
            }
        })
        .unwrap();
    });

    app.run().unwrap();
}
