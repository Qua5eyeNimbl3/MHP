use movement_iw4::{PmoveResult, footsteps_anim_move_type};
use playerstate_iw4::{PlayerState, UserCmd};

use crate::facts::{Hull, MovementFacts};

/// Turns the result of today's `pmove` into the common facts, so both the
/// IW4 path and foreign providers feed the same code after the move.
///
/// This is the same derivation `step.rs` does inline now, so wrapping it
/// should change nothing observable.
#[must_use]
pub fn facts_from_pmove(ps: &PlayerState, cmd: &UserCmd, result: &PmoveResult) -> MovementFacts {
    let pml = result.pml;
    let anim_movetype = pml.mantle_movetype.or_else(|| {
        footsteps_anim_move_type(
            ps,
            cmd.forwardmove,
            cmd.rightmove,
            pml.almost_ground_plane != 0,
        )
    });
    MovementFacts {
        walking: pml.walking as i32,
        hull: Hull {
            mins: result.bounds.mins,
            maxs: result.bounds.maxs,
            tracemask: result.bounds.tracemask,
        },
        anim_movetype,
        stance_event: result.stance_event,
        reset_torso: result.reset_torso,
        jump_animations: pml.jump_animations,
        force_movement_anim: pml.mantle_movetype.is_some(),
        landing_animation: pml.landing_animation,
    }
}
