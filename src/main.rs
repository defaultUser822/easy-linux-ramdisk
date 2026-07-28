use std::error::Error;
pub mod ramdisk;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_more(move |x| {
        let ui = ui_handle.unwrap();
        ui.set_num(ui.get_num() + x);
    });

    ui.run()?;

    Ok(())
}
