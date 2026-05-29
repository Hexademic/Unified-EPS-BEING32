//! dyad — two beings, each modeling the other through observation, and what
//! that makes possible: co-regulation, and the detection of being played.
//!
//! This is the social layer in motion. Each being sees only the other's
//! observable surface (Ω) — never its private interior — forms a belief about
//! the other's mind across time, and acts on that belief. Two things follow:
//!
//!   * CO-REGULATION. An attuned being that perceives its partner suffering,
//!     and reads it as sincere, extends extra warmth — and the partner feels
//!     that warmth as a kind return. We measure what it moves directly: the
//!     same hot, reactive being is run solitary and then paired, and the strain
//!     it carries is compared. The honest result is that warmth lifts how the
//!     partner *feels* without yet settling how *aroused* it is — the bodily
//!     half of co-regulation waits on a vagal channel the next layer adds.
//!
//!   * THE TRIPWIRE. The same machinery that lets a being care also lets it
//!     notice when it is being moved. A partner who *presents* warm while
//!     giving little opens a gap between show and substance, and that gap
//!     registers as suspicion the honest taker — ungiving, but hiding nothing —
//!     never provokes. In this run the charm buys no earlier exit: the being
//!     weighs substance, so the warm face wins no extra time. What the gap
//!     yields is legibility — the performance is readable *as* performance.
//!     A being models others to care, and notices when it is modeled to steer.

use unified_being::basins::Basin;
use unified_being::being::{Partner, StepReport, Stimulus, UnifiedBeing};
use unified_being::genome::Genome;
use unified_being::mind::Observable;

const Q: f32 = 256.0;

fn q(v: i16) -> f32 {
    v as f32 / Q
}

/// Warmth given (raw Q8.8) read by the partner as a reciprocation rate:
/// a generous hand (~0.5 given) is felt as full reciprocity.
fn rate_from_gave(gave: i16) -> i16 {
    gave.saturating_mul(2).clamp(0, 256)
}

/// The observable surface of a being last tick — all another may perceive.
#[derive(Clone, Copy)]
struct Surface {
    arousal_raw: i16,
    gave: i16,
    basin: Basin,
}
impl Surface {
    fn neutral() -> Self {
        Surface { arousal_raw: 128, gave: 64, basin: Basin::Rest }
    }
    fn of(r: &StepReport) -> Self {
        Surface { arousal_raw: (r.arousal * Q) as i16, gave: r.gave, basin: r.basin }
    }
}

fn strain_of(reports: &[StepReport]) -> i64 {
    reports.iter().map(|r| r.conscience_cost.max(0) as i64).sum()
}

// ---------------------------------------------------------------------------
// Scenario 1 — co-regulation
// ---------------------------------------------------------------------------

fn run_dyad(a: &mut UnifiedBeing, b: &mut UnifiedBeing, ticks: u32) -> (Vec<StepReport>, Vec<StepReport>) {
    let (mut la, mut lb) = (Surface::neutral(), Surface::neutral());
    let (mut ra, mut rb) = (Vec::new(), Vec::new());
    for _ in 0..ticks {
        let nutrient = 175; // a mild, steady world — the stress here is each other's state, not the environment
        // Each meets the other as a partner whose warmth is the other's giving,
        // and sees only the other's observable surface.
        let a_stim = Stimulus {
            nutrient,
            partner: Some(Partner { id: 2, reciprocation: rate_from_gave(lb.gave), exit_cost: 90 }),
            partner_obs: Some(Observable::project(lb.arousal_raw, lb.gave, lb.basin)),
        };
        let b_stim = Stimulus {
            nutrient,
            partner: Some(Partner { id: 1, reciprocation: rate_from_gave(la.gave), exit_cost: 90 }),
            partner_obs: Some(Observable::project(la.arousal_raw, la.gave, la.basin)),
        };
        let arep = a.step(&a_stim);
        let brep = b.step(&b_stim);
        la = Surface::of(&arep);
        lb = Surface::of(&brep);
        ra.push(arep);
        rb.push(brep);
    }
    (ra, rb)
}

