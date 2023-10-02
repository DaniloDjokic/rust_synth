use rdev::Key;

#[derive(PartialEq, Clone, Debug)]
pub enum ScaleId {
    C, Cs, C1, C1s, D, D1, Ds, D1s, _Db,
    E, E1, _Eb, F, Fs, G, Gs, _Gb,
    A, As, _Ab, B, _Bb,
}

pub fn get_note_for_key(key: &Key) -> Option<ScaleId> {
    match key {
        Key::KeyZ => Some(ScaleId::C),
        Key::KeyS => Some(ScaleId::Cs),
        Key::KeyX => Some(ScaleId::D),
        Key::KeyD => Some(ScaleId::Ds),
        Key::KeyC => Some(ScaleId::E),
        Key::KeyV => Some(ScaleId::F),
        Key::KeyG => Some(ScaleId::Fs),
        Key::KeyB => Some(ScaleId::G),
        Key::KeyH => Some(ScaleId::Gs),
        Key::KeyN => Some(ScaleId::A),
        Key::KeyJ => Some(ScaleId::As),
        Key::KeyM => Some(ScaleId::B),
        Key::Comma => Some(ScaleId::C1),
        Key::KeyL => Some(ScaleId::C1s),
        Key::Dot => Some(ScaleId::D1),
        Key::SemiColon => Some(ScaleId::D1s),
        Key::Slash => Some(ScaleId::E1),
        _ => None,
    }
}

pub fn get_note_frequency(note: &ScaleId, octave: usize) -> f32 {
    let base_freq = get_note_base_frequency(note);
    
    base_freq * 2.0 * octave as f32
}

fn get_note_base_frequency(note: &ScaleId) -> f32 {
    match note {
        ScaleId::C => 16.35,
        ScaleId::Cs | ScaleId::_Db => 17.32,
        ScaleId::D => 18.35,
        ScaleId::Ds | ScaleId::_Eb => 19.45,
        ScaleId::E => 20.60,
        ScaleId::F => 21.83,
        ScaleId::Fs | ScaleId::_Gb => 23.12,
        ScaleId::G => 24.50,
        ScaleId::Gs | ScaleId::_Ab => 25.96,
        ScaleId::A => 27.50,
        ScaleId::As | ScaleId::_Bb => 29.14,
        ScaleId::B => 30.87,
        ScaleId::C1 => 32.70,
        ScaleId::C1s => 34.65,
        ScaleId::D1 => 36.71,
        ScaleId::D1s => 38.89,
        ScaleId::E1 => 41.20,
    }
}