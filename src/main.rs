// Prevent console window in addition to Slint window in Windows release builds when, e.g., 
// starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, rc::Rc};

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui  = AppWindow::new()?;

    let model_a: Rc<slint::VecModel<_>> = Rc::new(slint::VecModel::from(
        vec![

        ]
    ));

    {   
        let model_b: Rc<slint::VecModel<_>> = model_a.clone(); 
        ui.on_add_todo(move |s|{
        model_b.push(TodoItem{ text: s, checked: false });
        });
    }

    ui.set_todos(Into::into(model_a));
    ui.run()?;

    Ok(())
}
