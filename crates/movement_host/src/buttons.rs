use playerstate_iw4::buttons;

/// Every button bit MW2 already assigns. A foreign game's extra actions
/// (dash, grapple, wall-jump) can only use the bits left over.
const USED_BUTTON_BITS: u32 = buttons::ATTACK
    | buttons::SPRINT
    | buttons::MELEE_CHARGE
    | buttons::USE
    | buttons::RELOAD
    | buttons::USE_RELOAD
    | buttons::PRONE
    | buttons::CROUCH
    | buttons::JUMP
    | buttons::ADS
    | buttons::STANCE_HELD
    | buttons::BREATH
    | buttons::FRAG
    | buttons::SMOKE
    | buttons::LOCATION_SELECT
    | buttons::LOCATION_CANCEL
    | buttons::THROW
    | buttons::REMOTE_CONTROL
    | buttons::OFFHAND_HOLD_CANCEL;

pub const FREE_BUTTON_BITS: u32 = !USED_BUTTON_BITS;

/// The `n`th unassigned button bit, for a profile's extra actions (dash,
/// grapple, wall-jump). `None` once a profile asks for more than the
/// usercmd can carry; anything past that, or any real analog input beyond
/// the two signed movement bytes, needs a wider usercmd, which is a wire
/// change.
#[must_use]
pub const fn extra_action_bit(n: u32) -> Option<u32> {
    let mut free = FREE_BUTTON_BITS;
    let mut skipped = 0;
    while free != 0 {
        let bit = free & free.wrapping_neg();
        if skipped == n {
            return Some(bit);
        }
        free &= !bit;
        skipped += 1;
    }
    None
}
