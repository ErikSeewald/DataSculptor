//! Module specifying all file dialogs used in data_sculptor

use rfd::FileDialog;

/// Opens a new file dialog and optionally returns a file path if a file is chosen
pub fn pick_file() -> Option<String>
{
    let file = FileDialog::new().pick_file();
    file.map(|f| f.as_path().to_string_lossy().into_owned())
}

/// Opens a new save dialogue for json files and optionally returns a file path if one is chosen
pub fn save_json_file() -> Option<String>
{
    let file = FileDialog::new()
        .add_filter("Json Files (*.json)", &["json"])
        .save_file();
    file.map(|f| f.as_path().to_string_lossy().into_owned())
}