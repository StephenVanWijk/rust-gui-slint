pub mod address_init;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;
    let app_weak = app.as_weak();
    app.on_process_input(move |input| {
        if let Some(app) = app_weak.upgrade() {
            app.set_output(format!("You entered: {}", input).into());
        }
    });
    app.run()
}