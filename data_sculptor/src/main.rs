use std::process;

mod gui;
mod data;
mod file_io;

fn main()
{
    //GUI
    let gui_init_result = gui::gui_core::init();

    if gui_init_result.is_err()
    {
        println!("There was a problem with setting up the gui: \n {}",
                 gui_init_result.unwrap_err());
        process::exit(1);
    }
}

