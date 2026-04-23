#![allow(clippy::default_constructed_unit_structs)]

use crate::bevy::prelude::{Reflect, ReflectResource, Resource};
use core_mod_macros::{scale_factor_exponent_dynamic_match, scale_type_generic_match};
use std::fmt::Debug;
use std::hash::Hash;

#[macro_export]
macro_rules! define_scale {
    (
        $name:ident,
        $exp:literal,
        $label:literal,
        up = $up:ident,
        down = $down:ident
    ) => {
        #[derive(Clone, Copy, Default, ::bevy::prelude::Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::usf::scale::ConstScale for $name {
            type Up = $up;
            type Down = $down;
            const SCALE_FACTOR_EXPONENT: i8 = $exp;
            const NAME: &'static str = $label;
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", Self::NAME)
            }
        }

        impl $crate::usf::scale::DynScale for $name {
            fn name(&self) -> &'static str {
                <Self as $crate::usf::scale::ConstScale>::NAME
            }

            fn scale_factor_exponent(&self) -> i8 {
                <Self as $crate::usf::scale::ConstScale>::SCALE_FACTOR_EXPONENT
            }

            fn scale_factor(&self) -> f64 {
                <Self as $crate::usf::scale::ConstScale>::scale_factor()
            }

            fn up(&self) -> Option<$crate::usf::scale::Scale> {
                match stringify!($up) {
                    "NoHigherScale" => None,
                    _ => Some($crate::usf::scale::Scale::$up),
                }
            }

            fn down(&self) -> Option<$crate::usf::scale::Scale> {
                match stringify!($down) {
                    "NoLowerScale" => None,
                    _ => Some($crate::usf::scale::Scale::$down),
                }
            }
        }
    };

    (
        $name:ident,
        $exp:literal,
        $label:literal,
        up = $up:ident
    ) => {
        #[derive(Clone, Copy, Default, ::bevy::prelude::Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::usf::scale::ConstScale for $name {
            type Up = $up;
            type Down = NoLowerScale;
            const SCALE_FACTOR_EXPONENT: i8 = $exp;
            const NAME: &'static str = $label;
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", Self::NAME)
            }
        }

        impl $crate::usf::scale::DynScale for $name {
            fn name(&self) -> &'static str {
                <Self as $crate::usf::scale::ConstScale>::NAME
            }

            fn scale_factor_exponent(&self) -> i8 {
                <Self as $crate::usf::scale::ConstScale>::SCALE_FACTOR_EXPONENT
            }

            fn scale_factor(&self) -> f64 {
                <Self as $crate::usf::scale::ConstScale>::scale_factor()
            }

            fn up(&self) -> Option<$crate::usf::scale::Scale> {
                match stringify!($up) {
                    "NoHigherScale" => None,
                    _ => Some($crate::usf::scale::Scale::$up),
                }
            }

            fn down(&self) -> Option<$crate::usf::scale::Scale> {
                None
            }
        }
    };

    (
        $name:ident,
        $exp:literal,
        $label:literal,
        down = $down:ident
    ) => {
        #[derive(Clone, Copy, Default, ::bevy::prelude::Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::usf::scale::ConstScale for $name {
            type Up = NoHigherScale;
            type Down = $down;
            const SCALE_FACTOR_EXPONENT: i8 = $exp;
            const NAME: &'static str = $label;
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", Self::NAME)
            }
        }

        impl $crate::usf::scale::DynScale for $name {
            fn name(&self) -> &'static str {
                <Self as $crate::usf::scale::ConstScale>::NAME
            }

            fn scale_factor_exponent(&self) -> i8 {
                <Self as $crate::usf::scale::ConstScale>::SCALE_FACTOR_EXPONENT
            }

            fn scale_factor(&self) -> f64 {
                <Self as $crate::usf::scale::ConstScale>::scale_factor()
            }

            fn up(&self) -> Option<$crate::usf::scale::Scale> {
                None
            }

            fn down(&self) -> Option<$crate::usf::scale::Scale> {
                match stringify!($down) {
                    "NoLowerScale" => None,
                    _ => Some($crate::usf::scale::Scale::$down),
                }
            }
        }
    };
    (
        $name:ident,
        $exp:literal,
        $label:literal
    ) => {
        #[derive(Clone, Copy, Default, ::bevy::prelude::Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
        pub struct $name;

        impl $crate::usf::scale::ConstScale for $name {
            type Up = $name;
            type Down = $name;
            const SCALE_FACTOR_EXPONENT: i8 = $exp;
            const NAME: &'static str = $label;
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", Self::NAME)
            }
        }

        impl $crate::usf::scale::DynScale for $name {
            fn name(&self) -> &'static str {
                <Self as $crate::usf::scale::ConstScale>::NAME
            }

            fn scale_factor_exponent(&self) -> i8 {
                <Self as $crate::usf::scale::ConstScale>::SCALE_FACTOR_EXPONENT
            }

            fn scale_factor(&self) -> f64 {
                <Self as $crate::usf::scale::ConstScale>::scale_factor()
            }

            fn up(&self) -> Option<$crate::usf::scale::Scale> {
                None
            }

            fn down(&self) -> Option<$crate::usf::scale::Scale> {
                None
            }
        }
    };
}

