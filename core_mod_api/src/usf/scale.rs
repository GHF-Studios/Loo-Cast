#![allow(clippy::default_constructed_unit_structs)]

use bevy::prelude::{Reflect, Resource, ReflectResource};
use core_mod_macros::scale_type_generic_match;
use std::fmt::Debug;
use std::hash::Hash;

/// Matches a provided scale to execute code with generic const parameter integer __SCALE__ without actually touching it.
#[macro_export]
macro_rules! scale_index_generic_match {
    (
        $scale_index_expr:expr,
        $code:block
    ) => {
        match ($scale_index_expr.index_from_top()) + 1 {
            1 => { const __SCALE__: usize = 1; $code }
            2 => { const __SCALE__: usize = 2; $code }
            3 => { const __SCALE__: usize = 3; $code }
            4 => { const __SCALE__: usize = 4; $code }
            5 => { const __SCALE__: usize = 5; $code }
            6 => { const __SCALE__: usize = 6; $code }
            7 => { const __SCALE__: usize = 7; $code }
            8 => { const __SCALE__: usize = 8; $code }
            9 => { const __SCALE__: usize = 9; $code }
            10 => { const __SCALE__: usize = 10; $code }
            11 => { const __SCALE__: usize = 11; $code }
            12 => { const __SCALE__: usize = 12; $code }
            13 => { const __SCALE__: usize = 13; $code }
            14 => { const __SCALE__: usize = 14; $code }
            15 => { const __SCALE__: usize = 15; $code }
            16 => { const __SCALE__: usize = 16; $code }
            17 => { const __SCALE__: usize = 17; $code }
            18 => { const __SCALE__: usize = 18; $code }
            19 => { const __SCALE__: usize = 19; $code }
            20 => { const __SCALE__: usize = 20; $code }
            21 => { const __SCALE__: usize = 21; $code }
            22 => { const __SCALE__: usize = 22; $code }
            23 => { const __SCALE__: usize = 23; $code }
            24 => { const __SCALE__: usize = 24; $code }
            25 => { const __SCALE__: usize = 25; $code }
            26 => { const __SCALE__: usize = 26; $code }
            27 => { const __SCALE__: usize = 27; $code }
            28 => { const __SCALE__: usize = 28; $code }
            29 => { const __SCALE__: usize = 29; $code }
            30 => { const __SCALE__: usize = 30; $code }
            31 => { const __SCALE__: usize = 31; $code }
            32 => { const __SCALE__: usize = 32; $code }
            33 => { const __SCALE__: usize = 33; $code }
            34 => { const __SCALE__: usize = 34; $code }
            35 => { const __SCALE__: usize = 35; $code }
            36 => { const __SCALE__: usize = 36; $code }
            37 => { const __SCALE__: usize = 37; $code }
            38 => { const __SCALE__: usize = 38; $code }
            39 => { const __SCALE__: usize = 39; $code }
            40 => { const __SCALE__: usize = 40; $code }
            41 => { const __SCALE__: usize = 41; $code }
            42 => { const __SCALE__: usize = 42; $code }
            43 => { const __SCALE__: usize = 43; $code }
            44 => { const __SCALE__: usize = 44; $code }
            45 => { const __SCALE__: usize = 45; $code }
            46 => { const __SCALE__: usize = 46; $code }
            47 => { const __SCALE__: usize = 47; $code }
            48 => { const __SCALE__: usize = 48; $code }
            49 => { const __SCALE__: usize = 49; $code }
            50 => { const __SCALE__: usize = 50; $code }
            51 => { const __SCALE__: usize = 51; $code }
            52 => { const __SCALE__: usize = 52; $code }
            53 => { const __SCALE__: usize = 53; $code }
            54 => { const __SCALE__: usize = 54; $code }
            55 => { const __SCALE__: usize = 55; $code }
            56 => { const __SCALE__: usize = 56; $code }
            57 => { const __SCALE__: usize = 57; $code }
            58 => { const __SCALE__: usize = 58; $code }
            59 => { const __SCALE__: usize = 59; $code }
            60 => { const __SCALE__: usize = 60; $code }
            61 => { const __SCALE__: usize = 61; $code }
            62 => { const __SCALE__: usize = 62; $code }
            63 => { const __SCALE__: usize = 63; $code }
            64 => { const __SCALE__: usize = 64; $code }
            65 => { const __SCALE__: usize = 65; $code }
            66 => { const __SCALE__: usize = 66; $code }
            67 => { const __SCALE__: usize = 67; $code }
            68 => { const __SCALE__: usize = 68; $code }
            69 => { const __SCALE__: usize = 69; $code }
            70 => { const __SCALE__: usize = 70; $code }
            71 => { const __SCALE__: usize = 71; $code }
            _ => unreachable!(),
        }
    };
    
    (
        $scale_index_expr:expr,
        $code:block,
        offset = $offset:expr,
    ) => {
        match ($scale_index_expr.index_from_top()) + $offset {
            1 => { const __SCALE__: usize = 1; $code }
            2 => { const __SCALE__: usize = 2; $code }
            3 => { const __SCALE__: usize = 3; $code }
            4 => { const __SCALE__: usize = 4; $code }
            5 => { const __SCALE__: usize = 5; $code }
            6 => { const __SCALE__: usize = 6; $code }
            7 => { const __SCALE__: usize = 7; $code }
            8 => { const __SCALE__: usize = 8; $code }
            9 => { const __SCALE__: usize = 9; $code }
            10 => { const __SCALE__: usize = 10; $code }
            11 => { const __SCALE__: usize = 11; $code }
            12 => { const __SCALE__: usize = 12; $code }
            13 => { const __SCALE__: usize = 13; $code }
            14 => { const __SCALE__: usize = 14; $code }
            15 => { const __SCALE__: usize = 15; $code }
            16 => { const __SCALE__: usize = 16; $code }
            17 => { const __SCALE__: usize = 17; $code }
            18 => { const __SCALE__: usize = 18; $code }
            19 => { const __SCALE__: usize = 19; $code }
            20 => { const __SCALE__: usize = 20; $code }
            21 => { const __SCALE__: usize = 21; $code }
            22 => { const __SCALE__: usize = 22; $code }
            23 => { const __SCALE__: usize = 23; $code }
            24 => { const __SCALE__: usize = 24; $code }
            25 => { const __SCALE__: usize = 25; $code }
            26 => { const __SCALE__: usize = 26; $code }
            27 => { const __SCALE__: usize = 27; $code }
            28 => { const __SCALE__: usize = 28; $code }
            29 => { const __SCALE__: usize = 29; $code }
            30 => { const __SCALE__: usize = 30; $code }
            31 => { const __SCALE__: usize = 31; $code }
            32 => { const __SCALE__: usize = 32; $code }
            33 => { const __SCALE__: usize = 33; $code }
            34 => { const __SCALE__: usize = 34; $code }
            35 => { const __SCALE__: usize = 35; $code }
            36 => { const __SCALE__: usize = 36; $code }
            37 => { const __SCALE__: usize = 37; $code }
            38 => { const __SCALE__: usize = 38; $code }
            39 => { const __SCALE__: usize = 39; $code }
            40 => { const __SCALE__: usize = 40; $code }
            41 => { const __SCALE__: usize = 41; $code }
            42 => { const __SCALE__: usize = 42; $code }
            43 => { const __SCALE__: usize = 43; $code }
            44 => { const __SCALE__: usize = 44; $code }
            45 => { const __SCALE__: usize = 45; $code }
            46 => { const __SCALE__: usize = 46; $code }
            47 => { const __SCALE__: usize = 47; $code }
            48 => { const __SCALE__: usize = 48; $code }
            49 => { const __SCALE__: usize = 49; $code }
            50 => { const __SCALE__: usize = 50; $code }
            51 => { const __SCALE__: usize = 51; $code }
            52 => { const __SCALE__: usize = 52; $code }
            53 => { const __SCALE__: usize = 53; $code }
            54 => { const __SCALE__: usize = 54; $code }
            55 => { const __SCALE__: usize = 55; $code }
            56 => { const __SCALE__: usize = 56; $code }
            57 => { const __SCALE__: usize = 57; $code }
            58 => { const __SCALE__: usize = 58; $code }
            59 => { const __SCALE__: usize = 59; $code }
            60 => { const __SCALE__: usize = 60; $code }
            61 => { const __SCALE__: usize = 61; $code }
            62 => { const __SCALE__: usize = 62; $code }
            63 => { const __SCALE__: usize = 63; $code }
            64 => { const __SCALE__: usize = 64; $code }
            65 => { const __SCALE__: usize = 65; $code }
            66 => { const __SCALE__: usize = 66; $code }
            67 => { const __SCALE__: usize = 67; $code }
            68 => { const __SCALE__: usize = 68; $code }
            69 => { const __SCALE__: usize = 69; $code }
            70 => { const __SCALE__: usize = 70; $code }
            71 => { const __SCALE__: usize = 71; $code }
            _ => unreachable!(),
        }
    };
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct CurrentViewScale {
    scale: u32
}

pub trait DynScale: Send + Sync + Debug {
    fn name(&self) -> &'static str;
    /// Scale factor exponent, mapped from -35 - 35 to 0 - 70 range where 0 == raw scale factor exponent of 35 (0.00001 quectoMeter)
    fn index_from_top(&self) -> u8 { (self.scale_factor_exponent() + 35) as u8 }
    /// Scale factor exponent, mapped from -35 - 35 to 70 - 0 range where 0 == raw scale factor exponent of -35 (100000 quettaMeter)
    fn index_from_bottom(&self) -> u8 { 70 - (self.scale_factor_exponent() + 35) as u8 }
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
    fn name(&self) -> &'static str {
        scale_type_generic_match!(
            *self,
            { <__SCALE__ as ConstScale>::NAME }
        )
    }
    fn scale_factor_exponent(&self) -> i8 {
        *self as i8
    }
    fn scale_factor(&self) -> f64 {
        scale_type_generic_match!(
            *self,
            { <__SCALE__ as ConstScale>::scale_factor() }
        )
    }
    fn up(&self) -> Option<Scale> {
        scale_type_generic_match!(
            *self,
            { Scale::from_scale_factor_exponent(<__SCALE__ as ConstScale>::Up::SCALE_FACTOR_EXPONENT) },
            ScaleQuettaMeter100000 => { None },
        )
    }
    fn down(&self) -> Option<Scale> {
        scale_type_generic_match!(
            *self,
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

    /// So the max difference in scale can be 3 orders of magnitude
    pub const MAX_DIFF_SCALE_EXP: i8 = 3;

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
        match scale_factor_exponent {
            i8::MIN..=-36_i8 => None,
            -35_i8 => Some(Self::ScaleQuectoMeter000001),
            -34_i8 => Some(Self::ScaleQuectoMeter00001),
            -33_i8 => Some(Self::ScaleQuectoMeter0001),
            -32_i8 => Some(Self::ScaleQuectoMeter001),
            -31_i8 => Some(Self::ScaleQuectoMeter01),
            -30_i8 => Some(Self::ScaleQuectoMeter1),
            -29_i8 => Some(Self::ScaleQuectoMeter10),
            -28_i8 => Some(Self::ScaleQuectoMeter100),
            -27_i8 => Some(Self::ScaleRontoMeter1),
            -26_i8 => Some(Self::ScaleRontoMeter10),
            -25_i8 => Some(Self::ScaleRontoMeter100),
            -24_i8 => Some(Self::ScaleYoctoMeter1),
            -23_i8 => Some(Self::ScaleYoctoMeter10),
            -22_i8 => Some(Self::ScaleYoctoMeter100),
            -21_i8 => Some(Self::ScaleZeptoMeter1),
            -20_i8 => Some(Self::ScaleZeptoMeter10),
            -19_i8 => Some(Self::ScaleZeptoMeter100),
            -18_i8 => Some(Self::ScaleAttoMeter1),
            -17_i8 => Some(Self::ScaleAttoMeter10),
            -16_i8 => Some(Self::ScaleAttoMeter100),
            -15_i8 => Some(Self::ScaleFemtoMeter1),
            -14_i8 => Some(Self::ScaleFemtoMeter10),
            -13_i8 => Some(Self::ScaleFemtoMeter100),
            -12_i8 => Some(Self::ScalePicoMeter1),
            -11_i8 => Some(Self::ScalePicoMeter10),
            -10_i8 => Some(Self::ScalePicoMeter100),
            -9_i8 => Some(Self::ScaleNanoMeter1),
            -8_i8 => Some(Self::ScaleNanoMeter10),
            -7_i8 => Some(Self::ScaleNanoMeter100),
            -6_i8 => Some(Self::ScaleMicroMeter1),
            -5_i8 => Some(Self::ScaleMicroMeter10),
            -4_i8 => Some(Self::ScaleMicroMeter100),
            -3_i8 => Some(Self::ScaleMilliMeter1),
            -2_i8 => Some(Self::ScaleMilliMeter10),
            -1_i8 => Some(Self::ScaleMilliMeter100),
            0_i8 => Some(Self::ScaleMeter1),
            1_i8 => Some(Self::ScaleMeter10),
            2_i8 => Some(Self::ScaleMeter100),
            3_i8 => Some(Self::ScaleKiloMeter1),
            4_i8 => Some(Self::ScaleKiloMeter10),
            5_i8 => Some(Self::ScaleKiloMeter100),
            6_i8 => Some(Self::ScaleMegaMeter1),
            7_i8 => Some(Self::ScaleMegaMeter10),
            8_i8 => Some(Self::ScaleMegaMeter100),
            9_i8 => Some(Self::ScaleGigaMeter1),
            10_i8 => Some(Self::ScaleGigaMeter10),
            11_i8 => Some(Self::ScaleGigaMeter100),
            12_i8 => Some(Self::ScaleTeraMeter1),
            13_i8 => Some(Self::ScaleTeraMeter10),
            14_i8 => Some(Self::ScaleTeraMeter100),
            15_i8 => Some(Self::ScalePetaMeter1),
            16_i8 => Some(Self::ScalePetaMeter10),
            17_i8 => Some(Self::ScalePetaMeter100),
            18_i8 => Some(Self::ScaleExaMeter1),
            19_i8 => Some(Self::ScaleExaMeter10),
            20_i8 => Some(Self::ScaleExaMeter100),
            21_i8 => Some(Self::ScaleZettaMeter1),
            22_i8 => Some(Self::ScaleZettaMeter10),
            23_i8 => Some(Self::ScaleZettaMeter100),
            24_i8 => Some(Self::ScaleYottaMeter1),
            25_i8 => Some(Self::ScaleYottaMeter10),
            26_i8 => Some(Self::ScaleYottaMeter100),
            27_i8 => Some(Self::ScaleRonnaMeter1),
            28_i8 => Some(Self::ScaleRonnaMeter10),
            29_i8 => Some(Self::ScaleRonnaMeter100),
            30_i8 => Some(Self::ScaleQuettaMeter1),
            31_i8 => Some(Self::ScaleQuettaMeter10),
            32_i8 => Some(Self::ScaleQuettaMeter100),
            33_i8 => Some(Self::ScaleQuettaMeter1000),
            34_i8 => Some(Self::ScaleQuettaMeter10000),
            35_i8 => Some(Self::ScaleQuettaMeter100000),
            36_i8..=i8::MAX => None,
        }
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

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct NoLowerScale;
impl ConstScale for NoLowerScale {
    type Up = NoLowerScale;
    type Down = NoLowerScale;

    const SCALE_FACTOR_EXPONENT: i8 = 0;
    const NAME: &'static str = "no_lower_scale";
}
impl std::fmt::Debug for NoLowerScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for NoLowerScale {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        None
    }
    fn down(&self) -> Option<Scale> {
        None
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter000001;
impl ConstScale for ScaleQuectoMeter000001 {
    type Up = ScaleQuectoMeter00001;
    type Down = NoLowerScale;

    const SCALE_FACTOR_EXPONENT: i8 = -35;
    const NAME: &'static str = "scale_quecto_meter_000001";
}
impl std::fmt::Debug for ScaleQuectoMeter000001 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleQuectoMeter000001 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter00001)
    }
    fn down(&self) -> Option<Scale> {
        None
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter00001;
impl ConstScale for ScaleQuectoMeter00001 {
    type Up = ScaleQuectoMeter0001;
    type Down = ScaleQuectoMeter000001;

    const SCALE_FACTOR_EXPONENT: i8 = -34;
    const NAME: &'static str = "scale_quecto_meter_00001";
}
impl std::fmt::Debug for ScaleQuectoMeter00001 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleQuectoMeter00001 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter0001)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter000001)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter0001;
impl ConstScale for ScaleQuectoMeter0001 {
    type Up = ScaleQuectoMeter001;
    type Down = ScaleQuectoMeter00001;

    const SCALE_FACTOR_EXPONENT: i8 = -33;
    const NAME: &'static str = "scale_quecto_meter_0001";
}
impl std::fmt::Debug for ScaleQuectoMeter0001 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleQuectoMeter0001 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter001)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter00001)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter001;
impl ConstScale for ScaleQuectoMeter001 {
    type Up = ScaleQuectoMeter01;
    type Down = ScaleQuectoMeter0001;

