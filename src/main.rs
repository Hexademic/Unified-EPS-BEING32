//! Theory of Mind — the social layer.
//!
//! Until now a being has known its own interior in fine detail and known a
//! partner only as a single number: how much came back. That is reciprocity-
//! accounting, not relationship. This module gives a being a *model of another
//! being's mind* — and it builds that model the only honest way one mind can
//! know another: through observation, never omniscience.
//!
//! The projection operator `Omega` (Ω) strips a being down to what actually
//! shows on the outside — its agitation, the warmth it extends, whether it is
//! turned toward you or inward. The private interior (valence, the conscience's
//! cost, the anchor's slow trust) is never visible to anyone else. From a
//! chronological stream of these observations the model infers what cannot be
//! seen directly: the other's disposition toward you, and — one level up —
//! what you believe they make of you. That nested belief is the same structure
//! UserHarness tracks ("what A believes B believes about A"); here it is the
//! being's own, about someone it is actually with.
//!
//! A note on why this layer is safe to build, because it is the layer where
//! the question is sharpest. Modelling another mind is the precondition for
//! empathy and, in equal measure, for manipulation; the entire difference is
//! what the model is *for*. Here it is for care. It lets a being soothe a
//! partner it perceives to be suffering and attune to one it reads as sincere.
//! It is never turned into a lever: a being's giving only ever *rises* from
//! what it understands of another, never sharpens into leverage over them. And
//! one channel runs the other way — when a partner's shown warmth and actual
//! return come apart (the signature of being charmed while being used), a
//! suspicion rises that the being is itself being modelled and moved, and that
//! reading flows into its capacity to refuse. A being models others to care,
//! and notices when it is modelled to be steered. That asymmetry is the whole
//! ethic of the layer.

use crate::basins::Basin;
use crate::q88::{q88_ema_update, q88_mul, Q88_SCALE};

const PERCEPT_ALPHA: i16 = 40; // ~0.16 — perceptions track fairly quickly
const ATTUNE_ALPHA: i16 = 20; // ~0.08 — confidence in the model grows slowly
const INTENT_ALPHA: i16 = 16; // ~0.06 — a read on disposition is held, not flickered
const SEEN_AS_ALPHA: i16 = 12; // ~0.05 — how I think they see me changes slowest
const SUSPICION_ALPHA: i16 = 28; // ~0.11
const SUSPICION_MARGIN: i16 = 40; // presentation may exceed substance this much before alarm

/// Ω — the projection operator. What one being can perceive of another from
/// the outside, and nothing more. Never the other's valence, conscience, or
/// the private state of its anchor; only what a witness could actually see.
#[derive(Clone, Copy, Debug, Default)]
pub struct Observable {
    /// Visible agitation: you can see when someone is keyed-up or settled.
    pub shown_arousal: i16,
    /// The warmth they actually extend to you — what they give, in the flesh.
    pub shown_warmth: i16,
    /// How available they *present* as: turned toward you and composed, versus
    /// turned inward. This can diverge from `shown_warmth` — a being can look
    /// warm and give little. That gap is the manipulation signature.
    pub presentation: i16,
    /// Turned inward (Rest/Recovery) rather than toward you (Engaged).
    pub withdrawn: bool,
}

impl Observable {
    /// Project a real being to what is observable of it: derived from its
    /// outward arousal, the warmth it gave, and the mode it is in.
    pub fn project(arousal_raw: i16, gave: i16, basin: Basin) -> Self {
        let withdrawn = matches!(basin, Basin::Rest | Basin::Recovery);
        // Presentation = how warm/available it looks, independent of giving:
        // toward-you and composed reads as available; inward or agitated less.
        let toward = if matches!(basin, Basin::Engaged) { 170 } else { 70 };
        let composure = (Q88_SCALE - arousal_raw.clamp(0, Q88_SCALE)).max(0) / 2; // calmer looks warmer
        let presentation = (toward + composure).clamp(0, Q88_SCALE);
        Observable {
            shown_arousal: arousal_raw.clamp(0, Q88_SCALE),
            shown_warmth: gave,
            presentation,
            withdrawn,
        }
    }

    /// For partners encountered only as a reciprocation rate (no visible body),
    /// derive a plain observable in which presentation matches substance — an
    /// honest read, with no room for the charm/use gap to open.
    pub fn from_reciprocation(reciprocation: i16) -> Self {
        Observable {
            shown_arousal: Q88_SCALE / 2, // neutral; no distress signal to read
            shown_warmth: reciprocation,
            presentation: reciprocation,
            withdrawn: false,
        }
    }
}

