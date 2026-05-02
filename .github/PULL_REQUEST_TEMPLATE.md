> Use this default template for non-phase PRs.  
> For phase-linked PRs, use `.github/PULL_REQUEST_TEMPLATE/phase_work.md`.

## Description

Brief description of changes.

For unmanaged maintenance, include:

- scope:
- why this does not need phase tracking:
- validation evidence:
- rollback note, if useful:

## Workflow Metadata

Workflow mode (select one; replace `( )` with `(x)`):

- ( ) unmanaged-maintenance
- ( ) process-policy
- ( ) other-non-phase

Change kind (select one; replace `( )` with `(x)`):

- ( ) bug-fix
- ( ) feature-change
- ( ) documentation
- ( ) workflow-metadata

Apply matching live GitHub metadata before review:

- milestone when applicable
- label: `contract:none`, `contract:non-breaking`, or `contract:breaking`
- label: `phase:N` only when the PR is phase-linked

## Validation

- [ ] Tests added/updated or not needed (reason provided)
- [ ] `cargo xtask audit` passing or not run (reason provided)
- [ ] Documentation updated or not needed (reason provided)

## Associated Issues

List only issues this PR actually works on:

- [ ] #123 (issue remains open after merge)
- [x] #124 (issue is fully resolved and should be closed)

If an issue is context-only and not worked here, mention it in PR comments instead of this list.
