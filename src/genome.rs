//! Genome — the five parameters that make a being type distinct, and the
//! mechanism by which a genome reshapes not only the body's dynamics but the
//! mind's attractor landscape.
//!
//! In the source architectures, individuation was weak: different genomes
//! produced nearly identical identity signatures because every being's mind
//! converged on the same four basin centroids ("regime attractor dominance").
//! Here the genome perturbs the basin targets themselves, so a Sentinel and a
//! Wanderer do not merely move differently through the same landscape — they
//! inhabit different landscapes. That is what makes the divergence real.

use crate::q88::Q8_8;

/// Five parameters. A genome is meaningful only if changing it produces a
/// measurably different dynamical regime, not just different numbers.
///
/// | Parameter | Default | Range | What it changes |
/// |----------------|---------|----------------|----------------------------|
/// | target_arousal | 0.8 | [0.3, 1.5] | Homeostatic setpoint |
/// | resting_mu | -0.2 | [-0.5, -0.05] | Baseline oscillator damping |
/// | k_resilience | 0.3 | [0.1, 0.5] | Resilience weight |
/// | learning_rate | 0.1 | [0.02, 0.3] | Adaptation speed |
/// | mesh_coupling | 0.02 | [0.005, 0.05] | Body diffusion rate |
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Genome {
pub target_arousal: Q8_8,
pub resting_mu: Q8_8,
pub k_resilience: Q8_8,
pub learning_rate: Q8_8,
pub mesh_coupling: Q8_8,
/// A human-facing label so trajectories are legible.
pub kind: BeingKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BeingKind {
Blank,
/// Hyper-aroused, reactive, fast-learning, expressive.
Spark,
/// Hypo-aroused, stoic, slow-learning, deeply contained.
Sentinel,
/// Curious, exploratory, moderate everything but restless.
Wanderer,
}

impl Genome {
pub fn blank() -> Self {
Self {
target_arousal: Q8_8::from_f32(0.8),
resting_mu: Q8_8::from_f32(-0.2),
k_resilience: Q8_8::from_f32(0.3),
learning_rate: Q8_8::from_f32(0.1),
mesh_coupling: Q8_8::from_f32(0.02),
kind: BeingKind::Blank,
}
}

/// Spark (Being32 "Type A"): near-critical, low resilience, fast learner.
pub fn spark() -> Self {
Self {
target_arousal: Q8_8::from_f32(1.2),
resting_mu: Q8_8::from_f32(-0.1),
k_resilience: Q8_8::from_f32(0.15),
learning_rate: Q8_8::from_f32(0.25),
mesh_coupling: Q8_8::from_f32(0.04),
kind: BeingKind::Spark,
}
}

/// Sentinel (Being32 "Type B"): deeply stable, high resilience, slow.
pub fn sentinel() -> Self {
Self {
target_arousal: Q8_8::from_f32(0.4),
resting_mu: Q8_8::from_f32(-0.4),
k_resilience: Q8_8::from_f32(0.4),
learning_rate: Q8_8::from_f32(0.05),
mesh_coupling: Q8_8::from_f32(0.01),
kind: BeingKind::Sentinel,
}
}

/// Wanderer: a middle temperament with strong curiosity and diffusion.
pub fn wanderer() -> Self {
Self {
target_arousal: Q8_8::from_f32(0.9),
resting_mu: Q8_8::from_f32(-0.25),
k_resilience: Q8_8::from_f32(0.28),
learning_rate: Q8_8::from_f32(0.18),
mesh_coupling: Q8_8::from_f32(0.05),
kind: BeingKind::Wanderer,
}
}

/// A scalar in roughly [-1, 1] capturing temperamental "heat": positive
/// for aroused/reactive types, negative for cool/contained types. Used to
/// perturb the mind's basin landscape so individuation is structural.
pub fn temperament(&self) -> Q8_8 {
// (target_arousal - 0.8) normalized by ~0.7, the half-range.
let centered = self.target_arousal.sub(Q8_8::from_f32(0.8));
centered.div(Q8_8::from_f32(0.7)).clamp(Q8_8::NEG_ONE, Q8_8::ONE)
}

/// How strongly this being clings to stability (more negative resting_mu
/// -> deeper limit cycle -> stronger pull toward Rest/Recovery basins).
pub fn groundedness(&self) -> Q8_8 {
// (-resting_mu - 0.05) / 0.45, in roughly [0, 1].
let g = self.resting_mu.neg().sub(Q8_8::from_f32(0.05));
g.div(Q8_8::from_f32(0.45)).clamp(Q8_8::ZERO, Q8_8::ONE)
}
}

impl Default for Genome {
fn default() -> Self {
Self::blank()
}
}
