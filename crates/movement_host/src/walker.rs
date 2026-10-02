use movement_iw4::{
    GroundTraceInput, ViewAngleClamp, footsteps_anim_move_type, update_view_angles,
};
use playerstate_iw4::{PlayerState, UserCmd, buttons};
use trace_iw4::{ENTITYNUM_NONE, ENTITYNUM_WORLD};

use crate::facts::{Hull, MovementFacts};
use crate::provider::{MovementProvider, Mover, ProfileId, StepEnv, TickRate};

/// The "parametric" class: a simple walker whose feel is all constants.
/// Axis-separated sweeps, no acceleration curve and no step-up, so it
/// cannot climb a block without jumping. It is here to prove the seam, not
/// to match any particular game.
#[derive(Clone, Copy, Debug)]
pub struct WalkerParams {
    pub profile: ProfileId,
    /// Map units per second.
    pub walk_speed: f32,
    pub sprint_scale: f32,
    pub gravity: f32,
    pub jump_speed: f32,
    /// 0 keeps air velocity, 1 gives full ground control in the air.
    pub air_control: f32,
    pub hull: Hull,
}

#[derive(Clone, Copy, Debug)]
pub struct WalkerProvider {
    pub params: WalkerParams,
}

const PITCH_LIMIT: f32 = 85.0;

impl MovementProvider for WalkerProvider {
    fn id(&self) -> ProfileId {
        self.params.profile
    }

    fn tick_rate(&self) -> TickRate {
        TickRate::Variable
    }

    fn hull(&self, _ps: &PlayerState) -> Hull {
        self.params.hull
    }

    fn step(&self, mover: &mut Mover<'_>, cmd: &mut UserCmd, env: &StepEnv<'_>) -> MovementFacts {
        let p = &self.params;
        let ps = &mut *mover.ps;
        let msec = cmd.server_time.wrapping_sub(ps.command_time).clamp(1, 200);
        let dt = msec as f32 * 0.001;
        ps.command_time = cmd.server_time;

        update_view_angles(
            ps,
            cmd,
            ViewAngleClamp {
                pitch_up: PITCH_LIMIT,
                pitch_down: PITCH_LIMIT,
                unclamped_pitch_bit: false,
            },
        );

        let was_grounded = ps.ground_entity_num != i32::from(ENTITYNUM_NONE);

        let yaw = ps.viewangles[1].to_radians();
        let (sin, cos) = (libm::sinf(yaw), libm::cosf(yaw));
        let forward = f32::from(cmd.forwardmove) / 127.0;
        let right = f32::from(cmd.rightmove) / 127.0;
        let mut speed = p.walk_speed * env.weapon_speed_scale;
        if cmd.buttons & buttons::SPRINT != 0 {
            speed *= p.sprint_scale;
        }
        let wish = [
            (cos * forward + sin * right) * speed,
            (sin * forward - cos * right) * speed,
        ];
        let control = if was_grounded { 1.0 } else { p.air_control };
        for (velocity, wish) in ps.velocity.iter_mut().zip(wish) {
            *velocity += (wish - *velocity) * control;
        }

        if was_grounded {
            ps.velocity[2] = if cmd.buttons & buttons::JUMP != 0 {
                p.jump_speed
            } else {
                0.0
            };
        }
        ps.velocity[2] -= p.gravity * dt;

        let mut grounded = false;
        for axis in [0usize, 1, 2] {
            let mut end = ps.origin;
            end[axis] += ps.velocity[axis] * dt;
            let hit = env.world.trace(GroundTraceInput {
                start: ps.origin,
                end,
                mins: p.hull.mins,
                maxs: p.hull.maxs,
                tracemask: p.hull.tracemask,
            });
            if hit.allsolid != 0 {
                ps.velocity[axis] = 0.0;
                continue;
            }
            ps.origin = hit.endpos;
            if hit.fraction < 1.0 {
                if axis == 2 && ps.velocity[2] < 0.0 {
                    grounded = true;
                }
                ps.velocity[axis] = 0.0;
            }
        }

        ps.ground_entity_num = if grounded {
            i32::from(ENTITYNUM_WORLD)
        } else {
            i32::from(ENTITYNUM_NONE)
        };

        // Borrow IW4's animation vocabulary for the locomotion hint.
        let anim_movetype = footsteps_anim_move_type(ps, cmd.forwardmove, cmd.rightmove, grounded);
        MovementFacts {
            walking: i32::from(grounded),
            hull: p.hull,
            anim_movetype,
            stance_event: None,
            reset_torso: false,
            jump_animations: [None; 4],
            force_movement_anim: false,
            landing_animation: !was_grounded && grounded,
        }
    }
}
