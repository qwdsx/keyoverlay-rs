use rdev::Key;

pub trait KeyExt {
    fn to_str(&self) -> &'static str;
}

impl KeyExt for Key {
    fn to_str(&self) -> &'static str {
        match self {
            Self::KeyZ => "Z",
            Self::KeyX => "X",
            _ => "(?)",
        }
    }
}
