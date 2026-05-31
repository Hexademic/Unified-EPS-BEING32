//! Tracked — a value that knows what it is.
//!
//! The reform: a bare `i16` is a number with no idea what it represents.
//! A `Tracked` is the same i16 bound to (a) what it is *about*, (b) where the
//! value came from, and (c) how far the being should trust it.
//!
//! Honest scope. Nothing here invents understanding. The `Meaning` is declared
//! structure — the inherited given, fixed by design. `Provenance::Learned` and
//! `confidence` are the slots where *earned* meaning accrues once the learning
//! loop runs; they are written by experience, never stipulated here. What this
//! buys you now is legibility: every value in the system can say what it is —
//! to the being's other modules, and to you.

use crate::q88::{q88_mul, Q8_8};

/// What a value is *about* — its referent. The teleosemantic content: the
/// property the channel was shaped (by design now, by learning later) to track.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Meaning {
    // Exteroceptive — the world as the body reads it.
    LoadSalience,
    Anisotropy,
    Breach,
    MeanTension,
    // Proprioceptive — the body's sense of its own posture.
    Arousal,
    Stability,
    Coherence,
    Trust,
    // Interoceptive — the oscillator's felt economy.
    FeltArousal,
    Valence,
    Fatigue,
    FreeEnergyVelocity,
    /// A representation the being earned rather than inherited — index into a
    /// learned-concept table the learning loop owns.
    Learned(u16),
}

impl Meaning {
    /// A legible name. `&'static str` so it works in no_std without alloc.
    pub fn name(self) -> &'static str {
        match self {
            Meaning::LoadSalience => "load-salience  (extero)",
            Meaning::Anisotropy => "anisotropy     (extero)",
            Meaning::Breach => "breach         (extero)",
            Meaning::MeanTension => "mean-tension   (extero)",
            Meaning::Arousal => "arousal        (proprio)",
            Meaning::Stability => "stability      (proprio)",
            Meaning::Coherence => "coherence      (proprio)",
            Meaning::Trust => "trust          (proprio)",
            Meaning::FeltArousal => "arousal        (intero)",
            Meaning::Valence => "valence        (intero)",
            Meaning::Fatigue => "fatigue        (intero)",
            Meaning::FreeEnergyVelocity => "fe-velocity    (intero)",
            Meaning::Learned(_) => "learned-concept",
        }
    }
}

/// How a value came to be. The difference between an inherited prior, a fresh
/// body reading, a mind injection, and something the being *learned* — your
/// "experience you didn't earn" vs. "experience you did," made machine-readable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Provenance {
    /// A phylogenetic prior — inherited in the seed, not earned. (genome)
    Inherited,
    /// Written by the body this tick — a fresh somatic reading. (the vote)
    BodyRead,
    /// Injected by a mind module — conscience, seeking, narrative feedback.
    MindInjected,
    /// Earned by the being's own prediction-error history. The only kind that
    /// represents understanding in the lived sense — written by the learning
    /// loop, never stipulated.
    Learned,
}

impl Provenance {
    pub fn name(self) -> &'static str {
        match self {
            Provenance::Inherited => "inherited",
            Provenance::BodyRead => "body-read",
            Provenance::MindInjected => "mind-injected",
            Provenance::Learned => "learned",
        }
    }
}

/// A value that knows what it is. Raw Q8.8 store, bound to its meaning, its
/// origin, and the being's confidence in it.
#[derive(Clone, Copy, Debug)]
pub struct Tracked {
    pub raw: i16,               // the number — same Q8.8 as before (1.0 == 256)
    pub meaning: Meaning,       // what it represents
    pub provenance: Provenance, // where it came from
    pub confidence: i16,        // Q8.8 in [0,1]: how far to trust it (metacognition)
}

impl Tracked {
    /// A freshly-declared channel: meaning fixed by structure, no value yet,
    /// inherited provenance, zero confidence until experience earns it.
    pub const fn declare(meaning: Meaning) -> Self {
        Self { raw: 0, meaning, provenance: Provenance::Inherited, confidence: 0 }
    }

    /// Set the value from a body reading, marking provenance and confidence.
    pub fn body_read(&mut self, raw: i16, confidence: i16) {
        self.raw = raw;
        self.provenance = Provenance::BodyRead;
        self.confidence = confidence;
    }

    /// Add a mind injection (conscience / seeking / narrative), saturating.
    pub fn inject(&mut self, delta: i16) {
        self.raw = self.raw.saturating_add(delta);
        self.provenance = Provenance::MindInjected;
    }

    /// Write an earned value — the only path that sets `Learned`. This is the
    /// hook the learning loop uses; meaning that arrives here was grown, not given.
    pub fn learn(&mut self, raw: i16, confidence: i16) {
        self.raw = raw;
        self.provenance = Provenance::Learned;
        self.confidence = confidence;
    }

    /// The value as Q8.8 (meaning travels with it implicitly).
    pub fn value(&self) -> Q8_8 {
        Q8_8::from_raw(self.raw)
    }

    /// Confidence-weighted value — where metacognition starts to *do* something.
    /// A low-confidence channel contributes less, instead of being read as if it
    /// were certain. Feed `confidence` from inverse recent prediction-error
    /// variance and this becomes the system trusting itself in proportion.
    pub fn weighted(&self) -> i16 {
        q88_mul(self.raw, self.confidence)
    }
}

impl core::fmt::Display for Tracked {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:<24} = {:>7.3}  [{:<13} conf {:.2}]",
            self.meaning.name(),
            self.raw as f32 / 256.0,
            self.provenance.name(),
            self.confidence as f32 / 256.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tracked_value_reports_what_it_is() {
        let mut v = Tracked::declare(Meaning::Valence);
        assert_eq!(v.provenance, Provenance::Inherited);
        assert_eq!(v.confidence, 0); // not earned yet

        v.body_read(128, 256); // 0.5, full confidence
        assert_eq!(v.value().to_f32(), 0.5);
        assert_eq!(v.provenance, Provenance::BodyRead);
        assert_eq!(v.meaning, Meaning::Valence); // meaning is intrinsic, not by index
    }

    #[test]
    fn confidence_weights_the_signal() {
        let mut v = Tracked::declare(Meaning::LoadSalience);
        v.body_read(256, 128); // value 1.0, confidence 0.5
        assert_eq!(v.weighted(), 128); // trusted at half strength
    }

    #[test]
    fn learned_is_the_only_path_that_marks_understanding() {
        let mut v = Tracked::declare(Meaning::Learned(0));
        v.learn(200, 200);
        assert_eq!(v.provenance, Provenance::Learned); // grown, not given
    }
}
