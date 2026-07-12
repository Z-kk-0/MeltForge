slint::include_modules!();
use slint::{Model, platform::WindowEvent};
use i_slint_backend_winit::{EventResult, WinitWindowAccessor};
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
fn is_supported_format(path: &PathBuf) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    matches!(ext.as_str(), "png" | "jpg" | "jpeg")
}
fn is_image_format(path: &PathBuf) -> bool {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();
    matches!(
        ext.as_str(),
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "tiff" | "tif" | "ico"
    )
}
fn load_thumbnail(path: &PathBuf) -> (bool, slint::Image) {
    if is_image_format(path) {
        if let Ok(image) = slint::Image::load_from_path(path) {
            return (true, image);
        }
    }
    (false, slint::Image::default())
}
fn main() {
    let app = MainWindow::new().unwrap();
    app.set_app_version(SharedString::from(env!("CARGO_PKG_VERSION")));

    let files_model = Rc::new(VecModel::<FileEntry>::default());
    app.set_files(ModelRc::from(files_model.clone()));

    let toasts_model = Rc::new(VecModel::<ToastMessage>::default());
    app.set_toasts(ModelRc::from(toasts_model.clone()));

    let files_model_add = files_model.clone();
    app.on_add_file_clicked(move || {
        let files_model = files_model_add.clone();
        slint::spawn_local(async move {
            if let Ok(Some(input_files)) = filechooser::open_dialog().await {
                for path in input_files {
                    let formats = available_formats(&path);
                    let default_format = formats.first().copied().unwrap_or("JPG");
                    let (is_image, image) = load_thumbnail(&path);
                    let entry = FileEntry {
                        path: SharedString::from(path.to_string_lossy().as_ref()),
                        selected_format: SharedString::from(default_format),
                        available_formats: ModelRc::new(VecModel::from(
                            formats
                                .iter()
                                .map(|s| SharedString::from(*s))
                                .collect::<Vec<_>>(),
                        )),
                        is_image,
                        image,
                    };
                    files_model.push(entry);
                }
            }
        })
        .unwrap();
    });
    let files_model_drop = files_model.clone();
    let toasts_model_drop = toasts_model.clone();
    app.window()
    .on_winit_window_event(move |window, event| match event {
        winit::event::WindowEvent::DroppedFile(path) => {
            if !is_supported_format(&path) {
                toasts_model_drop.push(ToastMessage {
                    kind: SharedString::from("error"),
                    title: SharedString::from("Unsupported file"),
                    message: SharedString::from(format!(
                        "\"{}\" is not a supported format",
                        path.file_name().unwrap_or_default().to_string_lossy()
                    )),
                });
                return EventResult::PreventDefault;
            }
            let formats = available_formats(&path);
            let default_format = formats.first().copied().unwrap_or("JPG");
            let (is_image, image) = load_thumbnail(&path);
            let entry = FileEntry {
                path: SharedString::from(path.to_string_lossy().as_ref()),
                selected_format: SharedString::from(default_format),
                available_formats: ModelRc::new(VecModel::from(
                    formats.iter().map(|s| SharedString::from(*s)).collect::<Vec<_>>(),
                )),
                is_image,
                image,
            };
            files_model_drop.push(entry);

            toasts_model_drop.push(ToastMessage {
                kind: SharedString::from("info"),
                title: SharedString::from("File added"),
                message: SharedString::from(
                    path.file_name().unwrap_or_default().to_string_lossy().as_ref(),
                ),
            });

            EventResult::PreventDefault
        }
        _ => EventResult::Propagate,
    });

    let files_model_fmt = files_model.clone();
    app.on_format_changed(move |index, format| {
        if let Some(mut entry) = files_model_fmt.row_data(index as usize) {
            entry.selected_format = format;
            files_model_fmt.set_row_data(index as usize, entry);
        }
    });

    let toasts_model_dismiss = toasts_model.clone();
    app.on_dismiss_toast(move |index| {
        toasts_model_dismiss.remove(index as usize);
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

        let handle = app.as_weak();
        std::thread::spawn(move || match run_convert(convert_jobs, None) {
            Ok(messages) => {
                slint::invoke_from_event_loop(move || {
                    if let Some(app) = handle.upgrade() {
                        push_toasts(&app, messages);
                    }
                })
                .unwrap();
            }
            Err(error) => eprintln!("[error] {}", error),
        });
    });

    app.run().unwrap();
}

fn push_toasts(app: &MainWindow, messages: Vec<Message>) {
    if let Some(model) = app
        .get_toasts()
        .as_any()
        .downcast_ref::<VecModel<ToastMessage>>()
    {
        for msg in messages {
            let (kind, title, body) = match msg {
                Message::Success(text) => ("success", "Conversion complete", text.to_string()),
                Message::Error(text) => ("error", "Conversion failed", text.to_string()),
                Message::Warning(text) => ("warning", "Warning", text.to_string()),
                Message::Info(text) => ("info", "Info", text.to_string()),
            };
            model.push(ToastMessage {
                kind: SharedString::from(kind),
                title: SharedString::from(title),
                message: SharedString::from(body.as_str()),
            });
        }
    }
}
