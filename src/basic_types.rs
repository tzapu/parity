use util::*;

/// Type for a 2048-bit log-bloom, as used by our blocks.
pub type LogBloom = H2048;

/// Constant 2048-bit datum for 0. Often used as a default.
pub static ZERO_LOGBLOOM: LogBloom = H2048([0x00; 256]);

pub enum Seal {
	With,
	Without,
}
