use data_sculptor::core::data_manager::DataManager;
use data_sculptor::gui::gui_core;
use std::sync::{Arc, Mutex};

/// Main program entry for data_sculptor. Builds the shared [`DataManager`]
/// and distributes it among the submodules (i.e. gui) that it constructs.
fn main()
{
    let data_manager = DataManager { data: Vec::new()};
    let arc_data_manager: Arc<Mutex<DataManager>> = Arc::new(Mutex::from(data_manager));

    //GUI
    let gui_init_result = gui_core::init(Arc::clone(&arc_data_manager));
    if gui_init_result.is_err()
    {
        println!("There was a problem with setting up the gui: \n {}",
                 gui_init_result.unwrap_err());
    }
}

