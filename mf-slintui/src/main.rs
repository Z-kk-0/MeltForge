slint::include_modules!();
use slint::Model; 
mod services;
mod util;
use mf_core::{ format::FormatType, job::ConvertJob};
use slint::{ModelRc, SharedString, VecModel};
use std::{path::{PathBuf}, rc::Rc};
use util::filechooser;

use crate::util::convert::run_multi_convert_job;

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

    let app_weak = app.as_weak();
    app.on_convert_clicked( move || {
        let app = app_weak.unwrap();
        let ui_files = app.get_selected_files();
        let convert_jobs:  Vec<ConvertJob> = ui_files.iter()
        .map(|file| {
            let input = PathBuf::from(file.as_str());
            ConvertJob {
                input: input.clone(),
                output: None,
                format_type: FormatType::JPEG
            }
        })
        .collect();
        match run_multi_convert_job(convert_jobs, None) {
            Ok(failures) if failures == 0 => println!("Conversion success"),
            Ok(failures) => println!("{} Conversion failures", failures), 
            Err(error) => eprintln!("Error: {}", error)
        }
    });


    app.run().unwrap();
}