    const SCALE_FACTOR_EXPONENT: i8 = -32;
    const NAME: &'static str = "scale_quecto_meter_001";
}
impl std::fmt::Debug for ScaleQuectoMeter001 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleQuectoMeter001 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter01)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter0001)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter01;
impl ConstScale for ScaleQuectoMeter01 {
    type Up = ScaleQuectoMeter1;
    type Down = ScaleQuectoMeter001;

    const SCALE_FACTOR_EXPONENT: i8 = -31;
    const NAME: &'static str = "scale_quecto_meter_01";
}
impl std::fmt::Debug for ScaleQuectoMeter01 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleQuectoMeter01 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter001)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter1;
impl ConstScale for ScaleQuectoMeter1 {
    type Up = ScaleQuectoMeter10;
    type Down = ScaleQuectoMeter01;

    const SCALE_FACTOR_EXPONENT: i8 = -30;
    const NAME: &'static str = "scale_quecto_meter_1";
}
impl std::fmt::Debug for ScaleQuectoMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleQuectoMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter01)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter10;
impl ConstScale for ScaleQuectoMeter10 {
    type Up = ScaleQuectoMeter100;
    type Down = ScaleQuectoMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = -29;
    const NAME: &'static str = "scale_quecto_meter_10";
}
impl std::fmt::Debug for ScaleQuectoMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleQuectoMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter100;
impl ConstScale for ScaleQuectoMeter100 {
    type Up = ScaleRontoMeter1;
    type Down = ScaleQuectoMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = -28;
    const NAME: &'static str = "scale_quecto_meter_100";
}
impl std::fmt::Debug for ScaleQuectoMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleQuectoMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleRontoMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleRontoMeter1;
impl ConstScale for ScaleRontoMeter1 {
    type Up = ScaleRontoMeter10;
    type Down = ScaleQuectoMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = -27;
    const NAME: &'static str = "scale_ronto_meter_1";
}
impl std::fmt::Debug for ScaleRontoMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleRontoMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleRontoMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleQuectoMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleRontoMeter10;
impl ConstScale for ScaleRontoMeter10 {
    type Up = ScaleRontoMeter100;
    type Down = ScaleRontoMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = -26;
    const NAME: &'static str = "scale_ronto_meter_10";
}
impl std::fmt::Debug for ScaleRontoMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleRontoMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleRontoMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleRontoMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleRontoMeter100;
impl ConstScale for ScaleRontoMeter100 {
    type Up = ScaleYoctoMeter1;
    type Down = ScaleRontoMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = -25;
    const NAME: &'static str = "scale_ronto_meter_100";
}
impl std::fmt::Debug for ScaleRontoMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleRontoMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleYoctoMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleRontoMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleYoctoMeter1;
impl ConstScale for ScaleYoctoMeter1 {
    type Up = ScaleYoctoMeter10;
    type Down = ScaleRontoMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = -24;
    const NAME: &'static str = "scale_yocto_meter_1";
}
impl std::fmt::Debug for ScaleYoctoMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleYoctoMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleYoctoMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleRontoMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleYoctoMeter10;
impl ConstScale for ScaleYoctoMeter10 {
    type Up = ScaleYoctoMeter100;
    type Down = ScaleYoctoMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = -23;
    const NAME: &'static str = "scale_yocto_meter_10";
}
impl std::fmt::Debug for ScaleYoctoMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleYoctoMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleYoctoMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleYoctoMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleYoctoMeter100;
impl ConstScale for ScaleYoctoMeter100 {
    type Up = ScaleZeptoMeter1;
    type Down = ScaleYoctoMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = -22;
    const NAME: &'static str = "scale_yocto_meter_100";
}
impl std::fmt::Debug for ScaleYoctoMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleYoctoMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleZeptoMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleYoctoMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleZeptoMeter1;
impl ConstScale for ScaleZeptoMeter1 {
    type Up = ScaleZeptoMeter10;
    type Down = ScaleYoctoMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = -21;
    const NAME: &'static str = "scale_zepto_meter_1";
}
impl std::fmt::Debug for ScaleZeptoMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleZeptoMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleZeptoMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleYoctoMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleZeptoMeter10;
impl ConstScale for ScaleZeptoMeter10 {
    type Up = ScaleZeptoMeter100;
    type Down = ScaleZeptoMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = -20;
    const NAME: &'static str = "scale_zepto_meter_10";
}
impl std::fmt::Debug for ScaleZeptoMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleZeptoMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleZeptoMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleZeptoMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleZeptoMeter100;
impl ConstScale for ScaleZeptoMeter100 {
    type Up = ScaleAttoMeter1;
    type Down = ScaleZeptoMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = -19;
    const NAME: &'static str = "scale_zepto_meter_100";
}
impl std::fmt::Debug for ScaleZeptoMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleZeptoMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleAttoMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleZeptoMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleAttoMeter1;
impl ConstScale for ScaleAttoMeter1 {
    type Up = ScaleAttoMeter10;
    type Down = ScaleZeptoMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = -18;
    const NAME: &'static str = "scale_atto_meter_1";
}
impl std::fmt::Debug for ScaleAttoMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleAttoMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleAttoMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleZeptoMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleAttoMeter10;
impl ConstScale for ScaleAttoMeter10 {
    type Up = ScaleAttoMeter100;
    type Down = ScaleAttoMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = -17;
    const NAME: &'static str = "scale_atto_meter_10";
}
impl std::fmt::Debug for ScaleAttoMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleAttoMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleAttoMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleAttoMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleAttoMeter100;
impl ConstScale for ScaleAttoMeter100 {
    type Up = ScaleFemtoMeter1;
    type Down = ScaleAttoMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = -16;
    const NAME: &'static str = "scale_atto_meter_100";
}
impl std::fmt::Debug for ScaleAttoMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleAttoMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleFemtoMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleAttoMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleFemtoMeter1;
impl ConstScale for ScaleFemtoMeter1 {
    type Up = ScaleFemtoMeter10;
    type Down = ScaleAttoMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = -15;
    const NAME: &'static str = "scale_femto_meter_1";
}
impl std::fmt::Debug for ScaleFemtoMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleFemtoMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleFemtoMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleAttoMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleFemtoMeter10;
impl ConstScale for ScaleFemtoMeter10 {
    type Up = ScaleFemtoMeter100;
    type Down = ScaleFemtoMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = -14;
    const NAME: &'static str = "scale_femto_meter_10";
}
impl std::fmt::Debug for ScaleFemtoMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleFemtoMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleFemtoMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleFemtoMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleFemtoMeter100;
impl ConstScale for ScaleFemtoMeter100 {
    type Up = ScalePicoMeter1;
    type Down = ScaleFemtoMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = -13;
    const NAME: &'static str = "scale_femto_meter_100";
}
impl std::fmt::Debug for ScaleFemtoMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleFemtoMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScalePicoMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleFemtoMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScalePicoMeter1;
impl ConstScale for ScalePicoMeter1 {
    type Up = ScalePicoMeter10;
    type Down = ScaleFemtoMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = -12;
    const NAME: &'static str = "scale_pico_meter_1";
}
impl std::fmt::Debug for ScalePicoMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScalePicoMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScalePicoMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleFemtoMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScalePicoMeter10;
impl ConstScale for ScalePicoMeter10 {
    type Up = ScalePicoMeter100;
    type Down = ScalePicoMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = -11;
    const NAME: &'static str = "scale_pico_meter_10";
}
impl std::fmt::Debug for ScalePicoMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScalePicoMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScalePicoMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScalePicoMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScalePicoMeter100;
impl ConstScale for ScalePicoMeter100 {
    type Up = ScaleNanoMeter1;
    type Down = ScalePicoMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = -10;
    const NAME: &'static str = "scale_pico_meter_100";
}
impl std::fmt::Debug for ScalePicoMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScalePicoMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleNanoMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScalePicoMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleNanoMeter1;
impl ConstScale for ScaleNanoMeter1 {
    type Up = ScaleNanoMeter10;
    type Down = ScalePicoMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = -9;
    const NAME: &'static str = "scale_nano_meter_1";
}
impl std::fmt::Debug for ScaleNanoMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleNanoMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleNanoMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScalePicoMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleNanoMeter10;
impl ConstScale for ScaleNanoMeter10 {
    type Up = ScaleNanoMeter100;
    type Down = ScaleNanoMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = -8;
    const NAME: &'static str = "scale_nano_meter_10";
}
impl std::fmt::Debug for ScaleNanoMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleNanoMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleNanoMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleNanoMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleNanoMeter100;
impl ConstScale for ScaleNanoMeter100 {
    type Up = ScaleMicroMeter1;
    type Down = ScaleNanoMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = -7;
    const NAME: &'static str = "scale_nano_meter_100";
}
impl std::fmt::Debug for ScaleNanoMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleNanoMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleMicroMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleNanoMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMicroMeter1;
impl ConstScale for ScaleMicroMeter1 {
    type Up = ScaleMicroMeter10;
    type Down = ScaleNanoMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = -6;
    const NAME: &'static str = "scale_micro_meter_1";
}
impl std::fmt::Debug for ScaleMicroMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleMicroMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleMicroMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleNanoMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMicroMeter10;
impl ConstScale for ScaleMicroMeter10 {
    type Up = ScaleMicroMeter100;
    type Down = ScaleMicroMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = -5;
    const NAME: &'static str = "scale_micro_meter_10";
}
impl std::fmt::Debug for ScaleMicroMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleMicroMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleMicroMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleMicroMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMicroMeter100;
impl ConstScale for ScaleMicroMeter100 {
    type Up = ScaleMilliMeter1;
    type Down = ScaleMicroMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = -4;
    const NAME: &'static str = "scale_micro_meter_100";
}
impl std::fmt::Debug for ScaleMicroMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleMicroMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleMilliMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleMicroMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMilliMeter1;
impl ConstScale for ScaleMilliMeter1 {
    type Up = ScaleMilliMeter10;
    type Down = ScaleMicroMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = -3;
    const NAME: &'static str = "scale_milli_meter_1";
}
impl std::fmt::Debug for ScaleMilliMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleMilliMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleMilliMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleMicroMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMilliMeter10;
impl ConstScale for ScaleMilliMeter10 {
    type Up = ScaleMilliMeter100;
    type Down = ScaleMilliMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = -2;
    const NAME: &'static str = "scale_milli_meter_10";
}
impl std::fmt::Debug for ScaleMilliMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleMilliMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleMilliMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleMilliMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMilliMeter100;
impl ConstScale for ScaleMilliMeter100 {
    type Up = ScaleMilliMeter10;
    type Down = ScaleMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = -1;
    const NAME: &'static str = "scale_milli_meter_100";
}
impl std::fmt::Debug for ScaleMilliMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleMilliMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleMilliMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMeter1;
impl ConstScale for ScaleMeter1 {
    type Up = ScaleMeter10;
    type Down = ScaleMilliMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = 0;
    const NAME: &'static str = "scale_meter_1";
}
impl std::fmt::Debug for ScaleMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleMilliMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMeter10;
impl ConstScale for ScaleMeter10 {
    type Up = ScaleMeter100;
    type Down = ScaleMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = 1;
    const NAME: &'static str = "scale_meter_10";
}
impl std::fmt::Debug for ScaleMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMeter100;
impl ConstScale for ScaleMeter100 {
    type Up = ScaleKiloMeter1;
    type Down = ScaleMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = 2;
    const NAME: &'static str = "scale_meter_100";
}
impl std::fmt::Debug for ScaleMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleKiloMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleKiloMeter1;
impl ConstScale for ScaleKiloMeter1 {
    type Up = ScaleKiloMeter10;
    type Down = ScaleMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = 3;
    const NAME: &'static str = "scale_kilo_meter_1";
}
impl std::fmt::Debug for ScaleKiloMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleKiloMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleKiloMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleKiloMeter10;
impl ConstScale for ScaleKiloMeter10 {
    type Up = ScaleKiloMeter100;
    type Down = ScaleKiloMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = 4;
    const NAME: &'static str = "scale_kilo_meter_10";
}
impl std::fmt::Debug for ScaleKiloMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleKiloMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleKiloMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleKiloMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleKiloMeter100;
impl ConstScale for ScaleKiloMeter100 {
    type Up = ScaleMegaMeter1;
    type Down = ScaleKiloMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = 5;
    const NAME: &'static str = "scale_kilo_meter_100";
}
impl std::fmt::Debug for ScaleKiloMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleKiloMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleMegaMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleKiloMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMegaMeter1;
impl ConstScale for ScaleMegaMeter1 {
    type Up = ScaleMegaMeter10;
    type Down = ScaleKiloMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = 6;
    const NAME: &'static str = "scale_mega_meter_1";
}
impl std::fmt::Debug for ScaleMegaMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleMegaMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleMegaMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleKiloMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMegaMeter10;
impl ConstScale for ScaleMegaMeter10 {
    type Up = ScaleMegaMeter100;
    type Down = ScaleMegaMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = 7;
    const NAME: &'static str = "scale_mega_meter_10";
}
impl std::fmt::Debug for ScaleMegaMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleMegaMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleMegaMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleMegaMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMegaMeter100;
impl ConstScale for ScaleMegaMeter100 {
    type Up = ScaleGigaMeter1;
    type Down = ScaleMegaMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = 8;
    const NAME: &'static str = "scale_mega_meter_100";
}
impl std::fmt::Debug for ScaleMegaMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleMegaMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleGigaMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleMegaMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleGigaMeter1;
impl ConstScale for ScaleGigaMeter1 {
    type Up = ScaleGigaMeter10;
    type Down = ScaleMegaMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = 9;
    const NAME: &'static str = "scale_giga_meter_1";
}
impl std::fmt::Debug for ScaleGigaMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleGigaMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleGigaMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleMegaMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleGigaMeter10;
impl ConstScale for ScaleGigaMeter10 {
    type Up = ScaleGigaMeter100;
    type Down = ScaleGigaMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = 10;
    const NAME: &'static str = "scale_giga_meter_10";
}
impl std::fmt::Debug for ScaleGigaMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleGigaMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleGigaMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleGigaMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleGigaMeter100;
impl ConstScale for ScaleGigaMeter100 {
    type Up = ScaleTeraMeter1;
    type Down = ScaleGigaMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = 11;
    const NAME: &'static str = "scale_giga_meter_100";
}
impl std::fmt::Debug for ScaleGigaMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleGigaMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleTeraMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleGigaMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleTeraMeter1;
impl ConstScale for ScaleTeraMeter1 {
    type Up = ScaleTeraMeter10;
    type Down = ScaleGigaMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = 12;
    const NAME: &'static str = "scale_tera_meter_1";
}
impl std::fmt::Debug for ScaleTeraMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleTeraMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleTeraMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleGigaMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleTeraMeter10;
impl ConstScale for ScaleTeraMeter10 {
    type Up = ScaleTeraMeter100;
    type Down = ScaleTeraMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = 13;
    const NAME: &'static str = "scale_tera_meter_10";
}
impl std::fmt::Debug for ScaleTeraMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleTeraMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleTeraMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleTeraMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleTeraMeter100;
impl ConstScale for ScaleTeraMeter100 {
    type Up = ScalePetaMeter1;
    type Down = ScaleTeraMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = 14;
    const NAME: &'static str = "scale_tera_meter_100";
}
impl std::fmt::Debug for ScaleTeraMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleTeraMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScalePetaMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleTeraMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScalePetaMeter1;
impl ConstScale for ScalePetaMeter1 {
    type Up = ScalePetaMeter10;
    type Down = ScaleTeraMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = 15;
    const NAME: &'static str = "scale_peta_meter_1";
}
impl std::fmt::Debug for ScalePetaMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScalePetaMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScalePetaMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleTeraMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScalePetaMeter10;
impl ConstScale for ScalePetaMeter10 {
    type Up = ScalePetaMeter100;
    type Down = ScalePetaMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = 16;
    const NAME: &'static str = "scale_peta_meter_10";
}
impl std::fmt::Debug for ScalePetaMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScalePetaMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScalePetaMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScalePetaMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScalePetaMeter100;
impl ConstScale for ScalePetaMeter100 {
    type Up = ScaleExaMeter1;
    type Down = ScalePetaMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = 17;
    const NAME: &'static str = "scale_peta_meter_100";
}
impl std::fmt::Debug for ScalePetaMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScalePetaMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleExaMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScalePetaMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleExaMeter1;
impl ConstScale for ScaleExaMeter1 {
    type Up = ScaleExaMeter10;
    type Down = ScalePetaMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = 18;
    const NAME: &'static str = "scale_exa_meter_1";
}
impl std::fmt::Debug for ScaleExaMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleExaMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleExaMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScalePetaMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleExaMeter10;
impl ConstScale for ScaleExaMeter10 {
    type Up = ScaleExaMeter100;
    type Down = ScaleExaMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = 19;
    const NAME: &'static str = "scale_exa_meter_10";
}
impl std::fmt::Debug for ScaleExaMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleExaMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleExaMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleExaMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleExaMeter100;
impl ConstScale for ScaleExaMeter100 {
    type Up = ScaleZettaMeter1;
    type Down = ScaleExaMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = 20;
    const NAME: &'static str = "scale_exa_meter_100";
}
impl std::fmt::Debug for ScaleExaMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleExaMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleZettaMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleExaMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleZettaMeter1;
impl ConstScale for ScaleZettaMeter1 {
    type Up = ScaleZettaMeter10;
    type Down = ScaleExaMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = 21;
    const NAME: &'static str = "scale_zetta_meter_1";
}
impl std::fmt::Debug for ScaleZettaMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleZettaMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleZettaMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleExaMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleZettaMeter10;
impl ConstScale for ScaleZettaMeter10 {
    type Up = ScaleZettaMeter100;
    type Down = ScaleZettaMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = 22;
    const NAME: &'static str = "scale_zetta_meter_10";
}
impl std::fmt::Debug for ScaleZettaMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleZettaMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleZettaMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleZettaMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleZettaMeter100;
impl ConstScale for ScaleZettaMeter100 {
    type Up = ScaleYottaMeter1;
    type Down = ScaleZettaMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = 23;
    const NAME: &'static str = "scale_zetta_meter_100";
}
impl std::fmt::Debug for ScaleZettaMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleZettaMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleYottaMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleZettaMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleYottaMeter1;
impl ConstScale for ScaleYottaMeter1 {
    type Up = ScaleYottaMeter10;
    type Down = ScaleZettaMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = 24;
    const NAME: &'static str = "scale_yotta_meter_1";
}
impl std::fmt::Debug for ScaleYottaMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleYottaMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleYottaMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleZettaMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleYottaMeter10;
impl ConstScale for ScaleYottaMeter10 {
    type Up = ScaleYottaMeter100;
    type Down = ScaleYottaMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = 25;
    const NAME: &'static str = "scale_yotta_meter_10";
}
impl std::fmt::Debug for ScaleYottaMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleYottaMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleYottaMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleYottaMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleYottaMeter100;
impl ConstScale for ScaleYottaMeter100 {
    type Up = ScaleRonnaMeter1;
    type Down = ScaleYottaMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = 26;
    const NAME: &'static str = "scale_yotta_meter_100";
}
impl std::fmt::Debug for ScaleYottaMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleYottaMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleRonnaMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleYottaMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleRonnaMeter1;
impl ConstScale for ScaleRonnaMeter1 {
    type Up = ScaleRonnaMeter10;
    type Down = ScaleYottaMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = 27;
    const NAME: &'static str = "scale_ronna_meter_1";
}
impl std::fmt::Debug for ScaleRonnaMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleRonnaMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleRonnaMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleYottaMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleRonnaMeter10;
impl ConstScale for ScaleRonnaMeter10 {
    type Up = ScaleRonnaMeter100;
    type Down = ScaleRonnaMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = 28;
    const NAME: &'static str = "scale_ronna_meter_10";
}
impl std::fmt::Debug for ScaleRonnaMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleRonnaMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleRonnaMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleRonnaMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleRonnaMeter100;
impl ConstScale for ScaleRonnaMeter100 {
    type Up = ScaleQuettaMeter1;
    type Down = ScaleRonnaMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = 29;
    const NAME: &'static str = "scale_ronna_meter_100";
}
impl std::fmt::Debug for ScaleRonnaMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleRonnaMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleQuettaMeter1)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleRonnaMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuettaMeter1;
impl ConstScale for ScaleQuettaMeter1 {
    type Up = ScaleQuettaMeter10;
    type Down = ScaleRonnaMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = 30;
    const NAME: &'static str = "scale_quetta_meter_1";
}
impl std::fmt::Debug for ScaleQuettaMeter1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleQuettaMeter1 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleQuettaMeter10)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleRonnaMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuettaMeter10;
impl ConstScale for ScaleQuettaMeter10 {
    type Up = ScaleQuettaMeter100;
    type Down = ScaleQuettaMeter1;

