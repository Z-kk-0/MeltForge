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
        .map_err(map_dialog_err)?;

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
        .map_err(map_dialog_err)?;

    if yes {
        Ok(Some(path))
    } else {
        Ok(None)
    }
}

fn map_dialog_err(err: DialogError) -> MeltforgeError {
    let msg = match err {
        DialogError::Io(e) => format!("native dialog I/O error: {}", e),
        DialogError::Utf8(e) => format!("native dialog utf-8 error: {}", e),
        DialogError::MissingDep => "native dialog dependency missing".to_string(),
        DialogError::Killed(sig) => format!("native dialog subprocess killed: {:?}", sig),
        DialogError::Other(e) => format!("native dialog error: {}", e),
    };
    MeltforgeError::Io(IoError::InvalidOutput(msg))
}
