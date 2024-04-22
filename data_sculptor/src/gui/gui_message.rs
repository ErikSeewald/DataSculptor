#[derive(Debug, Clone)]
pub enum GUIMessage
{
    // LIST LOAD VIEW
    SelectFile,
    FileSelected(String)
}