use data_sculptor::core::data_manager::DataManager;
use data_sculptor::gui;
use std::sync::{Arc, Mutex};

fn main()
{
    let data_manager: Arc<Mutex<DataManager>> =
        Arc::new(Mutex::from(DataManager { data: Vec::new() }));

    //GUI
    let gui_init_result = gui::gui_core::init(Arc::clone(&data_manager));

    if gui_init_result.is_err()
    {
        println!("There was a problem with setting up the gui: \n {}",
                 gui_init_result.unwrap_err());
    }
}

