slint::include_modules!();
use slint::Model;
mod services;
mod util;
use mf_core::{format::FormatType, job::ConvertJob, message::Message};
use slint::{ModelRc, SharedString, VecModel};
use std::{path::PathBuf, rc::Rc};
use util::filechooser;

use crate::util::convert::run_convert;

fn main() {
    let app = MainWindow::new().unwrap();
    app.set_app_version(SharedString::from(env!("CARGO_PKG_VERSION")));

    let files_model = Rc::new(VecModel::<SharedString>::default());
    app.set_selected_files(ModelRc::from(files_model.clone()));

    app.on_add_file_clicked(move || {
        let files_model = files_model.clone();
        slint::spawn_local(async move {
            if let Ok(Some(input_files)) = filechooser::open_dialog().await {
                for input_file in input_files {
                    files_model.push(SharedString::from(input_file.to_string_lossy().as_ref()));
                }
            }
        })
        .unwrap();
    });

    let app_weak = app.as_weak();
    app.on_convert_clicked(move || {
        let app = app_weak.unwrap();
        let ui_files = app.get_selected_files();
        let convert_jobs: Vec<ConvertJob> = ui_files
            .iter()
            .map(|file| ConvertJob {
                input: PathBuf::from(file.as_str()),
                output: None,
                format_type: FormatType::JPEG,
            })
            .collect();

        match run_convert(convert_jobs, None) {
            Ok(messages) => handle_messages(messages),
            Err(error) => eprintln!("[error] {}", error),
        }
    });

    app.run().unwrap();
}

fn handle_messages(messages: Vec<Message>) {
    for message in messages {
        match message {
            Message::Info(text) => println!("[info]    {}", text),
            Message::Success(text) => println!("[success] {}", text),
            Message::Warning(text) => println!("[warning] {}", text),
            Message::Error(text) => eprintln!("[error]   {}", text),
        }
    }
}
