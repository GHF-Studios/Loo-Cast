//! Map-wide symbols, references and generated-object budget.

use std::collections::HashSet;

use super::super::{AuthoredMap, MAX_GENERATED_OBJECTS, MapObject};
use super::constraints::validate_color;

/// Borrowed symbol table for one authored map validation pass. The schema owns
/// the strings; validation only checks their uniqueness and references.
struct MapValidation<'a> {
    zones: HashSet<&'a str>,
    materials: HashSet<&'a str>,
    objects: HashSet<&'a str>,
    generated_objects: usize,
}

impl AuthoredMap {
    pub fn validate(&self) -> Result<(), String> {
        let mut validation = MapValidation::new(self)?;
        for object in &self.objects {
            validation.validate_object(object)?;
        }
        Ok(())
    }
}

impl<'a> MapValidation<'a> {
    fn new(map: &'a AuthoredMap) -> Result<Self, String> {
        let mut zones = HashSet::new();
        for zone in &map.zones {
            if zone.id.trim().is_empty() {
                return Err("zone id must not be empty".into());
            }
            if !zones.insert(zone.id.as_str()) {
                return Err(format!("duplicate zone id {:?}", zone.id));
            }
        }

        let mut materials = HashSet::new();
        for material in &map.materials {
            if material.id.trim().is_empty() {
                return Err("material id must not be empty".into());
            }
            if !materials.insert(material.id.as_str()) {
                return Err(format!("duplicate material id {:?}", material.id));
            }
            validate_color(material.color, &format!("material {:?}", material.id))?;
            if !material.metallic.is_finite() || !(0.0..=1.0).contains(&material.metallic) {
                return Err(format!(
                    "material {:?} metallic must be in 0..=1",
                    material.id
                ));
            }
            if !material.roughness.is_finite() || !(0.0..=1.0).contains(&material.roughness) {
                return Err(format!(
                    "material {:?} roughness must be in 0..=1",
                    material.id
                ));
            }
        }

        Ok(Self {
            zones,
            materials,
            objects: HashSet::new(),
            generated_objects: 0,
        })
    }

    fn validate_object(&mut self, object: &'a MapObject) -> Result<(), String> {
        let id = object.id();
        if id.trim().is_empty() {
            return Err("object id must not be empty".into());
        }
        if !self.objects.insert(id) {
            return Err(format!("duplicate object id {id:?}"));
        }

        let zone = object.zone();
        if !zone.is_empty() && !self.zones.contains(zone) {
            return Err(format!("object {id:?} references unknown zone {zone:?}"));
        }
        if let Some(material) = object.material()
            && !self.materials.contains(material)
        {
            return Err(format!(
                "object {id:?} references unknown material {material:?}"
            ));
        }

        object.validate()?;
        self.generated_objects = self
            .generated_objects
            .checked_add(object.generated_count()?)
            .ok_or_else(|| "generated object count overflowed usize".to_string())?;
        if self.generated_objects > MAX_GENERATED_OBJECTS {
            return Err(format!(
                "map generates {} runtime objects; maximum is {MAX_GENERATED_OBJECTS}",
                self.generated_objects
            ));
        }
        Ok(())
    }
}
