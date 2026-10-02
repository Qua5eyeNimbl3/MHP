//! The movement systems a match can select, by name.
//!
//! `iw4` is MW2's own `pmove`. It has no provider here: the sim keeps it on
//! its original path, so selecting it changes nothing.

use movement_iw4::MoveBounds;

use crate::provider::{MovementProvider, PROFILE_IW4, ProfileId};
use crate::walker::{WalkerParams, WalkerProvider};

pub const IW4_NAME: &str = "iw4";

/// A plain walker for trying the seam in a real match. Tuned near MW2's
/// numbers (190 units/s, gravity 800, a 39-unit jump) and MW2's standing
/// hull and trace mask, so it fits through the same doorways; it has no
/// stance, mantle, ladder or step-up.
pub static WALKER: WalkerProvider = WalkerProvider {
    params: WalkerParams {
        profile: ProfileId(1),
        name: "walker",
        walk_speed: 190.0,
        sprint_scale: 1.5,
        gravity: 800.0,
        // sqrt(2 * 800 * 39)
        jump_speed: 250.0,
        air_control: 0.1,
        bounds: MoveBounds {
            mins: [-15.0, -15.0, 0.0],
            maxs: [15.0, 15.0, 70.0],
            tracemask: 0x0281_0011,
        },
    },
};

static PROVIDERS: [&dyn MovementProvider; 1] = [&WALKER];

/// The provider for `profile`, or `None` for `iw4`, which the sim runs on
/// its own path, and for an id nothing registers.
#[must_use]
pub fn provider(profile: ProfileId) -> Option<&'static dyn MovementProvider> {
    PROVIDERS.iter().copied().find(|p| p.id() == profile)
}

/// The profile a host setting names, ignoring case.
#[must_use]
pub fn profile_named(name: &str) -> Option<ProfileId> {
    if name.eq_ignore_ascii_case(IW4_NAME) {
        return Some(PROFILE_IW4);
    }
    PROVIDERS
        .iter()
        .find(|p| p.name().eq_ignore_ascii_case(name))
        .map(|p| p.id())
}

/// Every name [`profile_named`] accepts, `iw4` first.
pub fn profile_names() -> impl Iterator<Item = &'static str> {
    core::iter::once(IW4_NAME).chain(PROVIDERS.iter().map(|p| p.name()))
}
