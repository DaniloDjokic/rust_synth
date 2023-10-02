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
}

impl ADSREnvelope {
    pub fn new() -> Self {
        Self {
            attack_time: 1.0,
            decay_time: 0.5,
            start_amplitude: 0.8,
            sustain_amplitude: 0.2,
            release_time: 0.5,
            trigger_on_time: 0.0,
            trigger_off_time: 0.0,
        }
    }

    pub fn set_note_on(&mut self, time: f32) {
        self.trigger_on_time = time;
    }

    pub fn set_note_off(&mut self, time: f32) {
        self.trigger_off_time = time;
    }

    pub fn get_amplitude(&self, time: f32) -> f32 {
        let mut amplitude;
        let release_amplitude;

        let life_time;
        if self.trigger_on_time > self.trigger_off_time {
            life_time = time - self.trigger_on_time;
            amplitude = self.get_state_amplitude(life_time);
        }
        else {
            life_time = self.trigger_off_time - self.trigger_on_time;
            release_amplitude = self.get_state_amplitude(life_time);
            amplitude = self.get_release_amplitude(time, release_amplitude);
        }
       
        if amplitude <= 0.0001 {
            amplitude = 0.0;
        }

        amplitude
    }

    fn get_state_amplitude(&self, life_time: f32) -> f32 {
        if let Some(state) = EnvelopeState::get_state(self, life_time) {
            match state {
                EnvelopeState::Attack => self.get_attack_amplitude(life_time),
                EnvelopeState::Decay => self.get_decay_amplitude(life_time),
                EnvelopeState::Sustain => self.get_sustain_amplitude(),
            }
        } else {
            0.0
        }
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

    fn get_release_amplitude(&self, time: f32, release_amplitude: f32) -> f32 {
        ((time - self.trigger_off_time) / self.release_time) * (0.0 - release_amplitude) + release_amplitude
    }
}

