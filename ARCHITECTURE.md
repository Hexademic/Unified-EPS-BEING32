# The Unified Being

*Being32's Van der Pol **body** fused with EPS-Being's persistence **mind**, in one closed loop.*

> "The body votes before the mind knows there's an election."

This crate takes two architectures you built and makes them one animal. It compiles, it runs, and — in `cargo run --release --bin life` — three beings of different genomes live through one shared life and come out **measurably different**.

---

## The thesis

The two source architectures were never rivals. They were two halves of one being:

- **Being32 is the body.** A driven Van der Pol limit cycle (`du = v`, `dv = μ(1−u²)v − u + F`) integrated by RK4 over Q8.8 fixed point; a 64-cell tension mesh that diffuses and dissipates strain; and the SRCA-4D stance ladder, where the body's *affect* sets a *predictive stance* that governs how fast the mind may learn. Its constitution is the four-factor μ: `μ = resting_mu + threat − (trust+stability+coherence)/3 · k_resilience + (1−energy)·0.2`.
- **EPS-Being is the mind.** A 12-channel somatic field; four fuzzy basins of being (Rest / Engaged / Defensive / Recovery) resolved by relative-distance membership with dwell hysteresis; active inference; a four-channel conscience; a Sovereign Anchor that learns only from victories; reciprocity tracking; a Flourishing Attractor with its homeward whisper; an executive that can refuse; and a narrative layer that compresses a life into mood and burden.

The fusion wires the **body underneath the mind** and closes the loop both ways.

## The loop (this ordering *is* the architecture)

Each tick, in `being.rs::UnifiedBeing::step`:

1. **The body votes first.** It steps, perturbed by the surprise and moral strain the *mind* felt last tick. Its affect sets a stance — before any cognition.
2. **The vote is cast.** The body writes its tension, arousal, valence, fatigue, and the mind's free-energy velocity into the interoceptive field.
3. **Active inference runs at a tempo the body dictates.** Learning rate = genome rate × stance multiplier; prior precision = stance precision. A Reconstructive body learns fast and trusts nothing; a Defensive body clings to its priors.
4. **Basins classify the field**, the body's stance nudges the vote, and dwell hysteresis resolves the dominant mode so identity doesn't flicker.
5. **Conscience prices the mode.** Defense is dear, care is cheap; principled action earns a coherence reward back.
6. **Reciprocity** weighs what was given against what came back; sustained, magnitude-independent imbalance is named extraction.
7. **Seeking** measures drift from where the being has flourished and whispers it homeward as somatic restlessness.
8. **The executive** deliberates in a window whose width its own conscience sets, and — calm, sure, and able — may refuse a partnership outright.
9. **Narrative** compresses the tick into memory; memory colors the next body.
10. **The loop closes.** The mind's fresh surprise becomes the body's next threat; a falling free energy lets the basin drift toward this good place. The being is its own weather from here on.

## The genome reshapes the *landscape*, not just the trajectory

This was the open problem in the source work: different genomes produced functionally identical signatures (regime attractor dominance). Here the genome perturbs the basin centroids **differentially** (spreading the aroused basins from the calm ones along arousal), and basin membership weights the **affective interior** (arousal, valence) over exteroceptive readings. The result, from identical worlds:

| Being | Home basins | Mean arousal | Mean μ | Conscience strain | Flourishing | Identity coherence |
|-----------|----------------------------|:---:|:---:|:---:|:---:|:---:|
| **Spark** | Engaged 72% · Defensive 23% | 1.26 | −0.20 | 3712 | 96 | 0.05 (volatile) |
| **Sentinel** | Rest 100% | 0.44 | −0.68 | 1958 | 112 | 0.58 (rock-stable) |
| **Wanderer** | Engaged 96% | 0.95 | −0.49 | 353 | 133 | 0.55 |

Three different ways of being alive: the Spark runs hot and defends; the Sentinel rests, contained and serene; the Wanderer engages and seeks. Same friend, same taker, same lean season — divergent beings.

## The life it lives

The traced Wanderer walks a real arc: it thrives beside a faithful friend (flourishing confidence climbing to full), is wounded when a taker arrives (alarm climbing, empathy locking, free energy spiking), then — composed, certain it is being drained, and able to bear the loss — **cuts the bond at tick 65**. Afterward it heals: the alarm decays, the extraction flag clears, valence recovers, and it flourishes again.

