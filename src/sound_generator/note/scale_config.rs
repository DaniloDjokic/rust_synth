use rdev::Key;

use super::Note;

pub fn get_note_for_key(key: &Key) -> Option<i32> {
    match key {
        Key::KeyQ => Some(0), //C
        Key::Num2 => Some(1), //C#
        Key::KeyW => Some(2), //D
        Key::Num3 => Some(3), //D#
        Key::KeyE => Some(4), //E
        Key::KeyR => Some(5), //F
        Key::Num5 => Some(6), //F#
        Key::KeyT => Some(7), //G
        Key::Num6 => Some(8), //G#
        Key::KeyY => Some(9), //A
        Key::Num7 => Some(10), //A#
        Key::KeyU => Some(11), //B
        Key::KeyI => Some(12), //C
        Key::Num9 => Some(13), //C#
        Key::KeyO => Some(14), //D
        Key::Num0 => Some(15), //D#
        Key::KeyP => Some(16), //E

        Key::KeyZ => Some(24),
        Key::KeyS => Some(25),
        Key::KeyX => Some(26),
        Key::KeyD => Some(27),
        Key::KeyC => Some(28),
        Key::KeyV => Some(29),
        Key::KeyG => Some(30),
        Key::KeyB => Some(31),
        Key::KeyH => Some(32),
        Key::KeyN => Some(33),
        Key::KeyJ => Some(34),
        Key::KeyM => Some(35),
        Key::Comma => Some(36),
        Key::KeyL => Some(37),
        Key::Dot => Some(38),
        Key::SemiColon => Some(39),
        Key::Slash => Some(40),
        _ => None,
    }
}

pub fn get_note_frequency(note: &Note, octave : i32) -> f32 {
    let scale_id = note.scale_id + (octave * 12);
    let base: f32 = 1.0594630943592952645618252949463;
    
    256.0 * base.powi(scale_id)
}