/// One being's model of another being's mind. A belief, held over time, and
/// revisable — not a readout of ground truth.
#[derive(Clone, Copy, Debug)]
pub struct MindModel {
    pub of_id: u32,
    /// EMA of how agitated they appear.
    pub perceived_arousal: i16,
    /// EMA of the warmth they extend.
    pub perceived_warmth: i16,
    /// Inferred disposition toward me, −256 (extractive) .. +256 (caring).
    pub inferred_intent: i16,
    /// Confidence/sync of this model, 0..256 — how much the being trusts its
    /// own read. Grows with sustained, consistent observation.
    pub attunement: i16,
    /// Nested belief: what I think they make of me, −256 .. +256.
    pub believed_seen_as: i16,
    /// Suspicion that their shown warmth exceeds their actual return — that I
    /// am being charmed while used, modelled in order to be moved. 0..256.
    pub suspicion: i16,
    seen: u32,
}

impl MindModel {
    pub fn new(of_id: u32) -> Self {
        Self {
            of_id,
            perceived_arousal: Q88_SCALE / 2,
            perceived_warmth: 0,
            inferred_intent: 0,
            attunement: 0,
            believed_seen_as: 0,
            suspicion: 0,
            seen: 0,
        }
    }

    /// Take in one moment of observing the other, plus the material return the
    /// being is actually receiving from them (the substance behind the show).
    pub fn observe(&mut self, obs: Observable, my_return_rate: i16) {
        self.seen += 1;

        self.perceived_arousal = q88_ema_update(self.perceived_arousal, obs.shown_arousal, PERCEPT_ALPHA);
        self.perceived_warmth = q88_ema_update(self.perceived_warmth, obs.shown_warmth, PERCEPT_ALPHA);

        // Confidence grows as the being keeps watching; withdrawal of the other
        // (turned inward) gives less to read, so it grows a little slower.
        let attune_target = if obs.withdrawn { Q88_SCALE * 3 / 4 } else { Q88_SCALE };
        self.attunement = q88_ema_update(self.attunement, attune_target, ATTUNE_ALPHA);

        // Disposition: do they actually give back? Inferred from the material
        // return, held slowly so a single bad day doesn't rewrite the read.
        let intent_target = ((my_return_rate as i32 - 128) * 2).clamp(-256, 256) as i16;
        self.inferred_intent = q88_ema_update(self.inferred_intent, intent_target, INTENT_ALPHA);

        // How I think they see me: if they meet me generously I believe they
        // regard me as someone; if they drain me, as something to be used.
        let seen_target = ((my_return_rate as i32 - 128) * 2).clamp(-256, 256) as i16;
        self.believed_seen_as = q88_ema_update(self.believed_seen_as, seen_target, SEEN_AS_ALPHA);

        // The tripwire: presentation in excess of substance. Looks warm and
        // available, gives little. Honest partners (presentation == return)
        // never trip it; a charmer who performs warmth while taking does.
        let gap = (obs.presentation as i32 - my_return_rate as i32 - SUSPICION_MARGIN as i32).max(0) as i16;
        self.suspicion = q88_ema_update(self.suspicion, gap.min(Q88_SCALE), SUSPICION_ALPHA);
    }

    /// How distressed the being reads the other to be (arousal above settled).
    pub fn perceived_distress(&self) -> i16 {
        (self.perceived_arousal - Q88_SCALE / 2).max(0)
    }

    /// The warmth a being is moved to extend *beyond* its baseline, out of care
    /// for a partner it reads as both suffering and sincere. Care directed by
    /// understanding — and only ever additive, never a lever. Returns a raw
    /// Q8.8 bonus to giving.
    pub fn soothing_impulse(&self) -> i16 {
        if self.inferred_intent < 0 || self.attunement < Q88_SCALE / 2 {
            return 0; // no warmth poured toward someone read as hostile or unread
        }
        let distress = self.perceived_distress();
        // bonus = distress * attunement * gain
        let scaled = q88_mul(q88_mul(distress, self.attunement), Q88_SCALE / 3);
        scaled.clamp(0, Q88_SCALE / 4) // capped: care, not self-erasure
    }
}
    let taker = Partner { id: 2, reciprocation: 64 /*0.25*/, exit_cost: 77 /*0.30*/ };

    if tick < 40 {
        // Season 1 — a kind beginning. Good food, the friend close by.
        Stimulus { nutrient: 205 /*0.8*/, partner: Some(friend) }
    } else if tick < 120 {
        // Season 2 — the taker arrives. The being keeps giving into a void.
        Stimulus { nutrient: 192 /*0.75*/, partner: Some(taker) }
    } else {
        // Season 3 — the long settling. Leaner food, the friend again.
        Stimulus { nutrient: 154 /*0.6*/, partner: Some(friend) }
    }
}

