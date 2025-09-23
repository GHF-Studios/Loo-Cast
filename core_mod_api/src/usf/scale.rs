use bevy::prelude::Reflect;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Scale: 'static + Send + Sync + Clone + Default + Debug + Reflect + PartialOrd + Ord + PartialEq + Eq + Hash {
    type Up: Scale;
    type Down: Scale;

    const SCALE_FACTOR_EXPONENT: i8;
    const NAME: &'static str;

    fn scale_factor() -> f64 {
        10f64.powi(Self::SCALE_FACTOR_EXPONENT as i32)
    }
}

pub trait ScaleRangeMarker {
    type Min: Scale;
    type Max: Scale;

    fn includes<S: Scale>() -> bool {
        S::SCALE_FACTOR_EXPONENT >= Self::Min::SCALE_FACTOR_EXPONENT && S::SCALE_FACTOR_EXPONENT <= Self::Max::SCALE_FACTOR_EXPONENT
    }
}

pub struct ScaleRange<Min: Scale, Max: Scale> {
    _phantom: std::marker::PhantomData<(Min, Max)>,
}
impl<Min: Scale, Max: Scale> ScaleRangeMarker for ScaleRange<Min, Max> {
    type Min = Min;
    type Max = Max;
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct NoLowerScale;
impl Scale for NoLowerScale {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter000001;
impl Scale for ScaleQuectoMeter000001 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter00001;
impl Scale for ScaleQuectoMeter00001 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter0001;
impl Scale for ScaleQuectoMeter0001 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter001;
impl Scale for ScaleQuectoMeter001 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter01;
impl Scale for ScaleQuectoMeter01 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter1;
impl Scale for ScaleQuectoMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter10;
impl Scale for ScaleQuectoMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuectoMeter100;
impl Scale for ScaleQuectoMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleRontoMeter1;
impl Scale for ScaleRontoMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleRontoMeter10;
impl Scale for ScaleRontoMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleRontoMeter100;
impl Scale for ScaleRontoMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleYoctoMeter1;
impl Scale for ScaleYoctoMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleYoctoMeter10;
impl Scale for ScaleYoctoMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleYoctoMeter100;
impl Scale for ScaleYoctoMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleZeptoMeter1;
impl Scale for ScaleZeptoMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleZeptoMeter10;
impl Scale for ScaleZeptoMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleZeptoMeter100;
impl Scale for ScaleZeptoMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleAttoMeter1;
impl Scale for ScaleAttoMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleAttoMeter10;
impl Scale for ScaleAttoMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleAttoMeter100;
impl Scale for ScaleAttoMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleFemtoMeter1;
impl Scale for ScaleFemtoMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleFemtoMeter10;
impl Scale for ScaleFemtoMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleFemtoMeter100;
impl Scale for ScaleFemtoMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScalePicoMeter1;
impl Scale for ScalePicoMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScalePicoMeter10;
impl Scale for ScalePicoMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScalePicoMeter100;
impl Scale for ScalePicoMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleNanoMeter1;
impl Scale for ScaleNanoMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleNanoMeter10;
impl Scale for ScaleNanoMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleNanoMeter100;
impl Scale for ScaleNanoMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMicroMeter1;
impl Scale for ScaleMicroMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMicroMeter10;
impl Scale for ScaleMicroMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMicroMeter100;
impl Scale for ScaleMicroMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMilliMeter1;
impl Scale for ScaleMilliMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMilliMeter10;
impl Scale for ScaleMilliMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMilliMeter100;
impl Scale for ScaleMilliMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMeter1;
impl Scale for ScaleMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMeter10;
impl Scale for ScaleMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMeter100;
impl Scale for ScaleMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleKiloMeter1;
impl Scale for ScaleKiloMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleKiloMeter10;
impl Scale for ScaleKiloMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleKiloMeter100;
impl Scale for ScaleKiloMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMegaMeter1;
impl Scale for ScaleMegaMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMegaMeter10;
impl Scale for ScaleMegaMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleMegaMeter100;
impl Scale for ScaleMegaMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleGigaMeter1;
impl Scale for ScaleGigaMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleGigaMeter10;
impl Scale for ScaleGigaMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleGigaMeter100;
impl Scale for ScaleGigaMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleTeraMeter1;
impl Scale for ScaleTeraMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleTeraMeter10;
impl Scale for ScaleTeraMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleTeraMeter100;
impl Scale for ScaleTeraMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScalePetaMeter1;
impl Scale for ScalePetaMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScalePetaMeter10;
impl Scale for ScalePetaMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScalePetaMeter100;
impl Scale for ScalePetaMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleExaMeter1;
impl Scale for ScaleExaMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleExaMeter10;
impl Scale for ScaleExaMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleExaMeter100;
impl Scale for ScaleExaMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleZettaMeter1;
impl Scale for ScaleZettaMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleZettaMeter10;
impl Scale for ScaleZettaMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleZettaMeter100;
impl Scale for ScaleZettaMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleYottaMeter1;
impl Scale for ScaleYottaMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleYottaMeter10;
impl Scale for ScaleYottaMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleYottaMeter100;
impl Scale for ScaleYottaMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleRonnaMeter1;
impl Scale for ScaleRonnaMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleRonnaMeter10;
impl Scale for ScaleRonnaMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleRonnaMeter100;
impl Scale for ScaleRonnaMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuettaMeter1;
impl Scale for ScaleQuettaMeter1 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuettaMeter10;
impl Scale for ScaleQuettaMeter10 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuettaMeter100;
impl Scale for ScaleQuettaMeter100 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuettaMeter1000;
impl Scale for ScaleQuettaMeter1000 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuettaMeter10000;
impl Scale for ScaleQuettaMeter10000 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct ScaleQuettaMeter100000;
impl Scale for ScaleQuettaMeter100000 {
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

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct NoHigherScale;
impl Scale for NoHigherScale {
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
