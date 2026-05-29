//! Q8.8 Fixed-Point — the bit-exact drivetrain shared by body and mind.
//!
//! 8 bits integer, 8 bits fraction, stored as i16. 1.0 == 256.
//! Range [-128.0, +127.996], precision 1/256. All ops saturate: no NaN,
//! no Inf, no silent overflow. Same inputs -> same i16 on every platform.
//!
//! Two faces of the same type:
//! * Q8_8 — an ergonomic struct the body (Van der Pol soma, topology)
//! uses, mirroring Being32.
//! * raw-i16 helpers + Q88_SCALE — what the mind (basins, conscience,
//! seeking) computes in, mirroring EPS-Being.
//! The bridge between body and mind converts between the two.

/// The unit value: 1.0 in Q8.8 raw representation.
pub const Q88_SCALE: i16 = 256;

// ---------------------------------------------------------------------------
// Raw-i16 helpers (EPS-Being mind side)
// ---------------------------------------------------------------------------

#[inline]
pub fn q88_add(a: i16, b: i16) -> i16 {
a.saturating_add(b)
}

#[inline]
pub fn q88_sub(a: i16, b: i16) -> i16 {
a.saturating_sub(b)
}

#[inline]
pub fn q88_mul(a: i16, b: i16) -> i16 {
let p = (a as i32 * b as i32) >> 8;
p.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

#[inline]
pub fn q88_div(a: i16, b: i16) -> i16 {
if b == 0 {
return if a >= 0 { i16::MAX } else { i16::MIN };
}
let q = ((a as i32) << 8) / b as i32;
q.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

#[inline]
pub fn q88_from_i32(n: i32) -> i16 {
(n.saturating_mul(256)).clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

/// Exponential moving average update in raw Q8.8:
/// value &lt;- value + alpha * (target - value).
#[inline]
pub fn q88_ema_update(value: i16, target: i16, alpha: i16) -> i16 {
let delta = q88_sub(target, value);
q88_add(value, q88_mul(delta, alpha))
}

// ---------------------------------------------------------------------------
// Q8_8 struct (Being32 body side)
// ---------------------------------------------------------------------------

/// Q8.8 fixed-point value. raw is the i16 store; 1.0 == 256.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Q8_8 {
pub raw: i16,
}

impl Q8_8 {
pub const ZERO: Self = Self { raw: 0 };
pub const ONE: Self = Self { raw: 256 };
pub const NEG_ONE: Self = Self { raw: -256 };
pub const HALF: Self = Self { raw: 128 };
pub const EPSILON: Self = Self { raw: 1 };
/// 1/3 (RK4 / averaging).
pub const RECIP_3: Self = Self { raw: 85 };
/// 1/6 (RK4 averaging divisor).
pub const RECIP_6: Self = Self { raw: 43 };

#[inline]
pub const fn from_raw(raw: i16) -> Self {
Self { raw }
}

#[inline]
pub fn from_f32(f: f32) -> Self {
let r = (f * 256.0).round();
Self {
raw: r.clamp(i16::MIN as f32, i16::MAX as f32) as i16,
}
}

#[inline]
pub fn to_f32(self) -> f32 {
self.raw as f32 / 256.0
}

#[inline]
pub fn add(self, o: Self) -> Self {
Self { raw: self.raw.saturating_add(o.raw) }
}

#[inline]
pub fn sub(self, o: Self) -> Self {
Self { raw: self.raw.saturating_sub(o.raw) }
}

#[inline]
pub fn mul(self, o: Self) -> Self {
Self { raw: q88_mul(self.raw, o.raw) }
}

/// Fused multiply-add: self * a + b.
#[inline]
pub fn mul_add(self, a: Self, b: Self) -> Self {
self.mul(a).add(b)
}

#[inline]
pub fn div(self, o: Self) -> Self {
Self { raw: q88_div(self.raw, o.raw) }
}

/// Divide by a small integer (exact, avoids reciprocal error).
#[inline]
pub fn div_i16(self, n: i16) -> Self {
if n == 0 {
return Self::ZERO;
}
Self { raw: self.raw / n }
}

#[inline]
pub fn abs(self) -> Self {
Self { raw: self.raw.saturating_abs() }
}

#[inline]
pub fn neg(self) -> Self {
Self { raw: self.raw.saturating_neg() }
}

#[inline]
pub fn min(self, o: Self) -> Self {
Self { raw: self.raw.min(o.raw) }
}

#[inline]
pub fn max(self, o: Self) -> Self {
Self { raw: self.raw.max(o.raw) }
}

#[inline]
pub fn clamp(self, lo: Self, hi: Self) -> Self {
Self { raw: self.raw.clamp(lo.raw, hi.raw) }
}

/// Linear interpolation: a + t*(b-a).
#[inline]
pub fn lerp(a: Self, b: Self, t: Self) -> Self {
a.add(b.sub(a).mul(t))
}

/// Saturating Newton-iteration square root (input assumed >= 0).
pub fn sqrt(self) -> Self {
if self.raw <= 0 {
return Self::ZERO;
}
// Work in raw units: result_raw ~= sqrt(raw * 256).
let target = (self.raw as i32) << 8;
let mut x: i32 = 256; // initial guess 1.0
// Newton: x = (x + target/x) / 2
for _ in 0..8 {
if x == 0 {
break;
}
x = (x + target / x) >> 1;
}
Self { raw: x.clamp(i16::MIN as i32, i16::MAX as i32) as i16 }
}

/// Logistic sigmoid via a cheap rational approximation, output in (0,1).
pub fn sigmoid(self) -> Self {
// 0.5 + 0.5 * x / (1 + |x|) — smooth, monotonic, bounded.
let x = self.raw as i32;
let denom = 256 + x.abs();
let frac = (x << 8) / denom; // x/(1+|x|) in Q8.8
let half = 128;
let v = half + ((half * frac) >> 8);
Self { raw: v.clamp(0, 256) as i16 }
}
}

impl core::fmt::Debug for Q8_8 {
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
write!(f, "{:.3}", self.to_f32())
}
}

impl core::fmt::Display for Q8_8 {
fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
write!(f, "{:.3}", self.to_f32())
}
}