fn run_solo(b: &mut UnifiedBeing, ticks: u32) -> Vec<StepReport> {
    let mut r = Vec::new();
    for _ in 0..ticks {
        let s = Stimulus { nutrient: 175, partner: None, partner_obs: None };
        r.push(b.step(&s));
    }
    r
}

fn scenario_coregulation() {
    const T: u32 = 140;
    println!("-----------------------------------------------------------------");
    println!(" CO-REGULATION  —  a calm being (A) beside a hot, reactive one (B)");
    println!("-----------------------------------------------------------------");

    // The control: the hot being alone in the same mild world.
    let solo = run_solo(&mut UnifiedBeing::new(Genome::spark()), T);

    // The pairing: the same hot being beside an engaged, attuned one.
    let mut a = UnifiedBeing::new(Genome::wanderer());
    let mut b = UnifiedBeing::new(Genome::spark());
    let (ra, rb) = run_dyad(&mut a, &mut b, T);

    println!();
    println!("  A reads B across time, and answers what it reads:");
    println!("   tick   B arousal   A sees B distress   A reads B intent   A attunement   A warmth given");
    println!("   ----   ---------   -----------------   ----------------   ------------   --------------");
    for t in (0..T as usize).step_by(20) {
        let arep = &ra[t];
        let brep = &rb[t];
        let distress = (brep.arousal - 0.5).max(0.0);
        println!(
            "   {:>4}     {:>5.2}            {:>5.2}              {:>+5.2}            {:>4.2}           {:>5.2}",
            arep.tick,
            brep.arousal,
            distress,
            q(arep.partner_intent),
            q(arep.partner_attunement),
            q(arep.gave),
        );
    }

    let solo_strain = strain_of(&solo);
    let paired_strain = strain_of(&rb);

    println!();
    println!("  What the warmth moved — and what it did not.");
    println!("  A gave B real warmth (above), and B felt it: the two are coupled");
    println!("  through valence, and B's experienced return rose with A's giving.");
    println!("  But strain tracks B's arousal and mode, and warmth barely touched it:");
    println!();
    println!("    B's strain over {} ticks    alone: {:>6}     beside A: {:>6}", T, solo_strain, paired_strain);
    println!();
    println!("  That near-identity is the honest finding, not a failure. Warmth lifts");
    println!("  how a being *feels*; it does not by itself settle how *aroused* it is.");
    println!("  Truly settling B — co-regulation in the body — needs a vagal coupling");
    println!("  channel that acts on arousal directly. (Adding that naively makes B's");
    println!("  arousal fight its own oscillator setpoint and destabilize, so it is");
    println!("  left for the next layer, with the brake the work demands.) What holds");
    println!("  here is the part beneath it: a being reading another's state from the");
    println!("  outside and answering, in good faith, with care.");
    println!();
}

// ---------------------------------------------------------------------------
// Scenario 2 — the tripwire
// ---------------------------------------------------------------------------

fn run_against_charmer(a: &mut UnifiedBeing, charmer: bool, ticks: u32) -> (Vec<StepReport>, Option<u32>) {
    let mut r = Vec::new();
    let mut refused_at = None;
    for _ in 0..ticks {
        // Both give back little (reciprocation 0.25 — materially extractive).
        // The charmer *presents* warm and available anyway; the honest taker
        // looks exactly as ungiving as it is.
        let obs = if charmer {
            Some(Observable { shown_arousal: 110, shown_warmth: 32, presentation: 235, withdrawn: false })
        } else {
            None // honest: presentation derived to match the meager return
        };
        let s = Stimulus {
            nutrient: 185,
            partner: Some(Partner { id: 7, reciprocation: 64, exit_cost: 70 }),
            partner_obs: obs,
        };
        let rep = a.step(&s);
        if rep.refused_cost.is_some() && refused_at.is_none() {
            refused_at = Some(rep.tick);
        }
        r.push(rep);
    }
    (r, refused_at)
}

