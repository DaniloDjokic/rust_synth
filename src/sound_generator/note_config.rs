use rdev::Key;

pub enum Note {
    C, Cs, C1, D, Ds, _Db,
    E, _Eb, F, Fs, G, Gs, _Gb,
    A, As, _Ab, B, _Bb,
}

pub fn get_frequency(key: Key) -> Option<f32> {
    let key = match key {
        Key::KeyZ => Some(Note::C),
        Key::KeyS => Some(Note::Cs),
        Key::KeyX => Some(Note::D),
        Key::KeyD => Some(Note::Ds),
        Key::KeyC => Some(Note::E),
        Key::KeyV => Some(Note::F),
        Key::KeyG => Some(Note::Fs),
        Key::KeyB => Some(Note::G),
        Key::KeyH => Some(Note::Gs),
        Key::KeyN => Some(Note::A),
        Key::KeyJ => Some(Note::As),
        Key::KeyM => Some(Note::B),
        Key::Comma => Some(Note::C1),
        _ => None,
    };

    match key {
        Some(note) => Some(get_note_frequency(note, 4)),
        None => None
    }
}

fn get_note_frequency(note: Note, octave: usize) -> f32 {
    let base_freq = get_note_base_frequency(note);

    base_freq * 2.0 * octave as f32
}

fn get_note_base_frequency(note: Note) -> f32 {
    match note {
        Note::C => 16.35,
        Note::Cs | Note::_Db => 17.32,
        Note::D => 18.35,
        Note::Ds | Note::_Eb => 19.45,
        Note::E => 20.60,
        Note::F => 21.83,
        Note::Fs | Note::_Gb => 23.12,
        Note::G => 24.50,
        Note::Gs | Note::_Ab => 25.96,
        Note::A => 27.50,
        Note::As | Note::_Bb => 29.14,
        Note::B => 30.87,
        Note::C1 => 32.70,
    }
}