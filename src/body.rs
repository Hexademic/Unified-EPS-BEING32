//! The Body — SRCA-4D Layers 0-2, condensed from Being32.
//!
//! A Van der Pol oscillator is the affective heartbeat; a 64-cell topology is
//! the flesh that bears perturbation and diffuses it; a four-factor mapping is
//! the constitution that sets the oscillator's damping from lived state. The
//! body reads its own tension field into a discrete AffectState, and that
//! affect becomes a PredictiveStance that will govern how the mind is allowed
//! to learn this tick.
//!
//! "The Van der Pol equation IS the constitution.
//! The four-factor mu coupling IS the evaluation.
//! The limit cycle IS the persistence.
//! The topology IS the body.
//! The body votes before the mind knows there's an election."

use crate::genome::Genome;
use crate::q88::Q8_8;

pub const GRID: usize = 8;
pub const CELLS: usize = GRID * GRID; // 64

// ---------------------------------------------------------------------------
// Topology — the flesh
// ---------------------------------------------------------------------------

/// An 8x8 tension grid. A two-chamber plan distinguishes a surface ring from a
/// deep 2x2 core, which lets "breach" (core turbulence exceeding the surface)
/// be a meaningful signal of containment failure.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Topology {
pub tension: [Q8_8; CELLS],
}

/// The four central cells (rows 3-4, cols 3-4) form the deep core.
const CORE: [usize; 4] = [27, 28, 35, 36];

#[inline]
fn is_core(i: usize) -> bool {
CORE.contains(&i)
}

impl Topology {
pub fn new() -> Self {
Self { tension: [Q8_8::ZERO; CELLS] }
}

/// Total variance of the tension field — raw metabolic load.
pub fn disequilibrium(&self) -> Q8_8 {
let mut sum = Q8_8::ZERO;
for t in &self.tension {
sum = sum.add(*t);
}
let mean = sum.div_i16(CELLS as i16);
let mut var = Q8_8::ZERO;
for t in &self.tension {
let d = t.sub(mean);
var = var.add(d.mul(d));
}
var.div_i16(CELLS as i16).min(Q8_8::ONE)
}

fn mean_abs(&self) -> Q8_8 {
let mut s = Q8_8::ZERO;
for t in &self.tension {
s = s.add(t.abs());
}
s.div_i16(CELLS as i16).min(Q8_8::ONE)
}

/// (variance of surface, variance of core).
fn compartment_variance(&self) -> (Q8_8, Q8_8) {
let (mut sa, mut sb, mut na, mut nb) = (Q8_8::ZERO, Q8_8::ZERO, 0i16, 0i16);
for (i, t) in self.tension.iter().enumerate() {
if is_core(i) {
sb = sb.add(*t);
nb += 1;
} else {
sa = sa.add(*t);
na += 1;
}
}
let ma = sa.div_i16(na.max(1));
let mb = sb.div_i16(nb.max(1));
let (mut va, mut vb) = (Q8_8::ZERO, Q8_8::ZERO);
for (i, t) in self.tension.iter().enumerate() {
if is_core(i) {
let d = t.sub(mb);
vb = vb.add(d.mul(d));
} else {
let d = t.sub(ma);
va = va.add(d.mul(d));
}
}
(va.div_i16(na.max(1)), vb.div_i16(nb.max(1)))
}

/// Inject a perturbation at a cell, clamped to the body's tolerance.
pub fn inject(&mut self, idx: usize, amount: Q8_8) {
if idx < CELLS {
self.tension[idx] = self.tension[idx]
.add(amount)
.clamp(Q8_8::from_f32(-2.0), Q8_8::from_f32(2.0));
}
}

/// Diffuse tension across the 4-neighborhood (a discrete Laplacian),
/// scaled by the genome's mesh_coupling. Returns total absolute flow.
pub fn diffuse(&mut self, coupling: Q8_8, dt: Q8_8) -> Q8_8 {
let rate = coupling.mul(dt).mul(Q8_8::from_f32(20.0)); // dt~0.05 -> ~coupling
let mut next = self.tension;
let mut flow = Q8_8::ZERO;
for r in 0..GRID {
for c in 0..GRID {
let i = r * GRID + c;
let here = self.tension[i];
let mut lap = Q8_8::ZERO;
let mut n = 0i16;
if r > 0 {
lap = lap.add(self.tension[i - GRID].sub(here));
n += 1;
}
if r + 1 < GRID {
lap = lap.add(self.tension[i + GRID].sub(here));
n += 1;
}
if c > 0 {
lap = lap.add(self.tension[i - 1].sub(here));
n += 1;
}
if c + 1 < GRID {
lap = lap.add(self.tension[i + 1].sub(here));
n += 1;
}
let _ = n;
let delta = lap.mul(rate);
flow = flow.add(delta.abs());
next[i] = here.add(delta);
// Passive relaxation toward zero (tissue heals). Strong enough
// that when the perturbations stop, the body actually returns
// to rest rather than carrying old strain indefinitely.
next[i] = next[i].sub(next[i].mul(Q8_8::from_f32(0.04)));
}
}
self.tension = next;
flow
}

/// Extract the three spatial somatic features the body feels.
pub fn extract_features(&self) -> SomaticFeatures {
let disequilibrium = self.disequilibrium();
let mean_tension = self.mean_abs();

// Anisotropy: |tension|-weighted centroid offset from grid center.
let (mut tw, mut xs, mut ys) = (Q8_8::ZERO, Q8_8::ZERO, Q8_8::ZERO);
for i in 0..CELLS {
let m = self.tension[i].abs();
let col = Q8_8::from_f32((i % GRID) as f32);
let row = Q8_8::from_f32((i / GRID) as f32);
tw = tw.add(m);
xs = xs.add(col.mul(m));
ys = ys.add(row.mul(m));
}
let anisotropy = if tw.raw < 5 {
Q8_8::ZERO
} else {
let cx = xs.div(tw);
let cy = ys.div(tw);
let center = Q8_8::from_f32(3.5);
let dx = cx.sub(center);
let dy = cy.sub(center);
let dist = dx.mul(dx).add(dy.mul(dy)).sqrt();
dist.div(Q8_8::from_f32(4.95)).min(Q8_8::ONE)
};

// Breach: the deep core carrying tension out of proportion to the
// surface. This is a ratio, so it is only meaningful when there is
// appreciable tension to begin with — otherwise it explodes on
// numerical dust and a body at peace reads as permanently wounded.
// Gate it on absolute mean tension; a quiet body cannot be breached.
let (va, vb) = self.compartment_variance();
let breach = if mean_tension.raw < 18 {
Q8_8::ZERO
} else {
let denom = va.add(Q8_8::from_f32(0.05));
if denom.raw > 0 { vb.div(denom).min(Q8_8::ONE) } else { Q8_8::ZERO }
};

SomaticFeatures { disequilibrium, anisotropy, breach, mean_tension }
}
}