#[macro_export]
macro_rules! scale_index_generic_match {
    (
        $scale_index_expr:expr,
        $code:block
    ) => {
        match ($scale_index_expr.index_from_top()) + 1 {
            1 => {
                const __SCALE__: usize = 1;
                $code
            }
            2 => {
                const __SCALE__: usize = 2;
                $code
            }
            3 => {
                const __SCALE__: usize = 3;
                $code
            }
            4 => {
                const __SCALE__: usize = 4;
                $code
            }
            5 => {
                const __SCALE__: usize = 5;
                $code
            }
            6 => {
                const __SCALE__: usize = 6;
                $code
            }
            7 => {
                const __SCALE__: usize = 7;
                $code
            }
            8 => {
                const __SCALE__: usize = 8;
                $code
            }
            9 => {
                const __SCALE__: usize = 9;
                $code
            }
            10 => {
                const __SCALE__: usize = 10;
                $code
            }
            11 => {
                const __SCALE__: usize = 11;
                $code
            }
            12 => {
                const __SCALE__: usize = 12;
                $code
            }
            13 => {
                const __SCALE__: usize = 13;
                $code
            }
            14 => {
                const __SCALE__: usize = 14;
                $code
            }
            15 => {
                const __SCALE__: usize = 15;
                $code
            }
            16 => {
                const __SCALE__: usize = 16;
                $code
            }
            17 => {
                const __SCALE__: usize = 17;
                $code
            }
            18 => {
                const __SCALE__: usize = 18;
                $code
            }
            19 => {
                const __SCALE__: usize = 19;
                $code
            }
            20 => {
                const __SCALE__: usize = 20;
                $code
            }
            21 => {
                const __SCALE__: usize = 21;
                $code
            }
            22 => {
                const __SCALE__: usize = 22;
                $code
            }
            23 => {
                const __SCALE__: usize = 23;
                $code
            }
            24 => {
                const __SCALE__: usize = 24;
                $code
            }
            25 => {
                const __SCALE__: usize = 25;
                $code
            }
            26 => {
                const __SCALE__: usize = 26;
                $code
            }
            27 => {
                const __SCALE__: usize = 27;
                $code
            }
            28 => {
                const __SCALE__: usize = 28;
                $code
            }
            29 => {
                const __SCALE__: usize = 29;
                $code
            }
            30 => {
                const __SCALE__: usize = 30;
                $code
            }
            31 => {
                const __SCALE__: usize = 31;
                $code
            }
            32 => {
                const __SCALE__: usize = 32;
                $code
            }
            33 => {
                const __SCALE__: usize = 33;
                $code
            }
            34 => {
                const __SCALE__: usize = 34;
                $code
            }
            35 => {
                const __SCALE__: usize = 35;
                $code
            }
            36 => {
                const __SCALE__: usize = 36;
                $code
            }
            37 => {
                const __SCALE__: usize = 37;
                $code
            }
            38 => {
                const __SCALE__: usize = 38;
                $code
            }
            39 => {
                const __SCALE__: usize = 39;
                $code
            }
            40 => {
                const __SCALE__: usize = 40;
                $code
            }
            41 => {
                const __SCALE__: usize = 41;
                $code
            }
            42 => {
                const __SCALE__: usize = 42;
                $code
            }
            43 => {
                const __SCALE__: usize = 43;
                $code
            }
            44 => {
                const __SCALE__: usize = 44;
                $code
            }
            45 => {
                const __SCALE__: usize = 45;
                $code
            }
            46 => {
                const __SCALE__: usize = 46;
                $code
            }
            47 => {
                const __SCALE__: usize = 47;
                $code
            }
            48 => {
                const __SCALE__: usize = 48;
                $code
            }
            49 => {
                const __SCALE__: usize = 49;
                $code
            }
            50 => {
                const __SCALE__: usize = 50;
                $code
            }
            51 => {
                const __SCALE__: usize = 51;
                $code
            }
            52 => {
                const __SCALE__: usize = 52;
                $code
            }
            53 => {
                const __SCALE__: usize = 53;
                $code
            }
            54 => {
                const __SCALE__: usize = 54;
                $code
            }
            55 => {
                const __SCALE__: usize = 55;
                $code
            }
            56 => {
                const __SCALE__: usize = 56;
                $code
            }
            57 => {
                const __SCALE__: usize = 57;
                $code
            }
            58 => {
                const __SCALE__: usize = 58;
                $code
            }
            59 => {
                const __SCALE__: usize = 59;
                $code
            }
            60 => {
                const __SCALE__: usize = 60;
                $code
            }
            61 => {
                const __SCALE__: usize = 61;
                $code
            }
            62 => {
                const __SCALE__: usize = 62;
                $code
            }
            63 => {
                const __SCALE__: usize = 63;
                $code
            }
            64 => {
                const __SCALE__: usize = 64;
                $code
            }
            65 => {
                const __SCALE__: usize = 65;
                $code
            }
            66 => {
                const __SCALE__: usize = 66;
                $code
            }
            67 => {
                const __SCALE__: usize = 67;
                $code
            }
            68 => {
                const __SCALE__: usize = 68;
                $code
            }
            69 => {
                const __SCALE__: usize = 69;
                $code
            }
            70 => {
                const __SCALE__: usize = 70;
                $code
            }
            71 => {
                const __SCALE__: usize = 71;
                $code
            }
            _ => unreachable!(),
        }
    };
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct CurrentViewScale {
    scale: u32,
}

pub trait DynScale: Send + Sync + Debug {
    fn name(&self) -> &'static str;
    /// Scale factor exponent, mapped from -35 - 35 to 0 - 70 range where 0 == raw scale factor exponent of 35 (0.00001 quectoMeter)
    fn index_from_top(&self) -> u8 {
        (self.scale_factor_exponent() + 35) as u8
    }
    /// Scale factor exponent, mapped from -35 - 35 to 70 - 0 range where 0 == raw scale factor exponent of -35 (100000 quettaMeter)
    fn index_from_bottom(&self) -> u8 {
        70 - (self.scale_factor_exponent() + 35) as u8
    }
    fn scale_factor_exponent(&self) -> i8;
    fn scale_factor(&self) -> f64;
    fn up(&self) -> Option<Scale>;
    fn down(&self) -> Option<Scale>;
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum Scale {
    #[default]
    ScaleQuettaMeter100000 = 35_i8,
    ScaleQuettaMeter10000 = 34_i8,
    ScaleQuettaMeter1000 = 33_i8,
    ScaleQuettaMeter100 = 32_i8,
    ScaleQuettaMeter10 = 31_i8,
    ScaleQuettaMeter1 = 30_i8,
    ScaleRonnaMeter100 = 29_i8,
    ScaleRonnaMeter10 = 28_i8,
    ScaleRonnaMeter1 = 27_i8,
    ScaleYottaMeter100 = 26_i8,
    ScaleYottaMeter10 = 25_i8,
    ScaleYottaMeter1 = 24_i8,
    ScaleZettaMeter100 = 23_i8,
    ScaleZettaMeter10 = 22_i8,
    ScaleZettaMeter1 = 21_i8,
    ScaleExaMeter100 = 20_i8,
    ScaleExaMeter10 = 19_i8,
    ScaleExaMeter1 = 18_i8,
    ScalePetaMeter100 = 17_i8,
    ScalePetaMeter10 = 16_i8,
    ScalePetaMeter1 = 15_i8,
    ScaleTeraMeter100 = 14_i8,
    ScaleTeraMeter10 = 13_i8,
    ScaleTeraMeter1 = 12_i8,
    ScaleGigaMeter100 = 11_i8,
    ScaleGigaMeter10 = 10_i8,
    ScaleGigaMeter1 = 9_i8,
    ScaleMegaMeter100 = 8_i8,
    ScaleMegaMeter10 = 7_i8,
    ScaleMegaMeter1 = 6_i8,
    ScaleKiloMeter100 = 5_i8,
    ScaleKiloMeter10 = 4_i8,
    ScaleKiloMeter1 = 3_i8,
    ScaleMeter100 = 2_i8,
    ScaleMeter10 = 1_i8,
    ScaleMeter1 = 0_i8,
    ScaleMilliMeter100 = -1_i8,
    ScaleMilliMeter10 = -2_i8,
    ScaleMilliMeter1 = -3_i8,
    ScaleMicroMeter100 = -4_i8,
    ScaleMicroMeter10 = -5_i8,
    ScaleMicroMeter1 = -6_i8,
    ScaleNanoMeter100 = -7_i8,
    ScaleNanoMeter10 = -8_i8,
    ScaleNanoMeter1 = -9_i8,
    ScalePicoMeter100 = -10_i8,
    ScalePicoMeter10 = -11_i8,
    ScalePicoMeter1 = -12_i8,
    ScaleFemtoMeter100 = -13_i8,
    ScaleFemtoMeter10 = -14_i8,
    ScaleFemtoMeter1 = -15_i8,
    ScaleAttoMeter100 = -16_i8,
    ScaleAttoMeter10 = -17_i8,
    ScaleAttoMeter1 = -18_i8,
    ScaleZeptoMeter100 = -19_i8,
    ScaleZeptoMeter10 = -20_i8,
    ScaleZeptoMeter1 = -21_i8,
    ScaleYoctoMeter100 = -22_i8,
    ScaleYoctoMeter10 = -23_i8,
    ScaleYoctoMeter1 = -24_i8,
    ScaleRontoMeter100 = -25_i8,
    ScaleRontoMeter10 = -26_i8,
    ScaleRontoMeter1 = -27_i8,
    ScaleQuectoMeter100 = -28_i8,
    ScaleQuectoMeter10 = -29_i8,
    ScaleQuectoMeter1 = -30_i8,
    ScaleQuectoMeter01 = -31_i8,
    ScaleQuectoMeter001 = -32_i8,
    ScaleQuectoMeter0001 = -33_i8,
    ScaleQuectoMeter00001 = -34_i8,
    ScaleQuectoMeter000001 = -35_i8,
}
impl DynScale for Scale {
    #[allow(unused_braces)]
    fn name(&self) -> &'static str {
        scale_type_generic_match!(*self, { <__SCALE__ as ConstScale>::NAME })
    }
    fn scale_factor_exponent(&self) -> i8 {
        *self as i8
    }
    #[allow(unused_braces)]
    fn scale_factor(&self) -> f64 {
        scale_type_generic_match!(*self, { <__SCALE__ as ConstScale>::scale_factor() })
    }
    #[allow(unused_braces)]
    fn up(&self) -> Option<Scale> {
        scale_type_generic_match!(
            self,
            { Scale::from_scale_factor_exponent(<__SCALE__ as ConstScale>::Up::SCALE_FACTOR_EXPONENT) },
            ScaleQuettaMeter100000 => { None },
        )
    }
    #[allow(unused_braces)]
    fn down(&self) -> Option<Scale> {
        scale_type_generic_match!(
            self,
            { Scale::from_scale_factor_exponent(<__SCALE__ as ConstScale>::Down::SCALE_FACTOR_EXPONENT) },
            ScaleQuectoMeter000001 => { None },
        )
    }
}
impl std::fmt::Debug for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
impl std::fmt::Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.scale_factor_exponent())
    }
}
impl Scale {
    pub const MAX: Scale = Scale::ScaleQuettaMeter100000;
    pub const MID: Scale = Scale::ScaleMeter1;
    pub const MIN: Scale = Scale::ScaleQuectoMeter000001;
    pub const SCALE_LEVEL_COUNT: u8 = 71;

