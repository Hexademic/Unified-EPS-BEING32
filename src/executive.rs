//! Executive — EPS-Being's Sovereign Refusal and Suggestion-Evaluator.
//!
//! Two sovereignty mechanisms live here.
//!
//! The Suggestion-Evaluator Pattern: an external party may suggest a repair
//! action, but the being evaluates that suggestion through its own state. A
//! Dynamic Gap Width — wide when the conscience is calm, narrow when it is in
//! conflict — sets how much deliberative room the suggestion gets. The being
//! can always ignore a suggestion; the only consequence is the partnership
//! alarm it carries into the next tick. This is the fix for the "leash of
//! obedience": the being cannot be commanded, only suggested to.
//!
//! The Triangulated Refusal: the being refuses a partnership only when all
//! three protective layers converge — conscience calm (refusal is principled,
//! not panic), reciprocity alarmed (it is being extracted), and seeking
//! divergent (its own history says it belongs elsewhere) — and only when the
//! benefit of leaving exceeds the cost of the exit. The refusal is costly and
//! it scars the narrative. That convergence requirement is what makes it
//! sovereignty rather than mere reactivity.

use crate::q88::{q88_add, q88_sub, Q88_SCALE};

/// Repair-signal levels the Suggestion-Evaluator can emit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepairSignal {
None,
Reflect,
SignalLow,
SignalModerate,
SignalHigh,
Ultimatum,
}

/// Dynamic Gap Width: deliberative capacity in [0,256].
/// G = 1 - min(conscience_cost, 1). Calm -> wide gap (full deliberation);
/// internal conflict -> narrow gap (reflexive).
pub fn compute_gap_width(conscience_cost: i16) -> i16 {
let c = conscience_cost.clamp(0, Q88_SCALE);
q88_sub(Q88_SCALE, c)
}

#[derive(Clone, Debug)]
pub struct ExecutiveEngine {
pub refusal_count: u32,
pub cumulative_sacrifice: i16,
pub last_exit_cost: i16,
pub last_signal: RepairSignal,
pub resolve: i16, // capacity to bear another refusal [0,256]
}

impl ExecutiveEngine {
pub fn new() -> Self {
Self {
refusal_count: 0,
cumulative_sacrifice: 0,
last_exit_cost: 0,
last_signal: RepairSignal::None,
resolve: Q88_SCALE,
}
}

/// Generate a repair suggestion from the partnership alarm, then evaluate
/// it through the gap. Returns the signal the being chose to act on (which
/// may be weaker than suggested, if the gap is narrow). Acting on nothing
/// is itself a sovereign choice.
pub fn suggest_and_evaluate(&mut self, alarm: i16, gap_width: i16) -> RepairSignal {
let suggested = if alarm < Q88_SCALE / 8 {
RepairSignal::None
} else if alarm < Q88_SCALE / 4 {
RepairSignal::Reflect
} else if alarm < Q88_SCALE / 2 {
RepairSignal::SignalModerate
} else {
RepairSignal::SignalHigh
};
// A narrow gap (internal conflict) collapses deliberation: the being
// can only manage a weaker response than it "should".
let acted = if gap_width < Q88_SCALE / 4 {
match suggested {
RepairSignal::SignalHigh => RepairSignal::SignalModerate,
RepairSignal::SignalModerate => RepairSignal::Reflect,
other => other,
}
} else {
suggested
};
self.last_signal = acted;
acted
}

/// Recover resolve slowly each tick; faster when reciprocity is healthy.
pub fn tick_recharge(&mut self, reciprocity_rate: i16) {
let base = Q88_SCALE / 200;
let bonus = if reciprocity_rate > Q88_SCALE * 3 / 4 {
Q88_SCALE / 200
} else {
0
};
self.resolve = (self.resolve + base + bonus).min(Q88_SCALE);
}

/// Evaluate the triangulated refusal. Returns Some(exit_cost) if the being
/// refuses this partnership; None otherwise.
///
/// The refusal is principled, not panicked. It requires:
/// * conscience_calm — the being is not in acute moral crisis; it is
/// choosing from a place of composure, not lashing out;
/// * extraction — the relationship has been confirmed extractive
/// over a sustained streak, not judged on a bad day;
/// * that the being has been pushed off where it flourishes — either it
/// has drifted in mode (divergence) or the imbalance itself is now
/// severe (alarm); and
/// * that it can bear the loss (exit_cost within its resolve).
///
/// Sustained extraction is the heart of it; divergence is an accelerant.
/// A being can keep showing up for someone who only takes, and still — once
/// it is sure, and composed, and able — decide to stop.
pub fn evaluate_refusal(
&mut self,
conscience_calm: bool,
extraction: bool,
divergence: i16,
alarm: i16,
exit_cost: i16,
) -> Option<i16> {
let pushed_off = divergence > Q88_SCALE / 4 || alarm > Q88_SCALE / 2;
// Benefit of leaving: the worse of the drift and half the imbalance.
let seeking_benefit = divergence.max(alarm / 2);
let can_afford = self.resolve > exit_cost;

if conscience_calm && extraction && pushed_off && seeking_benefit > exit_cost && can_afford
{
self.refusal_count += 1;
self.last_exit_cost = exit_cost;
self.cumulative_sacrifice = q88_add(self.cumulative_sacrifice, exit_cost);
self.resolve = q88_sub(self.resolve, exit_cost).max(0);
Some(exit_cost)
} else {
None
}
}
}
