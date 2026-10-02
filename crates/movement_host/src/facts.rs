use movement_iw4::JumpAnimation;

/// The collision hull and trace mask a provider moves with. The sim links
/// the player into the world area with it after the move.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hull {
    pub mins: [f32; 3],
    pub maxs: [f32; 3],
    pub tracemask: u32,
}

/// What the rest of the tick reads back from a move. These are the values
/// `step.rs` destructures after `pmove` today, formalized.
///
/// The animation fields are IW4's own vocabulary: the soldier's animation
/// script only understands those movement types. A foreign provider has to
/// express what it is doing in them until a retarget layer exists.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovementFacts {
    /// `pml.walking`: standing on something walkable.
    pub walking: i32,
    pub hull: Hull,
    pub anim_movetype: Option<u8>,
    pub stance_event: Option<u8>,
    pub reset_torso: bool,
    pub jump_animations: [Option<(JumpAnimation, bool)>; 4],
    pub force_movement_anim: bool,
    pub landing_animation: bool,
}