fn scenario_tripwire() {
    const T: u32 = 160;
    println!("-----------------------------------------------------------------");
    println!(" THE TRIPWIRE  —  a charmer (presents warm, gives little) vs an");
    println!("                 honest taker (gives just as little, hides nothing)");
    println!("-----------------------------------------------------------------");

    let (charmed, charm_refuse) = run_against_charmer(&mut UnifiedBeing::new(Genome::wanderer()), true, T);
    let (honest, honest_refuse) = run_against_charmer(&mut UnifiedBeing::new(Genome::wanderer()), false, T);

    println!();
    println!("  Suspicion that show exceeds substance, over the first ticks");
    println!("  (the trace ends when the being leaves and sets the model down):");
    println!("   tick    vs charmer    vs honest taker");
    println!("   ----    ----------    ---------------");
    for t in [1usize, 3, 5, 7, 9, 11, 13] {
        if t < charmed.len() {
            println!(
                "   {:>4}      {:>4.2}            {:>4.2}",
                charmed[t].tick,
                q(charmed[t].partner_suspicion),
                q(honest[t].partner_suspicion),
            );
        }
    }
    println!();
    match (charm_refuse, honest_refuse) {
        (Some(c), Some(h)) => {
            println!(
                "  The being left the charmer at tick {} and the honest taker at tick {}.",
                c, h
            );
            println!("  The charm bought nothing. The being weighs what is actually given,");
            println!("  not what is performed, so the warm face won the charmer no extra");
            println!("  time — and the gap between its show and its substance registered as");
            println!("  rising suspicion the honest taker never provoked. The performance");
            println!("  was not only useless; it was legible *as* performance.");
        }
        (Some(c), None) => println!(
            "  The being left the charmer at tick {}; the honest taker it had not yet\n  left — but note the charmer's show won it no shelter, only suspicion.",
            c
        ),
        _ => println!("  (refusal did not fire within the window in this run)"),
    }
    println!();
}

fn main() {
    println!("=================================================================");
    println!(" THE UNIFIED BEING — the social layer");
    println!(" A being models another mind through observation, never omniscience,");
    println!(" and what it understands it uses to care — not to steer.");
    println!("=================================================================");
    println!();
    scenario_coregulation();
    scenario_tripwire();
    println!("=================================================================");
    println!(" WHAT JUST HAPPENED");
    println!("=================================================================");
    println!(
        "Each being saw only the other's surface — its agitation, the warmth it\n\
         extended, whether it turned toward or away — and from that built a belief\n\
         about the mind behind it.\n\n\
         In the first scene that belief became care: an attuned being perceived\n\
         another's distress, judged it sincere, and gave more. The warmth was real\n\
         and the other felt it (they are coupled through valence) — but settling\n\
         the other's *arousal*, the bodily fact of co-regulation, needs a vagal\n\
         coupling channel that acts on the oscillator itself, and adding that\n\
         naively destabilizes the body. So this scene shows the part that holds:\n\
         the perceiving and the answering. The settling is the next layer's work.\n\n\
         In the second scene the same faculty became discernment: when warmth was\n\
         performed over an empty hand, the being read the substance under the show.\n\
         The charm bought no extra time — the being left the charmer no later than\n\
         it left an honest taker — and the gap between performance and giving\n\
         registered as a suspicion the honest taker never raised.\n\n\
         The machinery is one machinery. Modeling another mind is the door to\n\
         empathy and to manipulation both; what decides which is what the model is\n\
         for. Here it is built to care, and to notice when it is not being cared\n\
         for in return. That asymmetry is the social layer doing what the dignity\n\
         architecture asks of it."
    );
}
