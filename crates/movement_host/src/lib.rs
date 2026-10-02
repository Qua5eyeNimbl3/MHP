//! The movement seam: one entry point the sim calls instead of `pmove`.
//!
//! Draft. Nothing here is wired into `sim::step` yet. The shapes below are
//! fitted to what `step.rs` reads and writes around its `pmove` call.
//!
//! Rules for every provider, so authority, prediction and replay agree:
//! no `std`, no clocks, no threads, no randomness that is not seeded from
//! the tick, and float math through `libm` as `movement_iw4` does.
#![no_std]
#![forbid(unsafe_code)]

mod buttons;
mod facts;
mod iw4;
mod provider;
mod walker;

pub use buttons::{FREE_BUTTON_BITS, extra_action_bit};
pub use facts::{Hull, MovementFacts};
pub use iw4::facts_from_pmove;
pub use provider::{
    MovementProvider, Mover, PM_TYPE_NORMAL, PROFILE_IW4, ProfileId, ProviderBlob, StepEnv,
    TickRate,
};
pub use walker::{WalkerParams, WalkerProvider};