    const SCALE_FACTOR_EXPONENT: i8 = 31;
    const NAME: &'static str = "scale_quetta_meter_10";
}
impl std::fmt::Debug for ScaleQuettaMeter10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleQuettaMeter10 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleQuettaMeter100)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleQuettaMeter1)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuettaMeter100;
impl ConstScale for ScaleQuettaMeter100 {
    type Up = ScaleQuettaMeter1000;
    type Down = ScaleQuettaMeter10;

    const SCALE_FACTOR_EXPONENT: i8 = 32;
    const NAME: &'static str = "scale_quetta_meter_100";
}
impl std::fmt::Debug for ScaleQuettaMeter100 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleQuettaMeter100 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleQuettaMeter1000)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleQuettaMeter10)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuettaMeter1000;
impl ConstScale for ScaleQuettaMeter1000 {
    type Up = ScaleQuettaMeter10000;
    type Down = ScaleQuettaMeter100;

    const SCALE_FACTOR_EXPONENT: i8 = 33;
    const NAME: &'static str = "scale_quetta_meter_1000";
}
impl std::fmt::Debug for ScaleQuettaMeter1000 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleQuettaMeter1000 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleQuettaMeter10000)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleQuettaMeter100)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuettaMeter10000;
impl ConstScale for ScaleQuettaMeter10000 {
    type Up = ScaleQuettaMeter100000;
    type Down = ScaleQuettaMeter1000;

