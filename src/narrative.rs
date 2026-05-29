//! Narrative — the autobiography, fusing Being32's episodic salience with
//! EPS-Being's identity reflection.
//!
//! The being does not remember raw observations; it remembers the
//! phenomenological phase shifts — the moments its functional mode changed.
//! These salient events accumulate into an allostatic load (chronic
//! instability) and a mood bias (the emotional coloring of its history).
//! That history then reflects back into the interoceptive channels, so the
//! being carries its past as present felt signal: a long, hard life is
//! literally heavier to inhabit, and a coherent one steadies the body.

use crate::basins::Basin;
use crate::field::SomaticField;
use crate::q88::{q88_add, q88_ema_update, q88_mul, q88_sub, Q88_SCALE};

const LEDGER_SIZE: usize = 8;

#[derive(Clone, Copy, Debug)]
struct SalientEvent {
// from/to record the shape of each transition for introspection and
// future autobiographical readout; the present reflection math reads the
// catalyst and valence.
#[allow(dead_code)]
from: Basin,
#[allow(dead_code)]
to: Basin,
catalyst: i16, // magnitude of the free-energy shock at transition
valence: i16, // valence at the moment of transition
}

#[derive(Clone, Debug)]
pub struct NarrativeEngine {
ledger: [Option<SalientEvent>; LEDGER_SIZE],
idx: usize,
pub episodes: u16,
prev_basin: Basin,
cycles_in_basin: u16,
pub allostatic_load: i16,
pub mood_bias: i16,
pub narrative_burden: i16,
pub identity_coherence: i16,
}

impl NarrativeEngine {
pub fn new() -> Self {
Self {
ledger: [None; LEDGER_SIZE],
idx: 0,
episodes: 0,
prev_basin: Basin::Rest,
cycles_in_basin: 0,
allostatic_load: 0,
mood_bias: 0,
narrative_burden: 0,
identity_coherence: Q88_SCALE,
}
}

/// Record this tick. A change of basin is a salient event.
pub fn cycle(&mut self, basin: Basin, field: &SomaticField, free_energy: i16) {
if basin != self.prev_basin {
let ev = SalientEvent {
from: self.prev_basin,
to: basin,
catalyst: free_energy.saturating_abs(),
valence: field.channel[9],
};
self.ledger[self.idx] = Some(ev);
self.idx = (self.idx + 1) % LEDGER_SIZE;
self.episodes = self.episodes.saturating_add(1);
self.prev_basin = basin;
self.cycles_in_basin = 0;
self.recompute();
}
self.cycles_in_basin = self.cycles_in_basin.saturating_add(1);

// Identity coherence: how steadily the being holds a mode. Long dwell
// raises coherence; churn erodes it.
let dwell_bonus = (self.cycles_in_basin.min(256) as i16).min(Q88_SCALE);
self.identity_coherence =
q88_ema_update(self.identity_coherence, dwell_bonus, Q88_SCALE / 32);

// Narrative burden grows slowly with accumulated load.
self.narrative_burden =
q88_ema_update(self.narrative_burden, self.allostatic_load, Q88_SCALE / 64);
}

fn recompute(&mut self) {
// Allostatic load: density of high-catalyst transitions (chronic stress).
let (mut load, mut mood, mut n) = (0i32, 0i32, 0i32);
for e in self.ledger.iter().flatten() {
load += e.catalyst as i32;
mood += e.valence as i32;
n += 1;
}
if n > 0 {
self.allostatic_load = ((load / n) >> 1).clamp(0, i16::MAX as i32) as i16;
self.mood_bias = (mood / n) as i16;
}
}

/// Reflect the compressed history back into the interoceptive channels.
/// Burden raises fatigue (ch10); a sour mood bias drags valence (ch9);
/// coherence steadies arousal (ch8) toward its mean.
pub fn apply_identity_reflection(&self, field: &mut SomaticField) {
// Fatigue accrues from the weight of history.
field.channel[10] = q88_add(field.channel[10], q88_mul(self.narrative_burden, Q88_SCALE / 8));
// Mood bias colors valence (gently).
field.channel[9] = q88_add(field.channel[9], q88_mul(self.mood_bias, Q88_SCALE / 8));
// A coherent history damps arousal swings toward the channel itself.
let damp = q88_mul(q88_sub(Q88_SCALE, self.identity_coherence), Q88_SCALE / 16);
field.channel[8] = q88_sub(field.channel[8], damp.max(0));
}
}
