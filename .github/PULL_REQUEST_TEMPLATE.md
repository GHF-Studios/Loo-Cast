> Use this default template for non-phase PRs.  
> For phase-linked PRs, use `.github/PULL_REQUEST_TEMPLATE/phase_work.md`.

## Description

Brief description of changes.

For unmanaged maintenance, include:

- scope:
- why this does not need phase tracking:
- validation evidence:
- rollback note, if useful:

## Workflow Mode

Choose one:

- [ ] Unmanaged maintenance
- [ ] Process-policy
- [ ] Other non-phase work

Apply matching live GitHub metadata before review:

- milestone when applicable
- label: `contract:none`, `contract:non-breaking`, or `contract:breaking`
- label: `phase:N` only when the PR is phase-linked

## Change Kind

- [ ] Bug fix
- [ ] Feature/change
- [ ] Documentation
- [ ] Workflow/repository metadata

## Checklist

- [ ] Tests added/updated
- [ ] `cargo xtask audit` passing
- [ ] Documentation updated

## Issue Relationship

- References: #123
- Advances: #123
- Closes: #123
