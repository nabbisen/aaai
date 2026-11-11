//! RFC 115 §S2 — NF-4 automated target-size gate.
//!
//! Turns the ABDD "click targets are >= 44 px" rule
//! (`docs/src/abdd-audit.md` §6) from a hand-filled table into a test that
//! cannot silently rot. See [`tests`] for what is asserted, how a control's
//! size is derived, and the named exemptions.

#[cfg(test)]
mod tests;