    /// So the max difference in scale can be 3 orders of magnitude
    pub const MAX_DIFF_SCALE_EXP: i8 = 3;

    /// Canonical render depth contract for all 71 scales.
    /// Adjacent levels are separated by >= 1000 units and the full span is ~100k.
    pub const CANONICAL_Z_BASE: f32 = 500.0;
    pub const CANONICAL_Z_SPACING: f32 = 1_420.0;
    pub const CANONICAL_Z_MAX: f32 = Self::CANONICAL_Z_BASE + ((Self::SCALE_LEVEL_COUNT - 1) as f32 * Self::CANONICAL_Z_SPACING);
    pub const CANONICAL_Z_SPAN: f32 = Self::CANONICAL_Z_MAX - Self::CANONICAL_Z_BASE;
    pub const CANONICAL_CAMERA_Z: f32 = Self::CANONICAL_Z_MAX + Self::CANONICAL_Z_SPACING;
    pub const CANONICAL_CAMERA_NEAR: f32 = -120_000.0;
    pub const CANONICAL_CAMERA_FAR: f32 = 120_000.0;

    /// Get the index from the top (0..=70)
    pub fn index_from_top(&self) -> u8 {
        let scale_factor_exponent = self.scale_factor_exponent();
        assert!((-35_i8..=35_i8).contains(&scale_factor_exponent));
        (35_i8 - scale_factor_exponent) as u8
    }

