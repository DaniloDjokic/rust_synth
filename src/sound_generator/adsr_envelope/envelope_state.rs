use super::ADSREnvelope;

pub enum EnvelopeState {
    Attack,
    Decay,
    Sustain,
    Release,
}

impl EnvelopeState {
    pub fn get_state(env: &ADSREnvelope, life_time: f32) -> Option<EnvelopeState> {
        if !env.is_note_on {
            return Some(EnvelopeState::Release);
        }
        else {
            if life_time <= env.attack_time {
                return Some(EnvelopeState::Attack);
            }

            if life_time > env.attack_time && life_time <= (env.attack_time + env.decay_time) {
                return Some(EnvelopeState::Decay);
            }

            if life_time > (env.attack_time + env.decay_time) {
                return Some(EnvelopeState::Sustain);
            }

            None
        }
    }
}