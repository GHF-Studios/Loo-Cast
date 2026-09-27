//! Identity/manifestation inspection contributed by the test game.

use super::*;

pub(super) fn collect_identity_inspection(
    focus: Res<DeveloperFocus>,
    runtime_ownership: UsfRuntimeOwnershipQuery,
    split_peers: Query<(Entity, &SpatialSplitPeer)>,
    active_split_peers: Query<(), With<SpatialSplitPeerActive>>,
    mut frame: ResMut<InspectionFrame>,
) {
    let Some(target) = focus.current() else {
        return;
    };

    let identity_label = if target.spatial_entity == target.semantic_entity {
        "Entity"
    } else {
        "Semantic entity"
    };
    let mut section = InspectSection::new(IDENTITY_SECTION, "Identity", 0).field(
        InspectField::new(identity_label, InspectValue::Entity(target.semantic_entity)),
    );

    if let Some(hit) = target.hit {
        section = section.field(InspectField::new(
            "Hit position",
            InspectValue::Vec3(hit.position),
        ));
    }

    if target.spatial_entity != target.semantic_entity {
        section = section
            .field(InspectField::new(
                "Spatial entity",
                InspectValue::Entity(target.spatial_entity),
            ))
            .field(InspectField::new(
                "Relationship",
                InspectValue::text(if split_peers.get(target.spatial_entity).is_ok() {
                    "USF split proxy"
                } else {
                    "USF logical realization"
                }),
            ))
            .field(InspectField::new(
                "Spatial authority",
                InspectValue::Bool(
                    runtime_ownership.authority_realization(target.spatial_entity)
                        == target.spatial_entity
                        && runtime_ownership.semantic_of(target.spatial_entity)
                            == Some(target.semantic_entity),
                ),
            ));
    }

    let runtime_entities = runtime_ownership.runtime_entities_of(target.semantic_entity);
    if !runtime_entities.is_empty() {
        section = section.field(InspectField::new(
            "Runtime representations",
            InspectValue::Integer(runtime_entities.len() as i64),
        ));
    }

    if let Some(semantic) = runtime_ownership.semantic_of(target.spatial_entity) {
        debug_assert_eq!(semantic, target.semantic_entity);
    }

    if let Ok((_, peer)) = split_peers.get(target.spatial_entity) {
        section = section
            .field(InspectField::new("Split role", InspectValue::text("peer")))
            .field(InspectField::new(
                "Split authority",
                InspectValue::Entity(peer.authority),
            ))
            .field(InspectField::new(
                "Split peer active",
                InspectValue::Bool(active_split_peers.contains(target.spatial_entity)),
            ));
    } else if let Some((peer_entity, _)) = split_peers
        .iter()
        .find(|(_, peer)| peer.authority == target.spatial_entity)
    {
        section = section
            .field(InspectField::new(
                "Split peer",
                InspectValue::Entity(peer_entity),
            ))
            .field(InspectField::new(
                "Split active",
                InspectValue::Bool(active_split_peers.contains(peer_entity)),
            ));
    }

    frame.submit(section);
}