    /// Get the index from the bottom (70..=0)
    pub fn index_from_bottom(&self) -> u8 {
        let scale_factor_exponent = self.scale_factor_exponent();
        assert!((-35_i8..=35_i8).contains(&scale_factor_exponent));
        (scale_factor_exponent + 35_i8) as u8
    }

    /// Create a Scale from an index from the top (0..=70)
    pub fn from_index_from_top(index_from_top: u8) -> Option<Self> {
        assert!(index_from_top <= 70);
        let scale_factor_exponent = 35_i8 - (index_from_top as i8);
        Self::from_scale_factor_exponent(scale_factor_exponent)
    }

    /// Create a Scale from an index from the bottom (70..=0)
    pub fn from_index_from_bottom(index_from_bottom: u8) -> Option<Self> {
        assert!(index_from_bottom <= 70);
        let scale_factor_exponent = -35_i8 + (index_from_bottom as i8);
        Self::from_scale_factor_exponent(scale_factor_exponent)
    }

    pub fn from_scale_factor_exponent(scale_factor_exponent: i8) -> Option<Self> {
        scale_factor_exponent_dynamic_match!(scale_factor_exponent, Some(__SCALE__), None)
    }

    /// Canonical layer index for render proxy composition.
    #[inline]
    pub fn render_layer_index(self) -> u8 {
        self.index_from_top()
    }

