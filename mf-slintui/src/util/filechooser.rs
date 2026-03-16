use mf_core::validation::error::{IoError, MeltforgeError};
use native_dialog::{DialogBuilder, Error as DialogError, MessageLevel};
use std::path::PathBuf;

pub async fn open_dialog() -> Result<Option<PathBuf>, MeltforgeError> {
    let path = DialogBuilder::file()
        .set_location("~/Desktop")
        .add_filter("Pictures", &["png", "jpg", "jpeg"])
        .add_filter("PNG", &["png"])
        .add_filter("JPEG", &["jpg", "jpeg"])
        .open_single_file()
        .show()
        .map_err(map_dialog_error)?;

    let Some(path) = path else {
        return Ok(None);
    };

    let yes = DialogBuilder::message()
        .set_level(MessageLevel::Info)
        .set_title("Do you want to open the file?")
        .set_text(format!("{:#?}", path))
        .confirm()
        .spawn()
        .await
        .map_err(map_dialog_error)?;

    if yes {
        Ok(Some(path))
    } else {
        Ok(None)
    }
}

fn map_dialog_error(dialog_error: DialogError) -> MeltforgeError {
    let message = match dialog_error {
        DialogError::Io(error) => format!("native dialog I/O error: {}", error),
        DialogError::Utf8(error) => format!("native dialog utf-8 error: {}", error),
        DialogError::MissingDep => "native dialog dependency missing".to_string(),
        DialogError::Killed(signal) => format!("native dialog subprocess killed: {:?}", signal),
        DialogError::Other(error) => format!("native dialog error: {}", error),
    };
    MeltforgeError::Io(IoError::InvalidOutput(message))
}