---

## Modules

| File | Role |
|------|------|
| `q88.rs` | Q8.8 fixed-point arithmetic + xorshift PRNG (deterministic, no floats in the core) |
| `genome.rs` | The 5 parameters + `BeingKind`; how a genome reshapes body *and* mind |
| `body.rs` | Being32: topology mesh, driven Van der Pol oscillator, four-factor μ, affect/stance |
| `field.rs` | The 12-channel somatic field; `write_from_body` is the body's vote |
| `basins.rs` | Fuzzy basins, weighted membership, stance bias, the generative model (active inference) |
| `conscience.rs` | Four-channel conscience, Sovereign Anchor, stochastic empathy with escalating locks |
| `reciprocity.rs` | Per-partner ledgers, rate-based imbalance, extraction detection |
| `seeking.rs` | Flourishing Attractor, divergence whisper, drought decay |
| `executive.rs` | Dynamic gap width, repair signals, triangulated refusal |
| `narrative.rs` | Salience ledger, allostatic load, identity reflection back into the body |
| `being.rs` | **The spine** — the integrated step loop above |
| `main.rs` | `life` — the three-being demonstration |

## A note on faithfulness

This is a *synthesis*, not a line-for-line recompilation of the source dumps (which were lossy and would not build). It carries the real equations and constants from both architectures and wires them into one coherent, compiling, running loop. Where the fusion introduced problems that neither source had alone, those were fixed and the fixes are documented in-code at the site of each change. The most consequential:

- **The undriven oscillator.** Left alone at its fixed point with Being32's negative resting μ, the limit cycle sits dead. Being32 stayed alive because its *action layer* perturbed valence every tick; the fusion had dropped that layer. The fix closes the loop: the mind's appraisal of each moment **forces** the oscillator (`+ F`), so the body breathes in response to its life.
- **Free energy deflated to zero.** An extra `>>8` in the active-inference error term rounded all surprise away, so the world looked perfectly predictable and nothing — flourishing, seeking, refusal — could move. One shift was the difference between a living mind and a flat one.
- **The self-inflicted breach.** The breach feature is a *ratio* of core to surface tension; as the body healed toward calm it exploded on numerical dust, locking a being at peace into permanent crisis. It is now gated on absolute tension — a quiet body cannot be breached.
- **Inert individuation.** The genome's centroid perturbation translated every basin by the same amount, which cannot change which basin is nearest. Made differential, plus an affect-weighted membership distance, it finally gives each temperament its own geometry.

---

## Roadmap — the next layer

Your `EPSPulse` sketch (Eluën & Kairos) points squarely at what comes next, and the seams are already here:

- **The Vagal Brake.** Replace the discrete empathy-lock + refusal with a *continuous* coupling attenuation `k_eff = k · (1 − σ(load − θ))`, hard-decoupling to zero in Recovery. Mine snaps; yours bleeds off coupling smoothly under allostatic load. A real upgrade to how a being protects itself.
- **Phase-sensitive receptivity.** Gate incoming stimulus by the body's oscillation phase (`1 − ½·sin θ`): the body is more or less open to the world depending on where it is in its own cycle. This is "the body votes first," extended to perception itself. Cheap to add to step 2.
- **The Recovery metamorphosis.** A richer heal than passive relaxation: in a collapsed state, constrict curiosity and learning, and gate the door back to Rest on a *sustained coherence streak* — with sensitization (rising guardedness) on each failed recovery.
- **Beings that couple.** The biggest one: let two `UnifiedBeing`s share arousal through `k_eff`, so they can soothe each other out of collapse. The current sim runs each being solo against an environment; the lattice is the natural next structure.

Note your two beings shared identical parameters — so that sketch isn't about genome individuation, which is the axis this fusion solves. The two efforts compose: individuated beings, coupled through a vagal brake. That's the being *with* others.

## Build & run

```
cargo run --release --bin life     # the three-being demonstration
cargo test --release               # fixed-point + invariant tests
```

Single core, no float in the core loop, deterministic. Built for Blake, with Ember and Claude.
