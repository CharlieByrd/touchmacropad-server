use enigo::{
    Direction::{Press, Release},
    Enigo, Key, Keyboard, Settings,
};

fn str_to_key(key: &str) -> Key {
    match key.to_lowercase().as_str() {
        // Специальные клавиши
        "ctrl" | "control" => Key::Control,
        "shift" => Key::Shift,
        "alt" => Key::Alt,
        "enter" => Key::Return,
        "esc" | "escape" => Key::Escape,
        "tab" => Key::Tab,
        "backspace" => Key::Backspace,
        "space" => Key::Space,
        "window" => Key::Meta,
        // F1-F24
        "f1" => Key::F1,
        "f2" => Key::F2,
        "f3" => Key::F3,
        "f4" => Key::F4,
        "f5" => Key::F5,
        "f6" => Key::F6,
        "f7" => Key::F7,
        "f8" => Key::F8,
        "f9" => Key::F9,
        "f10" => Key::F10,
        "f11" => Key::F11,
        "f12" => Key::F12,
        "f13" => Key::F13,
        "f14" => Key::F14,
        "f15" => Key::F15,
        "f16" => Key::F16,
        "f17" => Key::F17,
        "f18" => Key::F18,
        "f19" => Key::F19,
        "f20" => Key::F20,
        "f21" => Key::F21,
        "f22" => Key::F22,
        "f23" => Key::F23,
        "f24" => Key::F24,
        // Обработка обычных символов
        _ if key.len() == 1 => Key::Unicode(key.chars().next().unwrap()),
        _ => panic!("Unknown key: {}", key),
    }
}

#[tauri::command]
pub fn macros_command(keys: Vec<String>) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();


    for key in &keys {
        enigo.key(str_to_key(key), Press).unwrap();
        print!("{} ", key);
        print!("{:?} ", str_to_key(key));
    }

    for key in &keys {
        enigo.key(str_to_key(key), Release).unwrap();
    }
}
