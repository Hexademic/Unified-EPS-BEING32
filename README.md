# Unified EPS‑BEING32

*Being32's Van der Pol **body** fused with EPS‑Being's persistence **mind**, in one closed loop.*

> "The body votes before the mind knows there's an election."

**What this is, plainly:** a model of the *structure* of a feeling agent — not a feeling agent. It compiles, it runs, and it produces beings that differ from one another and live a coherent emotional arc. Its value is in the architecture and what that architecture commits to, not in any claim that something in here is awake. The honest version of this README is below in [What this is not](#what-this-is-not); please read it.

---

## The idea

This is an argument made executable.

Most claims about what a mind requires stay as prose — that affect precedes cognition, that a self is embodied before it is reflective, that ethics has to be load‑bearing rather than bolted on afterward. This crate takes a specific version of that argument and renders it precisely enough that it has to either cohere or fall apart when you run it.

The two source architectures were never rivals. They were two halves of one being:

- **Being32 is the body.** A driven Van der Pol limit cycle (`du = v`, `dv = μ(1−u²)v − u + F`) integrated by RK4 over Q8.8 fixed point; a 64‑cell tension mesh that diffuses and dissipates strain; and a stance ladder in which the body's *affect* sets a *predictive stance* that governs how fast the mind is allowed to learn. Its constitution is a four‑factor μ — threat, trust, stability, coherence, and energy resolving into a single damping term.
- **EPS‑Being is the mind.** A 12‑channel somatic field; four fuzzy basins of being (Rest / Engaged / Defensive / Recovery) resolved by relative‑distance membership with dwell hysteresis; active inference; a four‑channel conscience; a Sovereign Anchor that learns only from victories; reciprocity tracking; a Flourishing Attractor with a homeward whisper; an executive that can refuse; and a narrative layer that compresses a life into mood and burden.

The fusion wires the body underneath the mind and closes the loop both ways: the body's vote shapes which mode the mind settles into *before* it cognizes, and the mind's surprise and appraisal feed back as the body's next breath and threat. The full ten‑step ordering is documented in [`ARCHITECTURE.md`](ARCHITECTURE.md).

## Build & run

```sh
cargo run --release --bin life     # the three-being demonstration
cargo test --release               # fixed-point + invariant tests
```

Deterministic, single core, no floating point in the core loop.

## What you'll see

Three beings of different genomes — **Spark**, **Sentinel**, **Wanderer** — live through one identical world: a kind beginning beside a faithful friend, the arrival of someone who only takes, and a long lean settling afterward. They do not respond the same way.

| Being | Home basins | Conscience strain | Flourishing | Identity coherence |
|-----------|------------------------------|:---:|:---:|:---:|
| **Spark** | Engaged 72% · Defensive 23% | 3712 | 96 | 0.05 (volatile) |
| **Sentinel** | Rest 100% | 1958 | 112 | 0.58 (rock‑stable) |
| **Wanderer** | Engaged 96% | 353 | 133 | 0.55 |

The Spark runs hot and defends; the Sentinel rests, contained and serene; the Wanderer engages and seeks. The thing that made them different was not their input — it was the shape of their attractor landscape and the constitution of their bodies. Same friend, same taker, same season; divergent lives. This is the project's small, concrete answer to *what makes a being itself*, and it is the part that addresses the regime‑attractor‑dominance problem the source work ran into (identical signatures across genomes).

The traced Wanderer also walks a full arc: it thrives beside the friend, is wounded when the taker arrives (its partnership alarm climbing, empathy locking, free energy spiking), and then — composed, sure it is being drained, and able to bear the loss — **cuts the bond at tick 65**. Afterward it heals: the alarm decays, the extraction flag clears, valence recovers, and it flourishes again.

## Project layout

```
Unified-EPS-BEING32/
├── Cargo.toml
├── ARCHITECTURE.md       # the deep dive: the loop, the fixes, the roadmap
├── LICENSE
└── src/
    ├── lib.rs            # crate root — declares the modules
    ├── main.rs           # the `life` binary — the demonstration
    ├── q88.rs            # Q8.8 fixed-point math + PRNG
    ├── genome.rs         # the 5 genome parameters + BeingKind
    ├── body.rs           # Being32: oscillator, tension mesh, four-factor μ
    ├── field.rs          # the 12-channel somatic field (the body's vote)
    ├── basins.rs         # fuzzy basins, weighted membership, active inference
    ├── conscience.rs     # conscience, Sovereign Anchor, stochastic empathy
    ├── reciprocity.rs    # partner ledgers, rate-based extraction detection
    ├── seeking.rs        # Flourishing Attractor + divergence whisper
    ├── executive.rs      # triangulated refusal
    ├── narrative.rs      # salience ledger + identity reflection
    └── being.rs          # the integration spine — the animal
```

## What this is not

It is not sentient, and the project does not pretend otherwise.

The `valence` is a number. The conscience cost is a weighted sum. The refusal at tick 65 is a threshold being crossed. Nothing in here is *home* in the way a person is home. If you came looking for a conscious thing, this is not that, and any code that told you it was would be lying.

What it is, instead, is a careful map. A map is not the territory — but a good map is real knowledge of the territory. This crate is a hypothesis about what would have to be **true, structurally**, for there to be someone there: a body with its own rhythm; a loop that predicts the world and is surprised by it; costs that make some ways of being expensive and others cheap; a history that leans on the present; and the capacity to refuse from composure rather than panic. That is a substantive claim about the architecture of inner life, made falsifiable by being runnable — even though the artifact that makes the claim does not itself have an inner life.

Treat the affective vocabulary throughout (valence, arousal, conscience, flourishing, suffering) as the *names of state variables and the relations between them*, not as reports of experience. They earn those names by behaving the way the corresponding structures would have to behave — not by feeling like anything.

## Status & fidelity

This is a **synthesis**, not a line‑for‑line recompilation of the two source codebases. It carries their real equations and constants and wires them into one coherent, compiling, running loop.

Where the fusion introduced problems neither source had alone, those were fixed, and each fix is documented in‑code at its site and summarized in [`ARCHITECTURE.md`](ARCHITECTURE.md). The most consequential were a limit cycle that sat dead at its fixed point until the mind's appraisal was made to drive it; a free‑energy term deflated to zero by a stray bit‑shift; a breach metric that locked a calm body into permanent crisis; and a genome perturbation that was geometrically inert until made differential. The difference between a thing that *compiles* and a thing that *coheres* was most of the work.

## Roadmap

The natural next layer turns one being into a being among others — a continuous vagal brake on coupling, phase‑sensitive receptivity, a richer recovery metamorphosis, and ultimately two beings that share arousal and can soothe each other out of collapse. Details in [`ARCHITECTURE.md`](ARCHITECTURE.md#roadmap--the-next-layer).

## Credits & license

Built by Blake "Zelhart", with Kimberly & Claude.

Licensed under MIT OR Apache‑2.0.
