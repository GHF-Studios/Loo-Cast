# Intention Records

Purpose: maintain canonical architecture intent through diagram atlases and aligned implementation summaries.

## Canonical Diagram Atlases

1. `README.puml`
   Repository-level manifest linking active documentation atlases.
2. `usf_records/`
   USF architecture, flow, prototype scope, and math contracts.
3. `scripting_records/`
   Script/runtime contracts and decision locks.
4. `platform_records/`
   Runtime composition and build/runtime flow.

## Companion Implementation Summaries

- `../markdown_summary/README.md`

## Authority Position

1. Owner direction in conversation is authoritative for target intent.
2. Diagram atlases are the canonical durable design surface.
3. Markdown summaries are implementation-oriented and must align to atlas contracts.
4. Temporary planning notes remain in `../temp_stuff/` and are not canonical.

## Maintenance Rules

1. Keep architecture contracts in atlas diagrams first.
2. Update summaries in lockstep with diagram changes.
3. Remove superseded concepts from canonical docs once cut over.
4. Stop and request owner decision whenever implementation and direction diverge.
