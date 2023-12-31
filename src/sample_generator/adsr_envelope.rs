mod envelope_state;
use envelope_state::EnvelopeState;

pub struct ADSREnvelope {
    attack_time: f32,
    decay_time: f32,
    release_time: f32,

    sustain_amplitude: f32,
    attack_amplitude: f32,
}

impl ADSREnvelope {
    pub fn new(
        attack_time: f32, 
        decay_time: f32, 
        release_time: f32, 
        attack_amplitude: f32,
        sustain_amplitude: f32
    ) -> Self {
        Self {
            attack_time,
            decay_time,
            attack_amplitude,
            sustain_amplitude,
            release_time,
        }
    }

    pub fn get_amplitude(&self, time: f32, time_on: f32, time_off: f32) -> f32 {
        let mut amplitude = 0.0;
        let mut release_amplitude = 0.0;
        let life_time;

        if time_on > time_off {
            life_time = time - time_on;
            if let Some(ampl) = self.get_state_amplitude(life_time) {
                amplitude = ampl;
            }
        }
        else {
            life_time = time_off - time_on;
            if let Some(r_ampl) = self.get_state_amplitude(life_time) {
                release_amplitude = r_ampl;
            }
            amplitude = self.get_release_amplitude(time, time_off, release_amplitude);
        }

        if amplitude <= 0.0001 {
            amplitude = 0.0;
        }
       
        amplitude 
    }

    fn get_state_amplitude(&self, life_time: f32) -> Option<f32> {
        if let Some(state) = EnvelopeState::get_state(self, life_time) {
            match state {
                EnvelopeState::Attack => Some(self.get_attack_amplitude(life_time)),
                EnvelopeState::Decay => Some(self.get_decay_amplitude(life_time)),
                EnvelopeState::Sustain => Some(self.get_sustain_amplitude()),
            }
        } else {
            None
        }
    }

    fn get_attack_amplitude(&self, env_time: f32) -> f32 {
        (env_time / self.attack_time) * self.attack_amplitude
    }

    fn get_decay_amplitude(&self, env_time: f32) -> f32 {
        ((env_time - self.attack_time) / self.decay_time) * (self.sustain_amplitude - self.attack_amplitude) + self.attack_amplitude
    }

    fn get_sustain_amplitude(&self) -> f32 {
        self.sustain_amplitude
    }

    fn get_release_amplitude(&self, time: f32, time_off: f32, release_amplitude: f32) -> f32 {
        ((time - time_off) / self.release_time) * (0.0 - release_amplitude) + release_amplitude
    }
}

