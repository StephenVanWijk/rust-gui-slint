pub mod address_init;
pub mod address_oper;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let address_infterface = address_init::Address::address_init();
    println!("Address initialized: {:?}\n", address_infterface);

    let app = AppWindow::new()?;

    // Populate the ComboBox with data
    // app.set_combo_items(slint::ModelRc::from([
    //     "De heer ".into(),
    //     "Mevrouw ".into(),
    // ]));

    app.run()
}