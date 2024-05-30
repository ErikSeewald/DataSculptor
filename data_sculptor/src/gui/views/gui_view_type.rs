//! Module defining the [`GUIViewType`] enum

/// Enum holding all gui views supported by data_sculptor.
/// Each value directly corresponds to a 'views' submodule.
pub enum GUIViewType
{
    ListView,
    FilterView,
    MenuView
}