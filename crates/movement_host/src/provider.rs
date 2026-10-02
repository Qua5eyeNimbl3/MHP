use movement_iw4::{CollisionBackend, MoveBounds};
use playerstate_iw4::{PlayerState, UserCmd};

use crate::facts::MovementFacts;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProfileId(pub u16);

/// The existing `pmove`. It stays on its own path in `step.rs`.
pub const PROFILE_IW4: ProfileId = ProfileId(0);

/// `pm_type` for an ordinary, unlinked, living player. `playerstate_iw4`
/// names the other types (linked, spectator, intermission, last stand,
/// dead) but not this one.
pub const PM_TYPE_NORMAL: i32 = 0;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TickRate {
    /// Steps by whatever `msec` the sim passes, as IW4 does.
    Variable,
    /// A game with a native fixed rate (Minecraft 20, Mario 64 30).
    /// The host would accumulate and sub-step; it must stay deterministic.
    Fixed { hz: u32 },
}

/// Provider-private state that has no slot in `PlayerState`.
///
/// `PlayerState` is documented as one-to-one retail data, so a wall-run
/// timer or momentum meter cannot live there. This rides beside it, is
/// `Copy`, and must be carried in snapshots or prediction diverges.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProviderBlob {
    bytes: [u8; Self::SIZE],
}

impl Default for ProviderBlob {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl ProviderBlob {
    pub const SIZE: usize = 64;
    /// All zero: a player's state before a provider has moved them.
    pub const EMPTY: Self = Self {
        bytes: [0; Self::SIZE],
    };
    /// How many `f32` slots the blob holds.
    pub const SLOTS: usize = Self::SIZE / 4;

    /// The `f32` in `slot`, or `None` past [`Self::SLOTS`].
    #[must_use]
    pub fn f32_at(&self, slot: usize) -> Option<f32> {
        let at = slot.checked_mul(4)?;
        let bytes = self.bytes.get(at..at + 4)?;
        Some(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Stores `value` in `slot`; `false` (and nothing written) past
    /// [`Self::SLOTS`].
    pub fn set_f32(&mut self, slot: usize, value: f32) -> bool {
        let Some(at) = slot.checked_mul(4) else {
            return false;
        };
        let Some(bytes) = self.bytes.get_mut(at..at + 4) else {
            return false;
        };
        bytes.copy_from_slice(&value.to_le_bytes());
        true
    }
}

/// One player's mutable movement state.
pub struct Mover<'a> {
    pub ps: &'a mut PlayerState,
    pub blob: &'a mut ProviderBlob,
}

/// What a provider may know about the world and the tick.
///
/// `world` is the sim's own collision for this player: map geometry, or
/// block collision while a Minecraft world is active, plus brush models
/// and other players' bodies. In a Minecraft world it ignores the trace
/// mask and reports every block as a stone surface, so a provider cannot
/// yet tell ice, water or a ladder from stone there.
///
/// The weapon scales exist because IW4 couples movement to the held gun.
/// They are the held weapon's own `moveSpeedScale` and
/// `adsMoveSpeedScale`; how far the player is aimed in is
/// `ps.f_weapon_pos_frac`. A provider that ignores them is legal; one that
/// honours them keeps guns feeling like guns.
pub struct StepEnv<'a> {
    pub world: &'a dyn CollisionBackend,
    pub level_time: i32,
    pub weapon_move_scale: f32,
    pub weapon_ads_move_scale: f32,
}

pub trait MovementProvider: Sync {
    fn id(&self) -> ProfileId;
    /// The name a host setting selects it by, such as `walker`.
    fn name(&self) -> &'static str;
    fn tick_rate(&self) -> TickRate;
    fn bounds(&self, ps: &PlayerState) -> MoveBounds;

    /// Whether this provider moves the player in this state. By default
    /// only an ordinary living player: spectating, linked (vehicles,
    /// turrets), last stand, intermission and death stay with IW4's
    /// `pmove`, which already knows them.
    fn handles(&self, ps: &PlayerState) -> bool {
        ps.pm_type == PM_TYPE_NORMAL
    }

    /// One move. Takes the usercmd mutably because `pmove` does too (it
    /// clears stance buttons on ladders) and `step.rs` reads the command
    /// again after the move, for weapons and the next tick's old buttons.
    fn step(&self, mover: &mut Mover<'_>, cmd: &mut UserCmd, env: &StepEnv<'_>) -> MovementFacts;
}
