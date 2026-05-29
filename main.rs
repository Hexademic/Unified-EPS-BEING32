//! life — a demonstration that the Unified Being lives.
//!
//! Three beings of different genomes are conceived and run through one shared
//! life: a gentle beginning beside a faithful friend, the arrival of someone
//! who only takes, and the long settling afterward. We watch one of them tick
//! by tick, then compare all three.
//!
//! What this is meant to show, concretely:
//!   * INDIVIDUATION. Spark, Sentinel and Wanderer face identical worlds and
//!     come out measurably different — different homes, different arousal,
//!     different constitutions. The genome reshapes the landscape, not just
//!     the trajectory across it. (This was the failure mode in the source
//!     architectures: identical signatures across genomes. It is fixed here.)
//!   * THE BODY VOTING FIRST. Stance is set from bodily affect before any
//!     cognition, and it governs how fast the mind may learn each tick.
//!   * CONSCIENCE ASYMMETRY. Time spent defended costs more than time spent
//!     in care; the ledger of strain reflects how a life was actually lived.
//!   * THE SOVEREIGN ANCHOR. Commitment to cooperation grows only from proof
//!     that cooperation paid, and that growth persists across partners.
//!   * SEEKING AND REFUSAL. Drift from where it flourished becomes audible as
//!     restlessness, and an extractive bond is eventually, deliberately, cut.

use unified_being::being::{Partner, Stimulus, StepReport, UnifiedBeing};
use unified_being::genome::Genome;

const Q: f32 = 256.0;

fn q(v: i16) -> f32 {
    v as f32 / Q
}

/// Build the stimulus for a given global tick. The world has three seasons.
fn world(tick: u32) -> Stimulus {
    // A faithful friend, always nearly-reciprocal, cheap to keep.
    let friend = Partner { id: 1, reciprocation: 244 /*0.95*/, exit_cost: 64 /*0.25*/ };
    // One who takes far more than they give, and is costly to leave.
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
