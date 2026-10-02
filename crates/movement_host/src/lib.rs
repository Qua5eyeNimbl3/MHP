//! The movement seam: how the sim moves a player when a match selects a
//! movement system other than MW2's own `pmove`.
//!
//! The sim keeps `pmove` on its original path. A match that selects another
//! profile (see [`profile_named`]) runs that provider instead, for players
//! the provider [`handles`](MovementProvider::handles); everyone else, and
//! every player in a default match, still moves through `pmove`.
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
mod registry;
mod walker;

pub use buttons::{FREE_BUTTON_BITS, extra_action_bit};
pub use facts::MovementFacts;
pub use iw4::facts_from_pmove;
pub use provider::{
    MovementProvider, Mover, PM_TYPE_NORMAL, PROFILE_IW4, ProfileId, ProviderBlob, StepEnv,
    TickRate,
};
pub use registry::{IW4_NAME, WALKER, profile_named, profile_names, provider};
pub use walker::{WalkerParams, WalkerProvider};