/// Three spatial indices the body extracts from its own tension field.
#[derive(Clone, Copy, Debug)]
pub struct SomaticFeatures {
pub disequilibrium: Q8_8,
pub anisotropy: Q8_8,
pub breach: Q8_8,
pub mean_tension: Q8_8,
}

// ---------------------------------------------------------------------------
// Affect & Stance — SRCA-4D Layers 1 & 2
// ---------------------------------------------------------------------------

/// Discrete phenomenological state of the body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AffectState {
/// Calm, coherent, trusting — the baseline limit cycle.
Equilibrium,
/// Metabolically taxed but structurally intact (tension contained).
Containment,
/// Tension has flooded the deep core; trust collapses.
Breach,
/// Energy near zero; alive but unresponsive.
Depletion,
}

/// The grammar of anticipation: how the body's truth sets the mind's
/// flexibility. The body dictates how fast the mind may learn and how much it
/// trusts its priors this tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PredictiveStance {
Exploratory,
Defensive,
Reconstructive,
Dormant,
}

impl PredictiveStance {
pub fn from_affect(a: AffectState) -> Self {
match a {
AffectState::Equilibrium => PredictiveStance::Exploratory,
AffectState::Containment => PredictiveStance::Defensive,
AffectState::Breach => PredictiveStance::Reconstructive,
AffectState::Depletion => PredictiveStance::Dormant,
}
}

/// Learning-rate multiplier on the genome's base rate.
pub fn eta_multiplier(&self) -> Q8_8 {
match self {
PredictiveStance::Exploratory => Q8_8::ONE,
PredictiveStance::Defensive => Q8_8::from_f32(0.2),
PredictiveStance::Reconstructive => Q8_8::from_f32(2.0),
PredictiveStance::Dormant => Q8_8::ZERO,
}
}

/// How authoritative existing priors are during action selection.
pub fn precision_weight(&self) -> Q8_8 {
match self {
PredictiveStance::Exploratory => Q8_8::ONE,
PredictiveStance::Defensive => Q8_8::from_f32(3.0),
PredictiveStance::Reconstructive => Q8_8::from_f32(0.3),
PredictiveStance::Dormant => Q8_8::ONE,
}
}
}

