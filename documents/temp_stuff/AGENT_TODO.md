# Agent TODO

Purpose: live, temporary working log for active work only.

Read first: `docs/RhaiAgentHandoff.md`

## Active: USF Border Jitter Stabilization

Status: in progress

Goal: remove the remaining border-crossing camera jump while preserving intentional follow lag.

### Scope

- USF translation pivot crosses a wrap boundary and can introduce follower target discontinuities.
- Camera must keep existing lag offset through wrap/load transitions.

### Checklist

- [x] Removed follower hard snap-to-target fallback that could erase lag.
- [x] Added follower smoothing `dt` clamp to avoid post-load hitch catch-up jumps.
- [ ] Manual validation: hold movement key across border and confirm no jump+glideback.
- [ ] Manual validation: confirm no regressions in normal camera follow feel.

### Runtime Validation Command

- `./build.sh dev; ./run.sh dev`
