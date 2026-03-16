slint::include_modules!();
use slint::Model;
mod services;
mod util;
use mf_core::{format::FormatType, job::ConvertJob, message::Message};
use slint::{ModelRc, SharedString, VecModel};
use std::{path::PathBuf, rc::Rc};
use util::filechooser;

use crate::util::convert::run_convert;

fn available_formats(path: &PathBuf) -> Vec<&'static str> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    match ext.as_str() {
        "png" => vec!["JPG"],
        "jpg" | "jpeg" => vec!["PNG"],
        _ => vec!["JPG", "PNG"],
    }
}

fn main() {
    let app = MainWindow::new().unwrap();
    app.set_app_version(SharedString::from(env!("CARGO_PKG_VERSION")));

    let files_model = Rc::new(VecModel::<FileEntry>::default());
    app.set_files(ModelRc::from(files_model.clone()));

    let files_model_add = files_model.clone();
    app.on_add_file_clicked(move || {
        let files_model = files_model_add.clone();
        slint::spawn_local(async move {
            if let Ok(Some(input_files)) = filechooser::open_dialog().await {
                for path in input_files {
                    let formats = available_formats(&path);
                    let default_format = formats.first().copied().unwrap_or("JPG");
                    let entry = FileEntry {
                        path: SharedString::from(path.to_string_lossy().as_ref()),
                        selected_format: SharedString::from(default_format),
                        available_formats: ModelRc::new(VecModel::from(
                            formats.iter().map(|s| SharedString::from(*s)).collect::<Vec<_>>(),
                        )),
                    };
                    files_model.push(entry);
                }
            }
        })
        .unwrap();
    });

    let files_model_fmt = files_model.clone();
    app.on_format_changed(move |index, format| {
        if let Some(mut entry) = files_model_fmt.row_data(index as usize) {
            entry.selected_format = format;
            files_model_fmt.set_row_data(index as usize, entry);
        }
    });

    let app_weak = app.as_weak();
    app.on_convert_clicked(move || {
        let app = app_weak.unwrap();
        let ui_files = app.get_files();
        let convert_jobs: Vec<ConvertJob> = ui_files
            .iter()
            .map(|entry| ConvertJob {
                input: PathBuf::from(entry.path.as_str()),
                output: None,
                format_type: match entry.selected_format.as_str() {
                    "PNG" => FormatType::PNG,
                    _ => FormatType::JPEG,
                },
            })
            .collect();

        std::thread::spawn(move || {
            match run_convert(convert_jobs, None) {
                Ok(messages) => {
                    slint::invoke_from_event_loop(move || handle_messages(messages)).unwrap();
                }
                Err(error) => eprintln!("[error] {}", error),
            }
        });
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
