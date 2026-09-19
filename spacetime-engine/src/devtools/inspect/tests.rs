use std::any::Any;

use super::*;

#[derive(crate::Inspect)]
#[inspect(label = "Derived example")]
struct DerivedExample {
    #[inspect(
        direct,
        label = "Opacity",
        role = "opacity",
        widget = "tests.opacity",
        speed = 0.01,
        range = 0.0..=1.0,
        slider
    )]
    opacity: f32,
    #[inspect(
        validated,
        unit = "K",
        hint = "Validation belongs to the domain adapter."
    )]
    temperature_kelvin: f32,
    #[inspect(skip)]
    cache: u32,
}

struct MutabilityProbe {
    direct: usize,
    read_only: usize,
}

impl InspectFieldVisitorMut for MutabilityProbe {
    fn read_only(&mut self, _: &'static InspectFieldMetadata, _: &dyn Any) {
        self.read_only += 1;
    }

    fn direct(&mut self, _: &'static InspectFieldMetadata, value: &mut dyn Any) {
        self.direct += 1;
        *value
            .downcast_mut::<f32>()
            .expect("the direct test field is f32") = 0.75;
    }
}

#[test]
fn derive_describes_fields_without_leaking_non_direct_mutability() {
    let metadata = DerivedExample::inspect_type_metadata();
    assert_eq!(metadata.label, "Derived example");
    assert_eq!(metadata.fields.len(), 2);

    let opacity = metadata.fields[0];
    assert_eq!(opacity.id, InspectFieldId("opacity"));
    assert_eq!(opacity.label, "Opacity");
    assert_eq!(opacity.role, Some("opacity"));
    assert_eq!(opacity.widget, Some(InspectWidgetId("tests.opacity")));
    assert_eq!(opacity.access, InspectAccess::Direct);
    assert!(opacity.number_input.expect("numeric metadata").slider);

    let temperature = metadata.fields[1];
    assert_eq!(temperature.access, InspectAccess::Validated);
    assert_eq!(temperature.unit, Some(InspectUnit::KELVIN));

    let mut value = DerivedExample {
        opacity: 0.25,
        temperature_kelvin: 350.0,
        cache: 7,
    };
    let mut probe = MutabilityProbe {
        direct: 0,
        read_only: 0,
    };
    value.visit_inspect_fields_mut(&mut probe);

    assert_eq!(probe.direct, 1);
    assert_eq!(probe.read_only, 1);
    assert_eq!(value.opacity, 0.75);
    assert_eq!(value.temperature_kelvin, 350.0);
    assert_eq!(value.cache, 7);
}
}
