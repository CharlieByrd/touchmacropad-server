use enigo::{Coordinate::Abs, Enigo, Mouse, Settings};

#[tauri::command]
pub fn my_custom_command() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    enigo.move_mouse(500, 200, Abs).unwrap();
}
