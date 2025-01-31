//! Handles io of the /assets/ folder.

use iced::{Task, window};
use iced::window::{Icon, icon};
use crate::gui::gui_message::GUIMessage;

/// Returns a task that will initialize the assets used by the GUI
pub fn init_assets_task() -> Task<GUIMessage>
{
    let set_icon: Task<GUIMessage> = window::get_latest()
        .then(|opt_id| {
            if let (Some(id), Some(icon)) = (opt_id, load_icon()) {window::change_icon(id, icon)}
            else {Task::none()}
        });

    return set_icon;
}

fn load_icon() -> Option<Icon>
{
    let img_bytes = include_bytes!("../../assets/icon.png");
    let img = image::load_from_memory(img_bytes).ok()?;
    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();

    icon::from_rgba(rgba_img.into_raw(), width, height).ok()
}