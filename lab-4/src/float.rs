// Lab B — Floating-Point Safety
//
// Your task: Fix the buggy functions so that running
//
//   cargo run -- float
//
// no longer produces incorrect security decisions.
//
// ------------------------------------
// TODO
// ------------------------------------
// 1) Fix `ratio_check_buggy`:
//      - Explicitly reject NaN and infinity.
//      - Do not allow NaN to silently influence control flow.
//      - Define safe behavior on invalid input (return Err).
//
// 2) Fix `threshold_check_buggy`:
//      - Do NOT rely on direct float equality or unstable thresholds.
//      - Introduce an explicit epsilon
//      - Make the security decision robust and well-defined.
//
// Constraints:
//   - Do not modify `run()`.
//   - Do not use `unsafe`.
//   - Do not introduce new floating-point vulnerabilities.
// ------------------------------------

#[derive(Debug)]
pub enum LabError {
    InvalidInput,
    NaNOrInfinity,
    PrecisionLoss,
}

pub type Result<T> = std::result::Result<T, LabError>;

pub fn run() {
    println!("=== Lab B: Floating-Point Safety (buggy demo) ===");

    // ------------------------------------------------------------
    // Demo 1: NaN / infinity propagation
    // ------------------------------------------------------------
    // 0.0 / 0.0 = NaN
    match ratio_check_buggy(0.0, 0.0) {
        Ok(allowed) => println!("Ratio check with 0/0 says allowed? {allowed}"),
        Err(e) => println!("Ratio check with 0/0 returned error: {e:?}"),
    };

    // ------------------------------------------------------------
    // Demo 2: Threshold comparison instability
    // ------------------------------------------------------------
    let a = 0.1f64;
    let b = 0.2f64;
    let c = 0.3f64;

    match threshold_check_buggy(a, b, c) {
        Ok(ok) => println!("Threshold check (0.1 + 0.2 <= 0.3) says ok? {ok}"),
        Err(e) => println!("Threshold check returned error: {e:?}"),
    };
}

/// BUG #1: NaN / infinity not handled
///
/// Scenario:
///   ratio is used for a security decision.
///   NaN comparisons are always false, which can silently bypass logic.
pub fn ratio_check_buggy(numer: f64, denom: f64) -> Result<bool> {
    if !numer.is_finite() || !denom.is_finite() {
        return Err(LabError::NaNOrInfinity);
    }

    let r = numer / denom;

    if !r.is_finite() {
        return Err(LabError::NaNOrInfinity);
    }

    Ok(r < 0.9)
}

/// BUG #2: Float threshold instability
///
/// Scenario:
///   Security / financial threshold logic using floats.
pub fn threshold_check_buggy(a: f64, b: f64, c: f64) -> Result<bool> {
    if !a.is_finite() || !b.is_finite() || !c.is_finite() {
        return Err(LabError::NaNOrInfinity);
    }

    let diff = (a + b) - c;
    if !diff.is_finite() {
        return Err(LabError::NaNOrInfinity);
    }

    let max_val = a.abs().max(b.abs()).max(c.abs()).max(1.0);
    let scaled_eps = f64::EPSILON * max_val;

    Ok(diff <= scaled_eps)
}
