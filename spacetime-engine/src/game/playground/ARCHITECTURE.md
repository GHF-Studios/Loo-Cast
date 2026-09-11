# Playground contract

- **Modding** is deeper than gameplay: static Vapor mods first; runtime systems/components/scripts later.
- **Actions** are executable mechanics independent of items.
- **Items** are persistent player-facing objects that bind/use actions.
- **Item presentation** is separate from item semantics and reusable across HUD/inventory/creative UI.
- **Hotbar** is authoritative gameplay state: 9 unstackable slots + selected slot.
- **Creative catalog** owns no items; it is an infinite, paginated view of registered item definitions.
- **Creative UI hotbar** and **HUD hotbar** are independent views of the same Hotbar state.
- Creative UI is virtualized: fixed visible slots only; item count does not scale UI entity count.
- UI input never leaks into gameplay input.
- Future runtime registries resolve namespaced logical IDs to Bevy runtime IDs/handles where needed.

Human Note: This document should be temporary, I do not like it being here and what it implies! But, we need it so we have sooome kind of roadmap for a day's work.