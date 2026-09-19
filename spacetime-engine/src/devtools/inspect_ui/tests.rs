use super::*;

#[test]
fn quantities_use_compact_engineering_notation() {
    assert_eq!(
        format_quantity(
            0.000_012_3,
            "m²/s",
            InspectNumberFormat::significant_digits(3),
        ),
        "12.3 × 10⁻⁶ m²/s",
    );
}