    /// Canonical depth placement for this scale.
    #[inline]
    pub fn canonical_z(self) -> f32 {
        Self::CANONICAL_Z_BASE + (self.index_from_top() as f32 * Self::CANONICAL_Z_SPACING)
    }

    pub fn zoom_in(&mut self) {
        let self_scale_factor_exp = self.scale_factor_exponent();
        if self_scale_factor_exp > Self::MIN.scale_factor_exponent() {
            *self = self.down().unwrap();
        }
    }

    pub fn zoomed_in(mut self) -> Self {
        self.zoom_in();
        self
    }

    pub fn zoom_out(&mut self) {
        let self_scale_factor_exp = self.scale_factor_exponent();
        if self_scale_factor_exp < Self::MAX.scale_factor_exponent() {
            *self = self.up().unwrap();
        }
    }

    pub fn zoomed_out(mut self) -> Self {
        self.zoom_out();
        self
    }
}

pub trait ConstScale: 'static + Send + Sync + Clone + Copy + Default + Debug + Reflect + PartialOrd + Ord + PartialEq + Eq + Hash {
    type Up: ConstScale;
    type Down: ConstScale;

    const SCALE_FACTOR_EXPONENT: i8;
    const NAME: &'static str;

    fn scale_factor() -> f64 {
        10f64.powi(Self::SCALE_FACTOR_EXPONENT as i32)
    }
}

