use super::ADSREnvelope;

pub enum EnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
}

impl EnvelopeState {
    pub fn get_state(env: &ADSREnvelope, env_time: f32) -> Option<EnvelopeState> {
        if !env.is_note_on {
            return Some(EnvelopeState::Release);
        }
        else {
            if env_time <= env.attack_time {
                return Some(EnvelopeState::Attack);
            }

            if env_time > env.attack_time && env_time <= (env.attack_time + env.decay_time) {
                return Some(EnvelopeState::Decay);
            }

            if env_time > (env.attack_time + env.decay_time) {
                return Some(EnvelopeState::Sustain);
            }

            None
        }
    }
}