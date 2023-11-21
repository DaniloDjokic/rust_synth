use rdev::Key;

use super::Note;

pub fn get_scale_id_for_key(key: &Key) -> Option<(i32, usize)> {
    match key {
        Key::KeyQ => Some((0, 1)), //C
        Key::Num2 => Some((0, 1)), //C#
        Key::KeyW => Some((2, 2)), //D
        Key::Num3 => Some((3, 1)), //D#
        Key::KeyE => Some((4, 1)), //E
        Key::KeyR => Some((5, 1)), //F
        Key::Num5 => Some((6, 1)), //F#
        Key::KeyT => Some((7, 1)), //G
        Key::Num6 => Some((8, 1)), //G#
        Key::KeyY => Some((9, 1)), //A
        Key::Num7 => Some((10, 1)), //A#
        Key::KeyU => Some((11, 1)), //B
        Key::KeyI => Some((12, 1)), //C
        Key::Num9 => Some((13, 1)), //C#
        Key::KeyO => Some((14, 1)), //D
        Key::Num0 => Some((15, 1)), //D#
        Key::KeyP => Some((16, 1)), //E

        Key::KeyZ => Some((12, 3)),
        Key::KeyS => Some((13, 3)),
        Key::KeyX => Some((14, 3)),
        Key::KeyD => Some((15, 3)),
        Key::KeyC => Some((16, 3)),
        Key::KeyV => Some((17, 3)),
        Key::KeyG => Some((18, 3)),
        Key::KeyB => Some((19, 3)),
        Key::KeyH => Some((20, 3)),
        Key::KeyN => Some((21, 3)),
        Key::KeyJ => Some((22, 3)),
        Key::KeyM => Some((23, 3)),
        Key::Comma => Some((24, 3)),
        Key::KeyL => Some((25, 3)),
        Key::Dot => Some((26, 3)),
        Key::SemiColon => Some((27, 3)),
        Key::Slash => Some((28, 3)),
        _ => None,
    }
}

pub fn get_note_frequency(note: &Note, octave : i32) -> f32 {
    let scale_id = note.scale_id + (octave * 12);
    let base: f32 = 1.0594630943592952645618252949463;
    
    8.0 * base.powi(scale_id)
}