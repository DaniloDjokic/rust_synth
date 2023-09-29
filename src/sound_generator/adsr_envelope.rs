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
            attack_time: 1.0,
            decay_time: 0.5,
            start_amplitude: 0.8,
            sustain_amplitude: 0.5,
            release_time: 0.5,
            trigger_on_time: 0.0,
            trigger_off_time: 0.0,
            is_note_on: false,
        }
    }

    pub fn get_amplitude(&self, time: f32) -> f32 {
        let mut amplitude = 0.0;

        let env_time = time - self.trigger_on_time;

        if let Some(state) = EnvelopeState::get_state(self, env_time) {
            amplitude = match state {
                EnvelopeState::Attack => self.get_attack_amplitude(env_time),
                EnvelopeState::Decay => self.get_decay_amplitude(env_time),
                EnvelopeState::Sustain => self.get_sustain_amplitude(),
                EnvelopeState::Release => self.get_release_amplitude(time), 
            }
        }
       
        if amplitude <= 0.0001 {
            amplitude = 0.0;
        }
        //println!("{amplitude}");

        amplitude
    }

    pub fn set_note_on(&mut self, time: f32) {
        self.is_note_on = true;

        self.trigger_on_time = time;
    }

    pub fn set_note_off(&mut self, time: f32) {
        self.is_note_on = false;

        self.trigger_off_time = time;
    }

    fn get_attack_amplitude(&self, env_time: f32) -> f32 {
        (env_time / self.attack_time) * self.start_amplitude
    }

    fn get_decay_amplitude(&self, env_time: f32) -> f32 {
        ((env_time - self.attack_time) / self.decay_time) * (self.sustain_amplitude - self.start_amplitude) + self.start_amplitude
    }

    fn get_sustain_amplitude(&self) -> f32 {
        self.sustain_amplitude
    }

    fn get_release_amplitude(&self, time: f32) -> f32 {
        ((time - self.trigger_off_time) / self.release_time) * (0.0 - self.sustain_amplitude) + self.sustain_amplitude
    }
}