define_scale!(NoLowerScale, 0, "no_lower_scale");
define_scale!(ScaleQuectoMeter000001, -35, "scale_quecto_meter_000001", up = ScaleQuectoMeter00001);
define_scale!(
    ScaleQuectoMeter00001,
    -34,
    "scale_quecto_meter_00001",
    up = ScaleQuectoMeter0001,
    down = ScaleQuectoMeter000001
);
define_scale!(
    ScaleQuectoMeter0001,
    -33,
    "scale_quecto_meter_0001",
    up = ScaleQuectoMeter001,
    down = ScaleQuectoMeter00001
);
define_scale!(
    ScaleQuectoMeter001,
    -32,
    "scale_quecto_meter_001",
    up = ScaleQuectoMeter01,
    down = ScaleQuectoMeter0001
);
define_scale!(
    ScaleQuectoMeter01,
    -31,
    "scale_quecto_meter_01",
    up = ScaleQuectoMeter1,
    down = ScaleQuectoMeter001
);
define_scale!(
    ScaleQuectoMeter1,
    -30,
    "scale_quecto_meter_1",
    up = ScaleQuectoMeter10,
    down = ScaleQuectoMeter01
);
define_scale!(
    ScaleQuectoMeter10,
    -29,
    "scale_quecto_meter_10",
    up = ScaleQuectoMeter100,
    down = ScaleQuectoMeter1
);
define_scale!(
    ScaleQuectoMeter100,
    -28,
    "scale_quecto_meter_100",
    up = ScaleRontoMeter1,
    down = ScaleQuectoMeter10
);
define_scale!(ScaleRontoMeter1, -27, "scale_ronto_meter_1", up = ScaleRontoMeter10, down = ScaleQuectoMeter100);
define_scale!(ScaleRontoMeter10, -26, "scale_ronto_meter_10", up = ScaleRontoMeter100, down = ScaleRontoMeter1);
define_scale!(
    ScaleRontoMeter100,
    -25,
    "scale_ronto_meter_100",
    up = ScaleYoctoMeter1,
    down = ScaleRontoMeter10
);
define_scale!(ScaleYoctoMeter1, -24, "scale_yocto_meter_1", up = ScaleYoctoMeter10, down = ScaleRontoMeter100);
define_scale!(ScaleYoctoMeter10, -23, "scale_yocto_meter_10", up = ScaleYoctoMeter100, down = ScaleYoctoMeter1);
define_scale!(
    ScaleYoctoMeter100,
    -22,
    "scale_yocto_meter_100",
    up = ScaleZeptoMeter1,
    down = ScaleYoctoMeter10
);
define_scale!(ScaleZeptoMeter1, -21, "scale_zepto_meter_1", up = ScaleZeptoMeter10, down = ScaleYoctoMeter100);
define_scale!(ScaleZeptoMeter10, -20, "scale_zepto_meter_10", up = ScaleZeptoMeter100, down = ScaleZeptoMeter1);
define_scale!(ScaleZeptoMeter100, -19, "scale_zepto_meter_100", up = ScaleAttoMeter1, down = ScaleZeptoMeter10);
define_scale!(ScaleAttoMeter1, -18, "scale_atto_meter_1", up = ScaleAttoMeter10, down = ScaleZeptoMeter100);
define_scale!(ScaleAttoMeter10, -17, "scale_atto_meter_10", up = ScaleAttoMeter100, down = ScaleAttoMeter1);
define_scale!(ScaleAttoMeter100, -16, "scale_atto_meter_100", up = ScaleFemtoMeter1, down = ScaleAttoMeter10);
define_scale!(ScaleFemtoMeter1, -15, "scale_femto_meter_1", up = ScaleFemtoMeter10, down = ScaleAttoMeter100);
define_scale!(ScaleFemtoMeter10, -14, "scale_femto_meter_10", up = ScaleFemtoMeter100, down = ScaleFemtoMeter1);
define_scale!(ScaleFemtoMeter100, -13, "scale_femto_meter_100", up = ScalePicoMeter1, down = ScaleFemtoMeter10);
define_scale!(ScalePicoMeter1, -12, "scale_pico_meter_1", up = ScalePicoMeter10, down = ScaleFemtoMeter100);
define_scale!(ScalePicoMeter10, -11, "scale_pico_meter_10", up = ScalePicoMeter100, down = ScalePicoMeter1);
define_scale!(ScalePicoMeter100, -10, "scale_pico_meter_100", up = ScaleNanoMeter1, down = ScalePicoMeter10);
define_scale!(ScaleNanoMeter1, -9, "scale_nano_meter_1", up = ScaleNanoMeter10, down = ScalePicoMeter100);
define_scale!(ScaleNanoMeter10, -8, "scale_nano_meter_10", up = ScaleNanoMeter100, down = ScaleNanoMeter1);
define_scale!(ScaleNanoMeter100, -7, "scale_nano_meter_100", up = ScaleMicroMeter1, down = ScaleNanoMeter10);
define_scale!(ScaleMicroMeter1, -6, "scale_micro_meter_1", up = ScaleMicroMeter10, down = ScaleNanoMeter100);
define_scale!(ScaleMicroMeter10, -5, "scale_micro_meter_10", up = ScaleMicroMeter100, down = ScaleMicroMeter1);
define_scale!(ScaleMicroMeter100, -4, "scale_micro_meter_100", up = ScaleMilliMeter1, down = ScaleMicroMeter10);
define_scale!(ScaleMilliMeter1, -3, "scale_milli_meter_1", up = ScaleMilliMeter10, down = ScaleMicroMeter100);
define_scale!(ScaleMilliMeter10, -2, "scale_milli_meter_10", up = ScaleMilliMeter100, down = ScaleMilliMeter1);
define_scale!(ScaleMilliMeter100, -1, "scale_milli_meter_100", up = ScaleMeter1, down = ScaleMilliMeter10);
define_scale!(ScaleMeter1, 0, "scale_meter_1", up = ScaleMeter10, down = ScaleMilliMeter100);
define_scale!(ScaleMeter10, 1, "scale_meter_10", up = ScaleMeter100, down = ScaleMeter1);
define_scale!(ScaleMeter100, 2, "scale_meter_100", up = ScaleKiloMeter1, down = ScaleMeter10);
define_scale!(ScaleKiloMeter1, 3, "scale_kilo_meter_1", up = ScaleKiloMeter10, down = ScaleMeter100);
define_scale!(ScaleKiloMeter10, 4, "scale_kilo_meter_10", up = ScaleKiloMeter100, down = ScaleKiloMeter1);
define_scale!(ScaleKiloMeter100, 5, "scale_kilo_meter_100", up = ScaleMegaMeter1, down = ScaleKiloMeter10);
define_scale!(ScaleMegaMeter1, 6, "scale_mega_meter_1", up = ScaleMegaMeter10, down = ScaleKiloMeter100);
define_scale!(ScaleMegaMeter10, 7, "scale_mega_meter_10", up = ScaleMegaMeter100, down = ScaleMegaMeter1);
define_scale!(ScaleMegaMeter100, 8, "scale_mega_meter_100", up = ScaleGigaMeter1, down = ScaleMegaMeter10);
define_scale!(ScaleGigaMeter1, 9, "scale_giga_meter_1", up = ScaleGigaMeter10, down = ScaleMegaMeter100);
define_scale!(ScaleGigaMeter10, 10, "scale_giga_meter_10", up = ScaleGigaMeter100, down = ScaleGigaMeter1);
define_scale!(ScaleGigaMeter100, 11, "scale_giga_meter_100", up = ScaleTeraMeter1, down = ScaleGigaMeter10);
define_scale!(ScaleTeraMeter1, 12, "scale_tera_meter_1", up = ScaleTeraMeter10, down = ScaleGigaMeter100);
define_scale!(ScaleTeraMeter10, 13, "scale_tera_meter_10", up = ScaleTeraMeter100, down = ScaleTeraMeter1);
define_scale!(ScaleTeraMeter100, 14, "scale_tera_meter_100", up = ScalePetaMeter1, down = ScaleTeraMeter10);
define_scale!(ScalePetaMeter1, 15, "scale_peta_meter_1", up = ScalePetaMeter10, down = ScaleTeraMeter100);
define_scale!(ScalePetaMeter10, 16, "scale_peta_meter_10", up = ScalePetaMeter100, down = ScalePetaMeter1);
define_scale!(ScalePetaMeter100, 17, "scale_peta_meter_100", up = ScaleExaMeter1, down = ScalePetaMeter10);
define_scale!(ScaleExaMeter1, 18, "scale_exa_meter_1", up = ScaleExaMeter10, down = ScalePetaMeter100);
define_scale!(ScaleExaMeter10, 19, "scale_exa_meter_10", up = ScaleExaMeter100, down = ScaleExaMeter1);
define_scale!(ScaleExaMeter100, 20, "scale_exa_meter_100", up = ScaleZettaMeter1, down = ScaleExaMeter10);
define_scale!(ScaleZettaMeter1, 21, "scale_zetta_meter_1", up = ScaleZettaMeter10, down = ScaleExaMeter100);
define_scale!(ScaleZettaMeter10, 22, "scale_zetta_meter_10", up = ScaleZettaMeter100, down = ScaleZettaMeter1);
define_scale!(ScaleZettaMeter100, 23, "scale_zetta_meter_100", up = ScaleYottaMeter1, down = ScaleZettaMeter10);
define_scale!(ScaleYottaMeter1, 24, "scale_yotta_meter_1", up = ScaleYottaMeter10, down = ScaleZettaMeter100);
define_scale!(ScaleYottaMeter10, 25, "scale_yotta_meter_10", up = ScaleYottaMeter100, down = ScaleYottaMeter1);
define_scale!(ScaleYottaMeter100, 26, "scale_yotta_meter_100", up = ScaleRonnaMeter1, down = ScaleYottaMeter10);
define_scale!(ScaleRonnaMeter1, 27, "scale_ronna_meter_1", up = ScaleRonnaMeter10, down = ScaleYottaMeter100);
define_scale!(ScaleRonnaMeter10, 28, "scale_ronna_meter_10", up = ScaleRonnaMeter100, down = ScaleRonnaMeter1);
define_scale!(
    ScaleRonnaMeter100,
    29,
    "scale_ronna_meter_100",
    up = ScaleQuettaMeter1,
    down = ScaleRonnaMeter10
);
define_scale!(
    ScaleQuettaMeter1,
    30,
    "scale_quetta_meter_1",
    up = ScaleQuettaMeter10,
    down = ScaleRonnaMeter100
);
define_scale!(
    ScaleQuettaMeter10,
    31,
    "scale_quetta_meter_10",
    up = ScaleQuettaMeter100,
    down = ScaleQuettaMeter1
);
define_scale!(
    ScaleQuettaMeter100,
    32,
    "scale_quetta_meter_100",
    up = ScaleQuettaMeter1000,
    down = ScaleQuettaMeter10
);
define_scale!(
    ScaleQuettaMeter1000,
    33,
    "scale_quetta_meter_1000",
    up = ScaleQuettaMeter10000,
    down = ScaleQuettaMeter100
);
define_scale!(
    ScaleQuettaMeter10000,
    34,
    "scale_quetta_meter_10000",
    up = ScaleQuettaMeter100000,
    down = ScaleQuettaMeter1000
);
define_scale!(ScaleQuettaMeter100000, 35, "scale_quetta_meter_100000", down = ScaleQuettaMeter10000);
define_scale!(NoHigherScale, 0, "no_higher_scale");