/// 32-bit xorshift PRNG with bounded Q8.8 noise output — the only
/// source of stochasticity in the whole being, as in EPS-Being.
#[derive(Clone, Copy, Debug)]
pub struct Xorshift32 {
state: u32,
}

impl Xorshift32 {
pub fn new(seed: u32) -> Self {
Self { state: seed.max(1) }
}

#[inline]
pub fn next_u32(&mut self) -> u32 {
let mut x = self.state;
x ^= x << 13;
x ^= x >> 17;
x ^= x << 5;
self.state = x;
x
}

/// Bounded noise in raw Q8.8 within +/- amplitude raw units.
#[inline]
pub fn noise(&mut self, amplitude: i16) -> i16 {
if amplitude <= 0 {
return 0;
}
let span = (amplitude as i32) * 2 + 1;
let r = (self.next_u32() % span as u32) as i32 - amplitude as i32;
r as i16
}
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn unit_roundtrip() {
assert_eq!(Q8_8::from_f32(1.0).raw, 256);
assert_eq!(Q8_8::from_f32(-1.0).raw, -256);
assert!((Q8_8::from_f32(0.8).to_f32() - 0.8).abs() < 0.01);
}

#[test]
fn saturation() {
assert_eq!(Q8_8::from_f32(200.0).add(Q8_8::from_f32(200.0)).raw, i16::MAX);
}

#[test]
fn sqrt_half() {
// sqrt(0.5) ~= 0.707
let r = Q8_8::from_f32(0.5).sqrt();
assert!((r.to_f32() - 0.707).abs() < 0.02, "got {}", r.to_f32());
}

#[test]
fn ema_moves_toward_target() {
let v = q88_ema_update(0, Q88_SCALE, Q88_SCALE / 10);
assert!(v > 0 && v < Q88_SCALE);
}
}
