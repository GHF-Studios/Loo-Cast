//! Flight HUD widgets and placement.

use super::*;

const HUD_TEXT: Color = Color::srgb(0.72, 0.95, 0.88);
const HUD_ACCENT: Color = Color::srgba(0.30, 0.84, 0.88, 0.84);
const HUD_PANEL: Color = Color::srgba(0.01, 0.035, 0.045, 0.68);
const HUD_WARNING: Color = Color::srgb(1.0, 0.72, 0.28);
const WING_CENTER_GAP_PX: f32 = 104.0;
const WING_TOP_OFFSET_PX: f32 = -72.0;
const WING_WIDTH_PX: f32 = 224.0;

#[derive(Component)]
pub(in crate::game::player) struct FlightHudLeft;
#[derive(Component)]
pub(in crate::game::player) struct FlightHudRight;
#[derive(Component)]
pub(in crate::game::player) struct FlightHudAlert;

pub(in crate::game::player) fn spawn_flight_hud(mut commands: Commands) {
    commands.spawn((
        Name::new("Flight HUD Left Wing"),
        FlightHudLeft,
        GlobalZIndex(UiLayer::HUD),
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextColor(HUD_TEXT),
        TextLayout::justify(Justify::Right),
        Node {
            position_type: PositionType::Absolute,
            right: percent(50.0),
            top: percent(50.0),
            margin: UiRect {
                right: px(WING_CENTER_GAP_PX),
                top: px(WING_TOP_OFFSET_PX),
                ..default()
            },
            width: px(WING_WIDTH_PX),
            padding: UiRect::axes(px(9.0), px(6.0)),
            border: UiRect::right(px(2.0)),
            ..default()
        },
        BackgroundColor(HUD_PANEL),
        BorderColor::all(HUD_ACCENT),
    ));

    commands.spawn((
        Name::new("Flight HUD Right Wing"),
        FlightHudRight,
        GlobalZIndex(UiLayer::HUD),
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(15.0),
            ..default()
        },
        TextColor(HUD_TEXT),
        TextLayout::justify(Justify::Left),
        Node {
            position_type: PositionType::Absolute,
            left: percent(50.0),
            top: percent(50.0),
            margin: UiRect {
                left: px(WING_CENTER_GAP_PX),
                top: px(WING_TOP_OFFSET_PX),
                ..default()
            },
            width: px(WING_WIDTH_PX),
            padding: UiRect::axes(px(9.0), px(6.0)),
            border: UiRect::left(px(2.0)),
            ..default()
        },
        BackgroundColor(HUD_PANEL),
        BorderColor::all(HUD_ACCENT),
    ));

    commands.spawn((
        Name::new("Flight HUD Center Alert"),
        FlightHudAlert,
        GlobalZIndex(UiLayer::HUD),
        Text::new(""),
        TextFont {
            font_size: FontSize::Px(14.0),
            ..default()
        },
        TextColor(HUD_WARNING),
        TextLayout::justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            left: percent(50.0),
            top: percent(50.0),
            margin: UiRect {
                top: px(78.0),
                ..default()
            },
            ..default()
        },
        UiTransform::from_translation(Val2::percent(-50.0, 0.0)),
    ));
}
