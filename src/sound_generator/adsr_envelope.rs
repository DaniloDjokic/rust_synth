mod envelope_state;
use envelope_state::EnvelopeState;

pub struct ADSREnvelope {
    attack_time: f32,
    decay_time: f32,
    release_time: f32,

    sustain_amplitude: f32,
    start_amplitude: f32,

    trigger_on_time: f32,
    trigger_off_time: f32,

    is_note_on: bool,
}

impl ADSREnvelope {
    pub fn new() -> Self {
        Self {
            attack_time: 0.100,
            decay_time: 0.01,
            start_amplitude: 1.0,
            sustain_amplitude: 0.8,
            release_time: 0.200,
            trigger_on_time: 0.0,
            trigger_off_time: 0.0,
            is_note_on: false,
        }
    }

    pub fn get_amplitude(&self, time: f32) -> f32 {
        let mut amplitude = 0.0;

        let env_time = self.trigger_on_time - time;

        if let Some(state) = EnvelopeState::get_state(self, env_time) {
            amplitude = match state {
                EnvelopeState::Attack => self.get_attack_amplitude(env_time),
                EnvelopeState::Decay => self.get_decay_amplitude(env_time),
                EnvelopeState::Sustain => self.sustain_amplitude,
                EnvelopeState::Release => self.get_release_amplitude(time), 
            }
        }

        println!("{amplitude}");

        if amplitude <= 0.0001 {
            return 0.0;
        }

        amplitude
    }

    pub fn set_envelope(&mut self, note_on: bool, time: f32) {
        if self.is_note_on != note_on {
            if note_on { self.trigger_on_time = time } else { self.trigger_off_time = time } 
        }

        self.is_note_on = note_on;
    }

    fn get_attack_amplitude(&self, env_time: f32) -> f32 {
        (env_time / self.attack_time) * self.start_amplitude
    }

    fn get_decay_amplitude(&self, env_time: f32) -> f32 {
        ((env_time - self.attack_time) / self.decay_time) * (self.sustain_amplitude - self.start_amplitude) + self.start_amplitude
    }

    fn get_release_amplitude(&self, env_time: f32) -> f32 {
        (env_time / self.release_time) * (0.0 - self.sustain_amplitude) + self.sustain_amplitude
    }
}

