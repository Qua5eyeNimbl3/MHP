# Movement profiles

A match can move its players with a movement system other than MW2's own.
The seam is `crates/movement_host`; the sim calls it from the one place that
moves a player, `run_players_system` in `crates/sim/src/step.rs`.

## Choosing one

`IW4L_MOVEMENT` in `.env` (or the environment) names the profile for every
player in a match this machine sets up:

| name | what it is |
|---|---|
| `iw4` (default, or unset) | MW2's `pmove`, on its original path |
| `walker` | a plain walker near MW2's numbers: no stance, mantle, ladder or step-up |

An unknown name refuses the match and lists the known ones.

## What a profile moves

Only an ordinary living player (`pm_type` 0). Spectating, vehicles and
turrets, last stand, intermission and death stay with `pmove`. A player under
`external_motion` (Skate mode) is not moved by either.

## Adding one

Implement `movement_host::MovementProvider` and add it to `PROVIDERS` in
`crates/movement_host/src/registry.rs`. It moves through the sim's own
collision (`StepEnv::world`), so it collides with the map, brush models,
other players and, on the Minecraft map, blocks. Keep it `no_std`, with
`libm` math and no clocks, threads or unseeded randomness: authority,
prediction and replay must agree.

## Not done yet

* Nothing checks across the network that every machine uses the same
  profile. A mismatch mispredicts instead of refusing to connect.
* A provider's private state (`ProviderBlob`) is not in snapshots, so a
  provider that keeps state mispredicts on remote clients. `walker` keeps none.
* Demos do not record the profile; replay one with the setting it was made with.
* Bots plan routes for MW2 movement; under another profile they may stall.
* On the Minecraft map every block traces as stone and the trace mask is
  ignored, so a profile cannot tell ice, water or a ladder from stone.