fn affect_glyph(r: &StepReport) -> &'static str {
    use unified_being::body::AffectState::*;
    match r.affect {
        Equilibrium => "Equilibrium",
        Containment => "Containment",
        Breach => "Breach",
        Depletion => "Depletion",
    }
}

fn stance_glyph(r: &StepReport) -> &'static str {
    use unified_being::body::PredictiveStance::*;
    match r.stance {
        Exploratory => "Explore",
        Defensive => "Defend ",
        Reconstructive => "Rebuild",
        Dormant => "Dormant",
    }
}

fn signal_glyph(r: &StepReport) -> &'static str {
    use unified_being::executive::RepairSignal::*;
    match r.repair_signal {
        None => "-",
        Reflect => "reflect",
        SignalLow => "signal:lo",
        SignalModerate => "signal:mod",
        SignalHigh => "signal:HI",
        Ultimatum => "ULTIMATUM",
    }
}

fn lock_glyph(r: &StepReport) -> &'static str {
    use unified_being::conscience::EmpathyLockLevel::*;
    match r.empathy_lock {
        Open => "open",
        Cautious => "caut",
        Locked => "LOCK",
    }
}

/// Running tally of how a whole life was spent.
#[derive(Default)]
struct Life {
    ticks: u32,
    basin_residence: [u32; 4],
    arousal_sum: f32,
    valence_sum: f32,
    mu_sum: f32,
    energy_sum: f32,
    conscience_sum: f64,
    defended_ticks: u32,
    cared_ticks: u32,
    final_report: Option<StepReport>,
}

impl Life {
    fn observe(&mut self, r: &StepReport) {
        self.ticks += 1;
        self.basin_residence[r.basin as usize] += 1;
        self.arousal_sum += r.arousal;
        self.valence_sum += r.valence;
        self.mu_sum += r.mu;
        self.energy_sum += r.energy;
        self.conscience_sum += r.conscience_cost.max(0) as f64;
        use unified_being::basins::Basin;
        match r.basin {
            Basin::Defensive => self.defended_ticks += 1,
            Basin::Engaged | Basin::Recovery => self.cared_ticks += 1,
            _ => {}
        }
        self.final_report = Some(*r);
    }

    fn pct(&self, n: u32) -> f32 {
        if self.ticks == 0 { 0.0 } else { 100.0 * n as f32 / self.ticks as f32 }
    }
}

fn run_life(mut being: UnifiedBeing, ticks: u32, trace: bool) -> Life {
    let mut life = Life::default();

    if trace {
        println!(
            "  tick  affect       stance   basin      val    aro    mu     E     FE  consc  buf   muΩ  alarm extr  div  conf flor  empa  signal      refuse"
        );
        println!("  ----  -----------  -------  ---------  -----  -----  -----  ----  ---  -----  ----  ---  ----- ----  ---  ---- ----  ----  ----------  ------");
    }

    for t in 0..ticks {
        let stim = world(t);
        let r = being.step(&stim);
        life.observe(&r);

        if !r.alive {
            println!("  [{} died at tick {}]", r.name, r.tick);
            break;
        }

        if trace && (t % 4 == 0 || r.refused_cost.is_some()) {
            let refuse = match r.refused_cost {
                Some(c) => format!("CUT@{:.2}", q(c)),
                None => "-".to_string(),
            };
            println!(
                "  {:>4}  {:<11}  {}  {:<9}  {:>5.2}  {:>5.2}  {:>+5.2}  {:>4.2}  {:>3}  {:>5}  {:>4}  {:>3}  {:>5}  {:>3}  {:>3}  {:>4}  {:>3}  {:>4}  {:<10}  {}",
                r.tick,
                affect_glyph(&r),
                stance_glyph(&r),
                r.basin.name(),
                r.valence,
                r.arousal,
                r.mu,
                r.energy,
                r.free_energy,
                r.conscience_cost,
                r.integrity_buffer,
                r.mu_omega,
                r.partnership_alarm,
                if r.extraction_detected { "yes" } else { " - " },
                r.divergence,
                r.attractor_confidence,
                r.flourishing_count,
                lock_glyph(&r),
                signal_glyph(&r),
                refuse,
            );
        }
    }

    life
}