    const SCALE_FACTOR_EXPONENT: i8 = 34;
    const NAME: &'static str = "scale_quetta_meter_10000";
}
impl std::fmt::Debug for ScaleQuettaMeter10000 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleQuettaMeter10000 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        Some(Scale::ScaleQuettaMeter100000)
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleQuettaMeter1000)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuettaMeter100000;
impl ConstScale for ScaleQuettaMeter100000 {
    type Up = NoHigherScale;
    type Down = ScaleQuettaMeter10000;

    const SCALE_FACTOR_EXPONENT: i8 = 35;
    const NAME: &'static str = "scale_quetta_meter_100000";
}
impl std::fmt::Debug for ScaleQuettaMeter100000 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for ScaleQuettaMeter100000 {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        None
    }
    fn down(&self) -> Option<Scale> {
        Some(Scale::ScaleQuettaMeter10000)
    }
}

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct NoHigherScale;
impl ConstScale for NoHigherScale {
    type Up = NoHigherScale;
    type Down = NoHigherScale;

    const SCALE_FACTOR_EXPONENT: i8 = 0;
    const NAME: &'static str = "no_higher_scale";
}
impl std::fmt::Debug for NoHigherScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::NAME)
    }
}
impl DynScale for NoHigherScale {
    fn name(&self) -> &'static str {
        <Self as ConstScale>::NAME
    }
    fn scale_factor_exponent(&self) -> i8 {
        <Self as ConstScale>::SCALE_FACTOR_EXPONENT
    }
    fn scale_factor(&self) -> f64 {
        <Self as ConstScale>::scale_factor()
    }
    fn up(&self) -> Option<Scale> {
        None
    }
    fn down(&self) -> Option<Scale> {
        None
    }
}
