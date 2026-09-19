//! Identity/manifestation inspection contributed by the test game.

use super::*;

pub(super) fn collect_identity_inspection(
    focus: Res<DeveloperFocus>,
    manifestations: Query<&UsfManifestationOf>,
    semantic_entities: Query<&UsfManifestations>,
    authorities: Query<(), With<UsfManifestationAuthority>>,
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
                InspectValue::text("USF manifestation"),
            ))
            .field(InspectField::new(
                "Spatial authority",
                InspectValue::Bool(authorities.contains(target.spatial_entity)),
            ));
    }

    if let Ok(all) = semantic_entities.get(target.semantic_entity) {
        section = section.field(InspectField::new(
            "Manifestations",
            InspectValue::Integer(all.len() as i64),
        ));
    }

    if let Ok(relation) = manifestations.get(target.spatial_entity) {
        debug_assert_eq!(relation.0, target.semantic_entity);
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