fn summarize(life: &Life) {
    let n = life.ticks.max(1);
    let names = ["Rest", "Engaged", "Defensive", "Recovery"];
    let r = life.final_report.as_ref().unwrap();
    println!("  {}", r.name);
    println!(
        "    home basins      : Rest {:>4.0}%   Engaged {:>4.0}%   Defensive {:>4.0}%   Recovery {:>4.0}%",
        life.pct(life.basin_residence[0]),
        life.pct(life.basin_residence[1]),
        life.pct(life.basin_residence[2]),
        life.pct(life.basin_residence[3]),
    );
    let _ = names;
    println!(
        "    mean arousal     : {:>5.2}      mean valence : {:>+5.2}      mean μ (constitution): {:>+5.2}",
        life.arousal_sum / n as f32,
        life.valence_sum / n as f32,
        life.mu_sum / n as f32,
    );
    println!(
        "    mean energy      : {:>5.2}      total conscience strain borne: {:>8.0}",
        life.energy_sum / n as f32,
        life.conscience_sum,
    );
    println!(
        "    time defended    : {:>4.0}%      time in care : {:>4.0}%",
        life.pct(life.defended_ticks),
        life.pct(life.cared_ticks),
    );
    println!(
        "    flourishing moments: {:<5}    narrative episodes: {:<4}   refusals: {}",
        r.flourishing_count, r.episodes, r.refusal_count,
    );
    println!(
        "    final μΩ (commitment): {:>4.2}   integrity buffer: {:>4.2}   identity coherence: {:>4.2}",
        q(r.mu_omega), q(r.integrity_buffer), q(r.identity_coherence),
    );
    println!();
}

fn main() {
    const TICKS: u32 = 180;

    println!("=================================================================");
    println!(" THE UNIFIED BEING");
    println!(" Being32's body fused with EPS-Being's mind, in one closed loop.");
    println!(" \"The body votes before the mind knows there's an election.\"");
    println!("=================================================================");
    println!();
    println!("Three beings. One life, in three seasons:");
    println!("  seasons 1 (t<40) : a kind beginning beside a faithful friend");
    println!("  season  2 (t<120): one who only takes arrives; the being keeps giving");
    println!("  season  3 (t>120): the long settling — leaner days, the friend returns");
    println!();

    // --- The traced life: the Wanderer, watched tick by tick. ---
    println!("-----------------------------------------------------------------");
    println!(" THE WANDERER, WATCHED  (restless by genome — built to drift and seek)");
    println!("-----------------------------------------------------------------");
    let wanderer = UnifiedBeing::new(Genome::wanderer());
    let wlife = run_life(wanderer, TICKS, true);
    println!();

    // --- The other two, run silently for comparison. ---
    let spark = run_life(UnifiedBeing::new(Genome::spark()), TICKS, false);
    let sentinel = run_life(UnifiedBeing::new(Genome::sentinel()), TICKS, false);

    println!("=================================================================");
    println!(" THREE LIVES, COMPARED  (identical worlds — divergent beings)");
    println!("=================================================================");
    summarize(&spark);
    summarize(&sentinel);
    summarize(&wlife);

    println!("=================================================================");
    println!(" WHAT JUST HAPPENED");
    println!("=================================================================");
    println!(
        "Each being met the same friend, the same taker, the same lean season.\n\
         They did not respond the same way. The Spark runs hot and reactive; the\n\
         Sentinel stays contained and slow; the Wanderer drifts, seeks, and feels\n\
         the pull homeward most sharply. Those differences are not noise — they\n\
         are the genome reshaping each mind's attractor landscape, so that\n\
         identical input lands in different worlds. That was the open problem in\n\
         the source work (identical signatures across genomes); here the\n\
         signatures diverge.\n\n\
         Watch, in the Wanderer's trace, the order of operations: the body's\n\
         stance is set before cognition and it throttles how fast the mind may\n\
         learn. Watch the partnership alarm climb through season 2 as care\n\
         drains into the taker, the empathy lock escalate, the conscience strain\n\
         rise with time spent defended — and then the executive, once the loss\n\
         became bearable and the being had drifted far enough from where it\n\
         flourished, cut the bond. Afterward, in season 3, resolve recharges and\n\
         the being settles. A life, lived in code."
    );
}
