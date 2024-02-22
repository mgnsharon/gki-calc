slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_calculate_gki({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let g_reading: f32 = match ui.get_g_reading().trim().parse() {
                Ok(v) => match ui.get_unit_type().as_str() {
                    "mg/dL" => v / 18.0,
                    _ => v,
                },
                Err(_) => 0.0,
            };
            let k_reading: f32 = ui.get_k_reading().trim().parse().unwrap_or(0.0);
            if g_reading > 0.0 && k_reading > 0.0 {
                ui.set_gki_result(format!("{:.2}", g_reading / k_reading).into())
            } else {
                ui.set_gki_result("Please enter valid numbers.".into())
            }
        }
    });

    ui.run()
}