#[cfg(test)]
mod tests {
    use super::Scale;

    #[test]
    fn meter_milli_zoom_links_are_consistent() {
        let mut scale = Scale::ScaleMeter1;
        scale.zoom_in();
        assert_eq!(scale, Scale::ScaleMilliMeter100);

        scale.zoom_in();
        assert_eq!(scale, Scale::ScaleMilliMeter10);

        scale.zoom_out();
        assert_eq!(scale, Scale::ScaleMilliMeter100);

        scale.zoom_out();
        assert_eq!(scale, Scale::ScaleMeter1);
    }

    #[test]
    fn canonical_layer_bounds_cover_all_scales() {
        assert_eq!(Scale::MAX.render_layer_index(), 0);
        assert_eq!(Scale::MIN.render_layer_index(), 70);
    }

    #[test]
    fn canonical_depth_spacing_and_span_meet_contract() {
        let coarse = Scale::MAX.canonical_z();
        let next_finer = Scale::MAX.zoomed_in().canonical_z();
        let finest = Scale::MIN.canonical_z();

        let spacing = next_finer - coarse;
        let span = finest - coarse;

        assert!(spacing >= 1000.0, "adjacent spacing was {spacing}");
        assert!((95_000.0..=105_000.0).contains(&span), "span was {span}");
    }
}