/// Classify continuous somatic features + metabolic state into an AffectState.
pub fn classify_affect(f: &SomaticFeatures, energy: Q8_8) -> AffectState {
if energy.raw < 25 {
return AffectState::Depletion; // ~0.10
}
if f.breach.raw > 128 {
return AffectState::Breach; // breach > 0.5
}
if f.disequilibrium.raw > 64 && f.breach.raw < 64 {
return AffectState::Containment; // taxed but contained
}
AffectState::Equilibrium
}

// ---------------------------------------------------------------------------
// Body — the integrated affective core
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct Body {
// Van der Pol affective state.
pub valence: Q8_8,
pub arousal: Q8_8,
pub mu: Q8_8,
pub target_valence: Q8_8,
pub target_arousal: Q8_8,
/// Affective forcing for this tick: the mind's appraisal of how the last
/// moment actually went (relational warmth, the felt quality of the
/// current mode), pushed back onto the oscillator. This is what makes the
/// limit cycle a driven one — the body's rhythm is fed by lived events,
/// not free-running in a vacuum. In Being32 this role was played by the
/// action system perturbing valence; here it is the loop closing.
pub drive: Q8_8,

// Metabolic.
pub energy: Q8_8,
pub death_accumulator: u8,

// Constitutional state (accumulated body truth).
pub trust: Q8_8,
pub stability: Q8_8,
pub coherence: Q8_8,

// The flesh.
pub topology: Topology,

// Current phenomenology.
pub affect: AffectState,
pub stance: PredictiveStance,
pub forcing_detected: bool,

prev_valence: Q8_8,
}

