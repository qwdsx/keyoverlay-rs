use rdev::{EventType, Key, Keyboard, KeyboardState};

pub trait KeyExt {
    fn to_string(&self) -> String;
}

// TODO: support mapping from the actual symbols to enum variants
impl KeyExt for Key {
    fn to_string(&self) -> String {
        let mut keyboard = Keyboard::new().unwrap();
        let key = keyboard.add(&EventType::KeyPress(*self));

        key.map(|s| s.to_uppercase()).unwrap_or("(?)".to_string())
    }
}