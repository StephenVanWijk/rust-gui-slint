// Prevent console window in addition to Slint window in Windows release builds when, e.g., 
// starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

slint::slint!{
    import { TextEdit } from "std-widgets.slint";

    // The exported component should inherit from Window to avoid the deprecation warning.
    export component ExportedComponent inherits Window {
        TextEdit {}
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()?;

    Ok(())
}
