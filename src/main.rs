// Prevent console window in addition to Slint window in Windows release builds when, e.g., 
// starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, rc::Rc};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let model = Rc::new(slint::VecModel::from(
        vec![
            // TodoItem{ text: "Ghi".into(), checked: false,},
            // TodoItem{ text: "Jki".into(), checked: false,},
        ]
    ));

    {   
        let model = model.clone(); 
        ui.on_add_todo(move |s|{
        model.push(TodoItem{ text: s, checked: false });
        });
    }

    ui.set_todos(Into::into(model));
    ui.run()?;

    Ok(())
}
