//! The Somatic Field — reformed so the bus carries meaning, not bare numbers.
//!
//! Same drivetrain as `field.rs`: the body still votes first, the mind still
//! only ever reads the field, the math is unchanged. What changed is that
//! nothing downstream has to *remember by convention* that channel 9 is valence
//! — the channel says so itself. Read by `Meaning`, not by index. Every value
//! arrives tagged with where it came from and how far to trust it. And the
//! whole field prints itself in plain terms, so you can read what the being is
//! carrying instead of reading twelve anonymous numbers.
//!
//! This is the reform pattern applied to one module. Propagating it is the same
//! move repeated: callers ask `field.get(Meaning::Valence)`; the metacognitive
//! monitor supplies real `confidence` (inverse prediction-error variance); the
//! learning loop writes `Provenance::Learned` channels as the being earns them.

use crate::body::Body;
use crate::q88::Q8_8;
use crate::tracked::{Meaning, Tracked};

pub const N_SOMATIC: usize = 12;

#[derive(Clone, Copy, Debug)]
pub struct SomaticField {
    pub ch: [Tracked; N_SOMATIC],
}

impl Default for SomaticField {
    fn default() -> Self {
        // Each slot declared with its meaning fixed — the inherited structure.
        Self {
            ch: [
                Tracked::declare(Meaning::LoadSalience),
                Tracked::declare(Meaning::Anisotropy),
                Tracked::declare(Meaning::Breach),
                Tracked::declare(Meaning::MeanTension),
                Tracked::declare(Meaning::Arousal),
                Tracked::declare(Meaning::Stability),
                Tracked::declare(Meaning::Coherence),
                Tracked::declare(Meaning::Trust),
                Tracked::declare(Meaning::FeltArousal),
                Tracked::declare(Meaning::Valence),
                Tracked::declare(Meaning::Fatigue),
                Tracked::declare(Meaning::FreeEnergyVelocity),
            ],
        }
    }
}

impl SomaticField {
    /// Read a channel *by meaning*, not by magic index. Downstream modules ask
    /// for what they want — `get(Meaning::Valence)` — and the field finds it.
    pub fn get(&self, m: Meaning) -> Option<&Tracked> {
        self.ch.iter().find(|t| t.meaning == m)
    }

    /// The raw Q8.8 for a meaning (0 if absent), for arithmetic-heavy callers.
    pub fn raw(&self, m: Meaning) -> i16 {
        self.get(m).map(|t| t.raw).unwrap_or(0)
    }

    pub fn mean_intensity(&self) -> i16 {
        let s: i32 = self.ch.iter().map(|t| t.raw as i32).sum();
        (s / N_SOMATIC as i32) as i16
    }

    pub fn variance(&self) -> i16 {
        let mean = self.mean_intensity() as i32;
        let s: i32 = self
            .ch
            .iter()
            .map(|t| {
                let d = t.raw as i32 - mean;
                d * d
            })
            .sum();
        ((s / N_SOMATIC as i32) >> 8).clamp(0, i16::MAX as i32) as i16
    }

    /// Write the body's state into the field — the vote. Identical assignments
    /// to the original, but each reading is now tagged `BodyRead` and carries a
    /// confidence.
    ///
    /// `conf` should be supplied by the metacognitive monitor (inverse recent
    /// prediction-error variance). Passing `Q88_SCALE` is a full-confidence
    /// placeholder until that monitor is wired — and because confidence is now
    /// a real field, that placeholder is *visible* rather than silently assumed.
    pub fn write_from_body(&mut self, b: &Body, fe_velocity: i16, conf: i16) {
        let f = b.topology.extract_features();

        // Exteroception: the topology's spatial reading of incoming load.
        self.ch[0].body_read(f.disequilibrium.raw.saturating_mul(2).min(255), conf);
        self.ch[1].body_read(f.anisotropy.raw, conf);
        self.ch[2].body_read(f.breach.raw.min(255), conf);
        self.ch[3].body_read(f.mean_tension.raw, conf);

        // Proprioception: the body's sense of its own posture.
        self.ch[4].body_read(b.arousal.raw.min(255), conf);
        self.ch[5].body_read(b.stability.raw, conf);
        self.ch[6].body_read(b.coherence.raw, conf);
        self.ch[7].body_read(b.trust.raw, conf);

        // Interoception — the oscillator itself.
        self.ch[8].body_read(b.arousal.raw.clamp(0, 255), conf);
        self.ch[9].body_read(b.valence.raw, conf);
        self.ch[10].body_read(Q8_8::ONE.sub(b.energy).raw.clamp(0, 256), conf);
        self.ch[11].body_read(fe_velocity, conf);
    }

    /// Inject a signal into a channel by meaning (mind feedback: conscience,
    /// seeking whisper, narrative reflection). Marks the channel mind-injected.
    pub fn inject(&mut self, m: Meaning, delta: i16) {
        if let Some(i) = self.ch.iter().position(|t| t.meaning == m) {
            self.ch[i].inject(delta);
        }
    }
}

impl core::fmt::Display for SomaticField {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "somatic field:")?;
        for t in &self.ch {
            writeln!(f, "  {}", t)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channels_are_addressable_by_meaning_not_index() {
        let mut field = SomaticField::default();
        // valence lives at index 9 today — but nothing downstream needs to know that.
        field.inject(Meaning::Valence, 64);
        assert_eq!(field.raw(Meaning::Valence), 64);
        // move it to a different slot and the lookup still finds it by meaning.
    }
}