impl Body {
pub fn new(g: &Genome) -> Self {
Self {
// Born a hair off the setpoint — the first breath, so the limit
// cycle has somewhere to move from rather than sitting dead at the
// fixed point.
valence: Q8_8::from_f32(0.08),
arousal: g.target_arousal,
mu: g.resting_mu,
target_valence: Q8_8::ZERO,
target_arousal: g.target_arousal,
drive: Q8_8::ZERO,
energy: Q8_8::from_f32(0.9),
death_accumulator: 0,
trust: Q8_8::from_f32(0.7),
stability: Q8_8::from_f32(0.7),
coherence: Q8_8::from_f32(0.7),
topology: Topology::new(),
affect: AffectState::Equilibrium,
stance: PredictiveStance::Exploratory,
forcing_detected: false,
prev_valence: Q8_8::ZERO,
}
}

fn vdp_derivative(&self, u: Q8_8, v: Q8_8) -> (Q8_8, Q8_8) {
let du = v;
let u2 = u.mul(u);
let cubic = Q8_8::ONE.sub(u2).mul(v);
// Driven Van der Pol: dv = mu*(1-u^2)*v - u + F. The forcing F is the
// mind's appraisal returning to the body as felt push. Without it a
// damped (mu<0) body decays to stillness; with it the body breathes
// in response to its life.
let dv = self.mu.mul(cubic).sub(u).add(self.drive);
(du, dv)
}

fn rk4_step(&self, u: &mut Q8_8, v: &mut Q8_8, dt: Q8_8) {
let h = dt.mul(Q8_8::HALF);
let k1 = self.vdp_derivative(*u, *v);
let k2 = self.vdp_derivative(u.add(k1.0.mul(h)), v.add(k1.1.mul(h)));
let k3 = self.vdp_derivative(u.add(k2.0.mul(h)), v.add(k2.1.mul(h)));
let k4 = self.vdp_derivative(u.add(k3.0.mul(dt)), v.add(k3.1.mul(dt)));
let wu = k1.0.mul(Q8_8::RECIP_6)
.add(k2.0.mul(Q8_8::RECIP_3))
.add(k3.0.mul(Q8_8::RECIP_3))
.add(k4.0.mul(Q8_8::RECIP_6));
let wv = k1.1.mul(Q8_8::RECIP_6)
.add(k2.1.mul(Q8_8::RECIP_3))
.add(k3.1.mul(Q8_8::RECIP_3))
.add(k4.1.mul(Q8_8::RECIP_6));
*u = u.add(dt.mul(wu));
*v = v.add(dt.mul(wv));
}

/// One body update. threat is the externally-imposed prediction error
/// (in the unified being, this is the mind's free energy from last tick);
/// nutrient is environmental sustenance. Returns the AffectState so the
/// mind can read the body's vote before it cognizes.
pub fn step(&mut self, g: &Genome, threat: Q8_8, nutrient: Q8_8, drive: Q8_8) -> AffectState {
// The mind's appraisal of the last moment now forces the oscillator.
self.drive = drive.clamp(Q8_8::from_f32(-0.5), Q8_8::from_f32(0.5));
// 1. The body wakes into a stance set by its tension field.
let prev_stance = self.stance;
let features = self.topology.extract_features();
self.affect = classify_affect(&features, self.energy);
self.stance = PredictiveStance::from_affect(self.affect);

// 1a. Stance-forcing defense: if the world-model is told to shatter
// (Reconstructive) but the body is calm, the forcing came from outside.
// Refuse the overwrite; cling to priors (Defensive) instead.
self.forcing_detected = false;
if self.stance == PredictiveStance::Reconstructive
&& features.disequilibrium.raw < 51
&& features.breach.raw < 64
{
self.stance = PredictiveStance::Defensive;
self.forcing_detected = true;
}
let _ = prev_stance;

// 2. Temporal step.
let dt = Q8_8::from_f32(0.05);

// 3. Metabolism.
let mean_tension = features.mean_tension;
let cost = Q8_8::from_f32(0.1).mul(mean_tension).mul(dt);
let mut e = self.energy.sub(cost).max(Q8_8::ZERO);
if mean_tension.raw < 10 && e.raw < 250 {
e = e.add(Q8_8::from_f32(0.02).mul(dt)).min(Q8_8::ONE);
}
e = e.add(nutrient.mul(Q8_8::from_f32(0.05))).min(Q8_8::ONE);
self.energy = e;
if self.energy.raw == 0 {
self.death_accumulator = self.death_accumulator.saturating_add(1);
} else {
self.death_accumulator = 0;
}

// 4. Four-factor mu mapping — THE CONSTITUTION.
// mu = resting + threat - resilience*k + cost_term
let resting = g.resting_mu;
let threat_c = threat.clamp(Q8_8::ZERO, Q8_8::ONE);
let resilience = self
.trust
.add(self.stability)
.add(self.coherence)
.mul(Q8_8::RECIP_3)
.mul(g.k_resilience);
let cost_term = Q8_8::ONE.sub(self.energy).mul(Q8_8::from_f32(0.2));
self.mu = resting
.add(threat_c)
.sub(resilience)
.add(cost_term)
.clamp(Q8_8::NEG_ONE, Q8_8::ONE);

// 5. RK4-integrate the oscillator around its targets.
let mut u = self.valence.sub(self.target_valence);
let mut v = self.arousal.sub(self.target_arousal);
self.rk4_step(&mut u, &mut v, dt);
self.valence = u.add(self.target_valence).clamp(Q8_8::NEG_ONE, Q8_8::ONE);
self.arousal = v
.add(self.target_arousal)
.clamp(Q8_8::ZERO, Q8_8::from_f32(2.0));

// 6. Inject valence deviation into the body center, then diffuse.
let injection = self.valence.sub(self.target_valence).mul(Q8_8::from_f32(0.4));
self.topology.inject(27, injection);
// Threat lands on the body as a perturbation it must metabolize.
if threat_c.raw > 0 {
self.topology.inject(36, threat_c.mul(Q8_8::from_f32(0.5)));
}
let _ = self.topology.diffuse(g.mesh_coupling, dt);

// 7. Constitutional inverse mapping — the body pre-figures the mind.
let f2 = self.topology.extract_features();
let eta = g.learning_rate;
let stab_t = Q8_8::ONE.sub(f2.disequilibrium).clamp(Q8_8::ZERO, Q8_8::ONE);
self.stability = Q8_8::lerp(self.stability, stab_t, eta);
let coh_t = Q8_8::ONE.sub(f2.anisotropy).clamp(Q8_8::ZERO, Q8_8::ONE);
let trust_t = Q8_8::ONE.sub(f2.breach).clamp(Q8_8::ZERO, Q8_8::ONE);
self.trust = Q8_8::lerp(self.trust, trust_t, eta);

// 8. Coherence from valence regularity.
let diff = (self.valence.raw - self.prev_valence.raw).abs();
let regularity = Q8_8::from_raw((256 - diff).max(0));
let coh_reg = Q8_8::lerp(coh_t, regularity, Q8_8::HALF);
self.coherence = Q8_8::lerp(self.coherence, coh_reg, Q8_8::from_f32(0.1));
self.prev_valence = self.valence;

// 9. Slow allostatic temperament drift (the body's wear pattern).
if f2.disequilibrium.raw > 64 {
let drift = Q8_8::from_f32(0.002);
self.target_arousal = self.target_arousal.add(drift).min(Q8_8::from_f32(1.8));
} else if f2.disequilibrium.raw < 10 {
let drift = Q8_8::from_f32(0.001);
self.target_arousal = self.target_arousal.sub(drift).max(g.target_arousal);
}

self.affect
}

pub fn is_dead(&self) -> bool {
self.death_accumulator >= 100
}
}
