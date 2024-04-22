//! Defines the [`GUIMessage`] enum

/// Enum holding all types of messages to be received and handled by the gui application.
/// Sorted by the view module they belong to.
#[derive(Debug, Clone)]
pub enum GUIMessage
{
    // LIST LOAD VIEW
    SelectFile,
    FileSelected(String)
}