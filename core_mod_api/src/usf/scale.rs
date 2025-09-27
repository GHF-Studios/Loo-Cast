use bevy::prelude::Reflect;
use std::fmt::Debug;
use std::hash::Hash;

pub trait ConstScale: 'static + Send + Sync + Clone + Default + Debug + Reflect + PartialOrd + Ord + PartialEq + Eq + Hash {
    type Up: ConstScale;
    type Down: ConstScale;

    const SCALE_FACTOR_EXPONENT: i8;
    const NAME: &'static str;

    fn scale_factor() -> f64 {
        10f64.powi(Self::SCALE_FACTOR_EXPONENT as i32)
    }
}

pub trait DynScale: Send + Sync + Debug {
    fn name(&self) -> &'static str;
    fn scale_factor_exponent(&self) -> i8;
    fn scale_factor(&self) -> f64;
    fn up(&self) -> Option<Box<dyn DynScale>>;
    fn down(&self) -> Option<Box<dyn DynScale>>;
}

#[derive(Clone, Default, Debug, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum Scale {
    ScaleQuectoMeter000001,
    ScaleQuectoMeter00001,
    ScaleQuectoMeter0001,
    ScaleQuectoMeter001,
    ScaleQuectoMeter01,
    ScaleQuectoMeter1,
    ScaleQuectoMeter10,
    ScaleQuectoMeter100,
    ScaleRontoMeter1,
    ScaleRontoMeter10,
    ScaleRontoMeter100,
    ScaleYoctoMeter1,
    ScaleYoctoMeter10,
    ScaleYoctoMeter100,
    ScaleZeptoMeter1,
    ScaleZeptoMeter10,
    ScaleZeptoMeter100,
    ScaleAttoMeter1,
    ScaleAttoMeter10,
    ScaleAttoMeter100,
    ScaleFemtoMeter1,
    ScaleFemtoMeter10,
    ScaleFemtoMeter100,
    ScalePicoMeter1,
    ScalePicoMeter10,
    ScalePicoMeter100,
    ScaleNanoMeter1,
    ScaleNanoMeter10,
    ScaleNanoMeter100,
    ScaleMicroMeter1,
    ScaleMicroMeter10,
    ScaleMicroMeter100,
    ScaleMilliMeter1,
    ScaleMilliMeter10,
    ScaleMilliMeter100,
    #[default]
    ScaleMeter1,
    ScaleMeter10,
    ScaleMeter100,
    ScaleKiloMeter1,
    ScaleKiloMeter10,
    ScaleKiloMeter100,
    ScaleMegaMeter1,
    ScaleMegaMeter10,
    ScaleMegaMeter100,
    ScaleGigaMeter1,
    ScaleGigaMeter10,
    ScaleGigaMeter100,
    ScaleTeraMeter1,
    ScaleTeraMeter10,
    ScaleTeraMeter100,
    ScalePetaMeter1,
    ScalePetaMeter10,
    ScalePetaMeter100,
    ScaleExaMeter1,
    ScaleExaMeter10,
    ScaleExaMeter100,
    ScaleZettaMeter1,
    ScaleZettaMeter10,
    ScaleZettaMeter100,
    ScaleYottaMeter1,
    ScaleYottaMeter10,
    ScaleYottaMeter100,
    ScaleRonnaMeter1,
    ScaleRonnaMeter10,
    ScaleRonnaMeter100,
    ScaleQuettaMeter1,
    ScaleQuettaMeter10,
    ScaleQuettaMeter100,
    ScaleQuettaMeter1000,
    ScaleQuettaMeter10000,
    ScaleQuettaMeter100000,
}
impl DynScale for Scale {
    fn name(&self) -> &'static str {
        match self {
            Self::ScaleQuectoMeter000001 => <ScaleQuectoMeter000001 as ConstScale>::NAME,
            Self::ScaleQuectoMeter00001 => <ScaleQuectoMeter00001 as ConstScale>::NAME,
            Self::ScaleQuectoMeter0001 => <ScaleQuectoMeter0001 as ConstScale>::NAME,
            Self::ScaleQuectoMeter001 => <ScaleQuectoMeter001 as ConstScale>::NAME,
            Self::ScaleQuectoMeter01 => <ScaleQuectoMeter01 as ConstScale>::NAME,
            Self::ScaleQuectoMeter1 => <ScaleQuectoMeter1 as ConstScale>::NAME,
            Self::ScaleQuectoMeter10 => <ScaleQuectoMeter10 as ConstScale>::NAME,
            Self::ScaleQuectoMeter100 => <ScaleQuectoMeter100 as ConstScale>::NAME,
            Self::ScaleRontoMeter1 => <ScaleRontoMeter1 as ConstScale>::NAME,
            Self::ScaleRontoMeter10 => <ScaleRontoMeter10 as ConstScale>::NAME,
            Self::ScaleRontoMeter100 => <ScaleRontoMeter100 as ConstScale>::NAME,
            Self::ScaleYoctoMeter1 => <ScaleYoctoMeter1 as ConstScale>::NAME,
            Self::ScaleYoctoMeter10 => <ScaleYoctoMeter10 as ConstScale>::NAME,
            Self::ScaleYoctoMeter100 => <ScaleYoctoMeter100 as ConstScale>::NAME,
            Self::ScaleZeptoMeter1 => <ScaleZeptoMeter1 as ConstScale>::NAME,
            Self::ScaleZeptoMeter10 => <ScaleZeptoMeter10 as ConstScale>::NAME,
            Self::ScaleZeptoMeter100 => <ScaleZeptoMeter100 as ConstScale>::NAME,
            Self::ScaleAttoMeter1 => <ScaleAttoMeter1 as ConstScale>::NAME,
            Self::ScaleAttoMeter10 => <ScaleAttoMeter10 as ConstScale>::NAME,
            Self::ScaleAttoMeter100 => <ScaleAttoMeter100 as ConstScale>::NAME,
            Self::ScaleFemtoMeter1 => <ScaleFemtoMeter1 as ConstScale>::NAME,
            Self::ScaleFemtoMeter10 => <ScaleFemtoMeter10 as ConstScale>::NAME,
            Self::ScaleFemtoMeter100 => <ScaleFemtoMeter100 as ConstScale>::NAME,
            Self::ScalePicoMeter1 => <ScalePicoMeter1 as ConstScale>::NAME,
            Self::ScalePicoMeter10 => <ScalePicoMeter10 as ConstScale>::NAME,
            Self::ScalePicoMeter100 => <ScalePicoMeter100 as ConstScale>::NAME,
            Self::ScaleNanoMeter1 => <ScaleNanoMeter1 as ConstScale>::NAME,
            Self::ScaleNanoMeter10 => <ScaleNanoMeter10 as ConstScale>::NAME,
            Self::ScaleNanoMeter100 => <ScaleNanoMeter100 as ConstScale>::NAME,
            Self::ScaleMicroMeter1 => <ScaleMicroMeter1 as ConstScale>::NAME,
            Self::ScaleMicroMeter10 => <ScaleMicroMeter10 as ConstScale>::NAME,
            Self::ScaleMicroMeter100 => <ScaleMicroMeter100 as ConstScale>::NAME,
            Self::ScaleMilliMeter1 => <ScaleMilliMeter1 as ConstScale>::NAME,
            Self::ScaleMilliMeter10 => <ScaleMilliMeter10 as ConstScale>::NAME,
            Self::ScaleMilliMeter100 => <ScaleMilliMeter100 as ConstScale>::NAME,
            Self::ScaleMeter1 => <ScaleMeter1 as ConstScale>::NAME,
            Self::ScaleMeter10 => <ScaleMeter10 as ConstScale>::NAME,
            Self::ScaleMeter100 => <ScaleMeter100 as ConstScale>::NAME,
            Self::ScaleKiloMeter1 => <ScaleKiloMeter1 as ConstScale>::NAME,
            Self::ScaleKiloMeter10 => <ScaleKiloMeter10 as ConstScale>::NAME,
            Self::ScaleKiloMeter100 => <ScaleKiloMeter100 as ConstScale>::NAME,
            Self::ScaleMegaMeter1 => <ScaleMegaMeter1 as ConstScale>::NAME,
            Self::ScaleMegaMeter10 => <ScaleMegaMeter10 as ConstScale>::NAME,
            Self::ScaleMegaMeter100 => <ScaleMegaMeter100 as ConstScale>::NAME,
            Self::ScaleGigaMeter1 => <ScaleGigaMeter1 as ConstScale>::NAME,
            Self::ScaleGigaMeter10 => <ScaleGigaMeter10 as ConstScale>::NAME,
            Self::ScaleGigaMeter100 => <ScaleGigaMeter100 as ConstScale>::NAME,
            Self::ScaleTeraMeter1 => <ScaleTeraMeter1 as ConstScale>::NAME,
            Self::ScaleTeraMeter10 => <ScaleTeraMeter10 as ConstScale>::NAME,
            Self::ScaleTeraMeter100 => <ScaleTeraMeter100 as ConstScale>::NAME,
            Self::ScalePetaMeter1 => <ScalePetaMeter1 as ConstScale>::NAME,
            Self::ScalePetaMeter10 => <ScalePetaMeter10 as ConstScale>::NAME,
            Self::ScalePetaMeter100 => <ScalePetaMeter100 as ConstScale>::NAME,
            Self::ScaleExaMeter1 => <ScaleExaMeter1 as ConstScale>::NAME,
            Self::ScaleExaMeter10 => <ScaleExaMeter10 as ConstScale>::NAME,
            Self::ScaleExaMeter100 => <ScaleExaMeter100 as ConstScale>::NAME,
            Self::ScaleZettaMeter1 => <ScaleZettaMeter1 as ConstScale>::NAME,
            Self::ScaleZettaMeter10 => <ScaleZettaMeter10 as ConstScale>::NAME,
            Self::ScaleZettaMeter100 => <ScaleZettaMeter100 as ConstScale>::NAME,
            Self::ScaleYottaMeter1 => <ScaleYottaMeter1 as ConstScale>::NAME,
            Self::ScaleYottaMeter10 => <ScaleYottaMeter10 as ConstScale>::NAME,
            Self::ScaleYottaMeter100 => <ScaleYottaMeter100 as ConstScale>::NAME,
            Self::ScaleRonnaMeter1 => <ScaleRonnaMeter1 as ConstScale>::NAME,
            Self::ScaleRonnaMeter10 => <ScaleRonnaMeter10 as ConstScale>::NAME,
            Self::ScaleRonnaMeter100 => <ScaleRonnaMeter100 as ConstScale>::NAME,
            Self::ScaleQuettaMeter1 => <ScaleQuettaMeter1 as ConstScale>::NAME,
            Self::ScaleQuettaMeter10 => <ScaleQuettaMeter10 as ConstScale>::NAME,
            Self::ScaleQuettaMeter100 => <ScaleQuettaMeter100 as ConstScale>::NAME,
            Self::ScaleQuettaMeter1000 => <ScaleQuettaMeter1000 as ConstScale>::NAME,
            Self::ScaleQuettaMeter10000 => <ScaleQuettaMeter10000 as ConstScale>::NAME,
            Self::ScaleQuettaMeter100000 => <ScaleQuettaMeter100000 as ConstScale>::NAME,
        }
    }
    fn scale_factor_exponent(&self) -> i8 {
        match self {
            Self::ScaleQuectoMeter000001 => <ScaleQuectoMeter000001 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleQuectoMeter00001 => <ScaleQuectoMeter00001 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleQuectoMeter0001 => <ScaleQuectoMeter0001 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleQuectoMeter001 => <ScaleQuectoMeter001 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleQuectoMeter01 => <ScaleQuectoMeter01 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleQuectoMeter1 => <ScaleQuectoMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleQuectoMeter10 => <ScaleQuectoMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleQuectoMeter100 => <ScaleQuectoMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleRontoMeter1 => <ScaleRontoMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleRontoMeter10 => <ScaleRontoMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleRontoMeter100 => <ScaleRontoMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleYoctoMeter1 => <ScaleYoctoMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleYoctoMeter10 => <ScaleYoctoMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleYoctoMeter100 => <ScaleYoctoMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleZeptoMeter1 => <ScaleZeptoMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleZeptoMeter10 => <ScaleZeptoMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleZeptoMeter100 => <ScaleZeptoMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleAttoMeter1 => <ScaleAttoMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleAttoMeter10 => <ScaleAttoMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleAttoMeter100 => <ScaleAttoMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleFemtoMeter1 => <ScaleFemtoMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleFemtoMeter10 => <ScaleFemtoMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleFemtoMeter100 => <ScaleFemtoMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScalePicoMeter1 => <ScalePicoMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScalePicoMeter10 => <ScalePicoMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScalePicoMeter100 => <ScalePicoMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleNanoMeter1 => <ScaleNanoMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleNanoMeter10 => <ScaleNanoMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleNanoMeter100 => <ScaleNanoMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleMicroMeter1 => <ScaleMicroMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleMicroMeter10 => <ScaleMicroMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleMicroMeter100 => <ScaleMicroMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleMilliMeter1 => <ScaleMilliMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleMilliMeter10 => <ScaleMilliMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleMilliMeter100 => <ScaleMilliMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleMeter1 => <ScaleMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleMeter10 => <ScaleMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleMeter100 => <ScaleMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleKiloMeter1 => <ScaleKiloMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleKiloMeter10 => <ScaleKiloMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleKiloMeter100 => <ScaleKiloMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleMegaMeter1 => <ScaleMegaMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleMegaMeter10 => <ScaleMegaMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleMegaMeter100 => <ScaleMegaMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleGigaMeter1 => <ScaleGigaMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleGigaMeter10 => <ScaleGigaMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleGigaMeter100 => <ScaleGigaMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleTeraMeter1 => <ScaleTeraMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleTeraMeter10 => <ScaleTeraMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleTeraMeter100 => <ScaleTeraMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScalePetaMeter1 => <ScalePetaMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScalePetaMeter10 => <ScalePetaMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScalePetaMeter100 => <ScalePetaMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleExaMeter1 => <ScaleExaMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleExaMeter10 => <ScaleExaMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleExaMeter100 => <ScaleExaMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleZettaMeter1 => <ScaleZettaMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleZettaMeter10 => <ScaleZettaMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleZettaMeter100 => <ScaleZettaMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleYottaMeter1 => <ScaleYottaMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleYottaMeter10 => <ScaleYottaMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleYottaMeter100 => <ScaleYottaMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleRonnaMeter1 => <ScaleRonnaMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleRonnaMeter10 => <ScaleRonnaMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleRonnaMeter100 => <ScaleRonnaMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleQuettaMeter1 => <ScaleQuettaMeter1 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleQuettaMeter10 => <ScaleQuettaMeter10 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleQuettaMeter100 => <ScaleQuettaMeter100 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleQuettaMeter1000 => <ScaleQuettaMeter1000 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleQuettaMeter10000 => <ScaleQuettaMeter10000 as ConstScale>::SCALE_FACTOR_EXPONENT,
            Self::ScaleQuettaMeter100000 => <ScaleQuettaMeter100000 as ConstScale>::SCALE_FACTOR_EXPONENT,
        }
    }
    fn scale_factor(&self) -> f64 {
        match self {
            Self::ScaleQuectoMeter000001 => <ScaleQuectoMeter000001 as ConstScale>::scale_factor(),
            Self::ScaleQuectoMeter00001 => <ScaleQuectoMeter00001 as ConstScale>::scale_factor(),
            Self::ScaleQuectoMeter0001 => <ScaleQuectoMeter0001 as ConstScale>::scale_factor(),
            Self::ScaleQuectoMeter001 => <ScaleQuectoMeter001 as ConstScale>::scale_factor(),
            Self::ScaleQuectoMeter01 => <ScaleQuectoMeter01 as ConstScale>::scale_factor(),
            Self::ScaleQuectoMeter1 => <ScaleQuectoMeter1 as ConstScale>::scale_factor(),
            Self::ScaleQuectoMeter10 => <ScaleQuectoMeter10 as ConstScale>::scale_factor(),
            Self::ScaleQuectoMeter100 => <ScaleQuectoMeter100 as ConstScale>::scale_factor(),
            Self::ScaleRontoMeter1 => <ScaleRontoMeter1 as ConstScale>::scale_factor(),
            Self::ScaleRontoMeter10 => <ScaleRontoMeter10 as ConstScale>::scale_factor(),
            Self::ScaleRontoMeter100 => <ScaleRontoMeter100 as ConstScale>::scale_factor(),
            Self::ScaleYoctoMeter1 => <ScaleYoctoMeter1 as ConstScale>::scale_factor(),
            Self::ScaleYoctoMeter10 => <ScaleYoctoMeter10 as ConstScale>::scale_factor(),
            Self::ScaleYoctoMeter100 => <ScaleYoctoMeter100 as ConstScale>::scale_factor(),
            Self::ScaleZeptoMeter1 => <ScaleZeptoMeter1 as ConstScale>::scale_factor(),
            Self::ScaleZeptoMeter10 => <ScaleZeptoMeter10 as ConstScale>::scale_factor(),
            Self::ScaleZeptoMeter100 => <ScaleZeptoMeter100 as ConstScale>::scale_factor(),
            Self::ScaleAttoMeter1 => <ScaleAttoMeter1 as ConstScale>::scale_factor(),
            Self::ScaleAttoMeter10 => <ScaleAttoMeter10 as ConstScale>::scale_factor(),
            Self::ScaleAttoMeter100 => <ScaleAttoMeter100 as ConstScale>::scale_factor(),
            Self::ScaleFemtoMeter1 => <ScaleFemtoMeter1 as ConstScale>::scale_factor(),
            Self::ScaleFemtoMeter10 => <ScaleFemtoMeter10 as ConstScale>::scale_factor(),
            Self::ScaleFemtoMeter100 => <ScaleFemtoMeter100 as ConstScale>::scale_factor(),
            Self::ScalePicoMeter1 => <ScalePicoMeter1 as ConstScale>::scale_factor(),
            Self::ScalePicoMeter10 => <ScalePicoMeter10 as ConstScale>::scale_factor(),
            Self::ScalePicoMeter100 => <ScalePicoMeter100 as ConstScale>::scale_factor(),
            Self::ScaleNanoMeter1 => <ScaleNanoMeter1 as ConstScale>::scale_factor(),
            Self::ScaleNanoMeter10 => <ScaleNanoMeter10 as ConstScale>::scale_factor(),
            Self::ScaleNanoMeter100 => <ScaleNanoMeter100 as ConstScale>::scale_factor(),
            Self::ScaleMicroMeter1 => <ScaleMicroMeter1 as ConstScale>::scale_factor(),
            Self::ScaleMicroMeter10 => <ScaleMicroMeter10 as ConstScale>::scale_factor(),
            Self::ScaleMicroMeter100 => <ScaleMicroMeter100 as ConstScale>::scale_factor(),
            Self::ScaleMilliMeter1 => <ScaleMilliMeter1 as ConstScale>::scale_factor(),
            Self::ScaleMilliMeter10 => <ScaleMilliMeter10 as ConstScale>::scale_factor(),
            Self::ScaleMilliMeter100 => <ScaleMilliMeter100 as ConstScale>::scale_factor(),
            Self::ScaleMeter1 => <ScaleMeter1 as ConstScale>::scale_factor(),
            Self::ScaleMeter10 => <ScaleMeter10 as ConstScale>::scale_factor(),
            Self::ScaleMeter100 => <ScaleMeter100 as ConstScale>::scale_factor(),
            Self::ScaleKiloMeter1 => <ScaleKiloMeter1 as ConstScale>::scale_factor(),
            Self::ScaleKiloMeter10 => <ScaleKiloMeter10 as ConstScale>::scale_factor(),
            Self::ScaleKiloMeter100 => <ScaleKiloMeter100 as ConstScale>::scale_factor(),
            Self::ScaleMegaMeter1 => <ScaleMegaMeter1 as ConstScale>::scale_factor(),
            Self::ScaleMegaMeter10 => <ScaleMegaMeter10 as ConstScale>::scale_factor(),
            Self::ScaleMegaMeter100 => <ScaleMegaMeter100 as ConstScale>::scale_factor(),
            Self::ScaleGigaMeter1 => <ScaleGigaMeter1 as ConstScale>::scale_factor(),
            Self::ScaleGigaMeter10 => <ScaleGigaMeter10 as ConstScale>::scale_factor(),
            Self::ScaleGigaMeter100 => <ScaleGigaMeter100 as ConstScale>::scale_factor(),
            Self::ScaleTeraMeter1 => <ScaleTeraMeter1 as ConstScale>::scale_factor(),
            Self::ScaleTeraMeter10 => <ScaleTeraMeter10 as ConstScale>::scale_factor(),
            Self::ScaleTeraMeter100 => <ScaleTeraMeter100 as ConstScale>::scale_factor(),
            Self::ScalePetaMeter1 => <ScalePetaMeter1 as ConstScale>::scale_factor(),
            Self::ScalePetaMeter10 => <ScalePetaMeter10 as ConstScale>::scale_factor(),
            Self::ScalePetaMeter100 => <ScalePetaMeter100 as ConstScale>::scale_factor(),
            Self::ScaleExaMeter1 => <ScaleExaMeter1 as ConstScale>::scale_factor(),
            Self::ScaleExaMeter10 => <ScaleExaMeter10 as ConstScale>::scale_factor(),
            Self::ScaleExaMeter100 => <ScaleExaMeter100 as ConstScale>::scale_factor(),
            Self::ScaleZettaMeter1 => <ScaleZettaMeter1 as ConstScale>::scale_factor(),
            Self::ScaleZettaMeter10 => <ScaleZettaMeter10 as ConstScale>::scale_factor(),
            Self::ScaleZettaMeter100 => <ScaleZettaMeter100 as ConstScale>::scale_factor(),
            Self::ScaleYottaMeter1 => <ScaleYottaMeter1 as ConstScale>::scale_factor(),
            Self::ScaleYottaMeter10 => <ScaleYottaMeter10 as ConstScale>::scale_factor(),
            Self::ScaleYottaMeter100 => <ScaleYottaMeter100 as ConstScale>::scale_factor(),
            Self::ScaleRonnaMeter1 => <ScaleRonnaMeter1 as ConstScale>::scale_factor(),
            Self::ScaleRonnaMeter10 => <ScaleRonnaMeter10 as ConstScale>::scale_factor(),
            Self::ScaleRonnaMeter100 => <ScaleRonnaMeter100 as ConstScale>::scale_factor(),
            Self::ScaleQuettaMeter1 => <ScaleQuettaMeter1 as ConstScale>::scale_factor(),
            Self::ScaleQuettaMeter10 => <ScaleQuettaMeter10 as ConstScale>::scale_factor(),
            Self::ScaleQuettaMeter100 => <ScaleQuettaMeter100 as ConstScale>::scale_factor(),
            Self::ScaleQuettaMeter1000 => <ScaleQuettaMeter1000 as ConstScale>::scale_factor(),
            Self::ScaleQuettaMeter10000 => <ScaleQuettaMeter10000 as ConstScale>::scale_factor(),
            Self::ScaleQuettaMeter100000 => <ScaleQuettaMeter100000 as ConstScale>::scale_factor(),
        }
    }
    fn up(&self) -> Option<Box<dyn DynScale>> {
        match self {
            Self::ScaleQuectoMeter000001 => Some(Box::new(ScaleQuectoMeter00001::default())),
            Self::ScaleQuectoMeter00001 => Some(Box::new(ScaleQuectoMeter0001::default())),
            Self::ScaleQuectoMeter0001 => Some(Box::new(ScaleQuectoMeter001::default())),
            Self::ScaleQuectoMeter001 => Some(Box::new(ScaleQuectoMeter01::default())),
            Self::ScaleQuectoMeter01 => Some(Box::new(ScaleQuectoMeter1::default())),
            Self::ScaleQuectoMeter1 => Some(Box::new(ScaleQuectoMeter10::default())),
            Self::ScaleQuectoMeter10 => Some(Box::new(ScaleQuectoMeter100::default())),
            Self::ScaleQuectoMeter100 => Some(Box::new(ScaleRontoMeter1::default())),
            Self::ScaleRontoMeter1 => Some(Box::new(ScaleRontoMeter10::default())),
            Self::ScaleRontoMeter10 => Some(Box::new(ScaleRontoMeter100::default())),
            Self::ScaleRontoMeter100 => Some(Box::new(ScaleYoctoMeter1::default())),
            Self::ScaleYoctoMeter1 => Some(Box::new(ScaleYoctoMeter10::default())),
            Self::ScaleYoctoMeter10 => Some(Box::new(ScaleYoctoMeter100::default())),
            Self::ScaleYoctoMeter100 => Some(Box::new(ScaleZeptoMeter1::default())),
            Self::ScaleZeptoMeter1 => Some(Box::new(ScaleZeptoMeter10::default())),
            Self::ScaleZeptoMeter10 => Some(Box::new(ScaleZeptoMeter100::default())),
            Self::ScaleZeptoMeter100 => Some(Box::new(ScaleAttoMeter1::default())),
            Self::ScaleAttoMeter1 => Some(Box::new(ScaleAttoMeter10::default())),
            Self::ScaleAttoMeter10 => Some(Box::new(ScaleAttoMeter100::default())),
            Self::ScaleAttoMeter100 => Some(Box::new(ScaleFemtoMeter1::default())),
            Self::ScaleFemtoMeter1 => Some(Box::new(ScaleFemtoMeter10::default())),
            Self::ScaleFemtoMeter10 => Some(Box::new(ScaleFemtoMeter100::default())),
            Self::ScaleFemtoMeter100 => Some(Box::new(ScalePicoMeter1::default())),
            Self::ScalePicoMeter1 => Some(Box::new(ScalePicoMeter10::default())),
            Self::ScalePicoMeter10 => Some(Box::new(ScalePicoMeter100::default())),
            Self::ScalePicoMeter100 => Some(Box::new(ScaleNanoMeter1::default())),
            Self::ScaleNanoMeter1 => Some(Box::new(ScaleNanoMeter10::default())),
            Self::ScaleNanoMeter10 => Some(Box::new(ScaleNanoMeter100::default())),
            Self::ScaleNanoMeter100 => Some(Box::new(ScaleMicroMeter1::default())),
            Self::ScaleMicroMeter1 => Some(Box::new(ScaleMicroMeter10::default())),
            Self::ScaleMicroMeter10 => Some(Box::new(ScaleMicroMeter100::default())),
            Self::ScaleMicroMeter100 => Some(Box::new(ScaleMilliMeter1::default())),
            Self::ScaleMilliMeter1 => Some(Box::new(ScaleMilliMeter10::default())),
            Self::ScaleMilliMeter10 => Some(Box::new(ScaleMilliMeter100::default())),
            Self::ScaleMilliMeter100 => Some(Box::new(ScaleMeter1::default())),
            Self::ScaleMeter1 => Some(Box::new(ScaleMeter10::default())),
            Self::ScaleMeter10 => Some(Box::new(ScaleMeter100::default())),
            Self::ScaleMeter100 => Some(Box::new(ScaleKiloMeter1::default())),
            Self::ScaleKiloMeter1 => Some(Box::new(ScaleKiloMeter10::default())),
            Self::ScaleKiloMeter10 => Some(Box::new(ScaleKiloMeter100::default())),
            Self::ScaleKiloMeter100 => Some(Box::new(ScaleMegaMeter1::default())),
            Self::ScaleMegaMeter1 => Some(Box::new(ScaleMegaMeter10::default())),
            Self::ScaleMegaMeter10 => Some(Box::new(ScaleMegaMeter100::default())),
            Self::ScaleMegaMeter100 => Some(Box::new(ScaleGigaMeter1::default())),
            Self::ScaleGigaMeter1 => Some(Box::new(ScaleGigaMeter10::default())),
            Self::ScaleGigaMeter10 => Some(Box::new(ScaleGigaMeter100::default())),
            Self::ScaleGigaMeter100 => Some(Box::new(ScaleTeraMeter1::default())),
            Self::ScaleTeraMeter1 => Some(Box::new(ScaleTeraMeter10::default())),
            Self::ScaleTeraMeter10 => Some(Box::new(ScaleTeraMeter100::default())),
            Self::ScaleTeraMeter100 => Some(Box::new(ScalePetaMeter1::default())),
            Self::ScalePetaMeter1 => Some(Box::new(ScalePetaMeter10::default())),
            Self::ScalePetaMeter10 => Some(Box::new(ScalePetaMeter100::default())),
            Self::ScalePetaMeter100 => Some(Box::new(ScaleExaMeter1::default())),
            Self::ScaleExaMeter1 => Some(Box::new(ScaleExaMeter10::default())),
            Self::ScaleExaMeter10 => Some(Box::new(ScaleExaMeter100::default())),
            Self::ScaleExaMeter100 => Some(Box::new(ScaleZettaMeter1::default())),
            Self::ScaleZettaMeter1 => Some(Box::new(ScaleZettaMeter10::default())),
            Self::ScaleZettaMeter10 => Some(Box::new(ScaleZettaMeter100::default())),
            Self::ScaleZettaMeter100 => Some(Box::new(ScaleYottaMeter1::default())),
            Self::ScaleYottaMeter1 => Some(Box::new(ScaleYottaMeter10::default())),
            Self::ScaleYottaMeter10 => Some(Box::new(ScaleYottaMeter100::default())),
            Self::ScaleYottaMeter100 => Some(Box::new(ScaleRonnaMeter1::default())),
            Self::ScaleRonnaMeter1 => Some(Box::new(ScaleRonnaMeter10::default())),
            Self::ScaleRonnaMeter10 => Some(Box::new(ScaleRonnaMeter100::default())),
            Self::ScaleRonnaMeter100 => Some(Box::new(ScaleQuettaMeter1::default())),
            Self::ScaleQuettaMeter1 => Some(Box::new(ScaleQuettaMeter10::default())),
            Self::ScaleQuettaMeter10 => Some(Box::new(ScaleQuettaMeter100::default())),
            Self::ScaleQuettaMeter100 => Some(Box::new(ScaleQuettaMeter1000::default())),
            Self::ScaleQuettaMeter1000 => Some(Box::new(ScaleQuettaMeter10000::default())),
            Self::ScaleQuettaMeter10000 => Some(Box::new(ScaleQuettaMeter100000::default())),
            Self::ScaleQuettaMeter100000 => Some(Box::new(NoHigherScale::default())),
        }
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        match self {
            Self::ScaleQuectoMeter000001 => Some(Box::new(NoLowerScale::default())),
            Self::ScaleQuectoMeter00001 => Some(Box::new(ScaleQuectoMeter000001::default())),
            Self::ScaleQuectoMeter0001 => Some(Box::new(ScaleQuectoMeter00001::default())),
            Self::ScaleQuectoMeter001 => Some(Box::new(ScaleQuectoMeter0001::default())),
            Self::ScaleQuectoMeter01 => Some(Box::new(ScaleQuectoMeter001::default())),
            Self::ScaleQuectoMeter1 => Some(Box::new(ScaleQuectoMeter01::default())),
            Self::ScaleQuectoMeter10 => Some(Box::new(ScaleQuectoMeter1::default())),
            Self::ScaleQuectoMeter100 => Some(Box::new(ScaleQuectoMeter10::default())),
            Self::ScaleRontoMeter1 => Some(Box::new(ScaleQuectoMeter100::default())),
            Self::ScaleRontoMeter10 => Some(Box::new(ScaleRontoMeter1::default())),
            Self::ScaleRontoMeter100 => Some(Box::new(ScaleRontoMeter10::default())),
            Self::ScaleYoctoMeter1 => Some(Box::new(ScaleRontoMeter100::default())),
            Self::ScaleYoctoMeter10 => Some(Box::new(ScaleYoctoMeter1::default())),
            Self::ScaleYoctoMeter100 => Some(Box::new(ScaleYoctoMeter10::default())),
            Self::ScaleZeptoMeter1 => Some(Box::new(ScaleYoctoMeter100::default())),
            Self::ScaleZeptoMeter10 => Some(Box::new(ScaleZeptoMeter1::default())),
            Self::ScaleZeptoMeter100 => Some(Box::new(ScaleZeptoMeter10::default())),
            Self::ScaleAttoMeter1 => Some(Box::new(ScaleZeptoMeter100::default())),
            Self::ScaleAttoMeter10 => Some(Box::new(ScaleAttoMeter1::default())),
            Self::ScaleAttoMeter100 => Some(Box::new(ScaleAttoMeter10::default())),
            Self::ScaleFemtoMeter1 => Some(Box::new(ScaleAttoMeter100::default())),
            Self::ScaleFemtoMeter10 => Some(Box::new(ScaleFemtoMeter1::default())),
            Self::ScaleFemtoMeter100 => Some(Box::new(ScaleFemtoMeter10::default())),
            Self::ScalePicoMeter1 => Some(Box::new(ScaleFemtoMeter100::default())),
            Self::ScalePicoMeter10 => Some(Box::new(ScalePicoMeter1::default())),
            Self::ScalePicoMeter100 => Some(Box::new(ScalePicoMeter10::default())),
            Self::ScaleNanoMeter1 => Some(Box::new(ScalePicoMeter100::default())),
            Self::ScaleNanoMeter10 => Some(Box::new(ScaleNanoMeter1::default())),
            Self::ScaleNanoMeter100 => Some(Box::new(ScaleNanoMeter10::default())),
            Self::ScaleMicroMeter1 => Some(Box::new(ScaleNanoMeter100::default())),
            Self::ScaleMicroMeter10 => Some(Box::new(ScaleMicroMeter1::default())),
            Self::ScaleMicroMeter100 => Some(Box::new(ScaleMicroMeter10::default())),
            Self::ScaleMilliMeter1 => Some(Box::new(ScaleMicroMeter100::default())),
            Self::ScaleMilliMeter10 => Some(Box::new(ScaleMilliMeter1::default())),
            Self::ScaleMilliMeter100 => Some(Box::new(ScaleMilliMeter10::default())),
            Self::ScaleMeter1 => Some(Box::new(ScaleMilliMeter100::default())),
            Self::ScaleMeter10 => Some(Box::new(ScaleMeter1::default())),
            Self::ScaleMeter100 => Some(Box::new(ScaleMeter10::default())),
            Self::ScaleKiloMeter1 => Some(Box::new(ScaleMeter100::default())),
            Self::ScaleKiloMeter10 => Some(Box::new(ScaleKiloMeter1::default())),
            Self::ScaleKiloMeter100 => Some(Box::new(ScaleKiloMeter10::default())),
            Self::ScaleMegaMeter1 => Some(Box::new(ScaleKiloMeter100::default())),
            Self::ScaleMegaMeter10 => Some(Box::new(ScaleMegaMeter1::default())),
            Self::ScaleMegaMeter100 => Some(Box::new(ScaleMegaMeter10::default())),
            Self::ScaleGigaMeter1 => Some(Box::new(ScaleMegaMeter100::default())),
            Self::ScaleGigaMeter10 => Some(Box::new(ScaleGigaMeter1::default())),
            Self::ScaleGigaMeter100 => Some(Box::new(ScaleGigaMeter10::default())),
            Self::ScaleTeraMeter1 => Some(Box::new(ScaleGigaMeter100::default())),
            Self::ScaleTeraMeter10 => Some(Box::new(ScaleTeraMeter1::default())),
            Self::ScaleTeraMeter100 => Some(Box::new(ScaleTeraMeter10::default())),
            Self::ScalePetaMeter1 => Some(Box::new(ScaleTeraMeter100::default())),
            Self::ScalePetaMeter10 => Some(Box::new(ScalePetaMeter1::default())),
            Self::ScalePetaMeter100 => Some(Box::new(ScalePetaMeter10::default())),
            Self::ScaleExaMeter1 => Some(Box::new(ScalePetaMeter100::default())),
            Self::ScaleExaMeter10 => Some(Box::new(ScaleExaMeter1::default())),
            Self::ScaleExaMeter100 => Some(Box::new(ScaleExaMeter10::default())),
            Self::ScaleZettaMeter1 => Some(Box::new(ScaleExaMeter100::default())),
            Self::ScaleZettaMeter10 => Some(Box::new(ScaleZettaMeter1::default())),
            Self::ScaleZettaMeter100 => Some(Box::new(ScaleZettaMeter10::default())),
            Self::ScaleYottaMeter1 => Some(Box::new(ScaleZettaMeter100::default())),
            Self::ScaleYottaMeter10 => Some(Box::new(ScaleYottaMeter1::default())),
            Self::ScaleYottaMeter100 => Some(Box::new(ScaleYottaMeter10::default())),
            Self::ScaleRonnaMeter1 => Some(Box::new(ScaleYottaMeter100::default())),
            Self::ScaleRonnaMeter10 => Some(Box::new(ScaleRonnaMeter1::default())),
            Self::ScaleRonnaMeter100 => Some(Box::new(ScaleRonnaMeter10::default())),
            Self::ScaleQuettaMeter1 => Some(Box::new(ScaleRonnaMeter100::default())),
            Self::ScaleQuettaMeter10 => Some(Box::new(ScaleQuettaMeter1::default())),
            Self::ScaleQuettaMeter100 => Some(Box::new(ScaleQuettaMeter10::default())),
            Self::ScaleQuettaMeter1000 => Some(Box::new(ScaleQuettaMeter100::default())),
            Self::ScaleQuettaMeter10000 => Some(Box::new(ScaleQuettaMeter1000::default())),
            Self::ScaleQuettaMeter100000 => Some(Box::new(ScaleQuettaMeter10000::default())),
        }
    }
}

pub trait ScaleRangeMarker {
    type Min: ConstScale;
    type Max: ConstScale;

    fn includes<S: ConstScale>() -> bool {
        S::SCALE_FACTOR_EXPONENT >= Self::Min::SCALE_FACTOR_EXPONENT && S::SCALE_FACTOR_EXPONENT <= Self::Max::SCALE_FACTOR_EXPONENT
    }
}

pub struct ScaleRange<Min: ConstScale, Max: ConstScale> {
    _phantom: std::marker::PhantomData<(Min, Max)>,
}
impl<Min: ConstScale, Max: ConstScale> ScaleRangeMarker for ScaleRange<Min, Max> {
    type Min = Min;
    type Max = Max;
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        None
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        None
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter00001::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        None
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter0001::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter000001::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter001::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter00001::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter01::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter0001::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter001::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter01::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleRontoMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleRontoMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuectoMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleRontoMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleRontoMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleYoctoMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleRontoMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleYoctoMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleRontoMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleYoctoMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleYoctoMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleZeptoMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleYoctoMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleZeptoMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleYoctoMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleZeptoMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleZeptoMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleAttoMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleZeptoMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleAttoMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleZeptoMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleAttoMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleAttoMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleFemtoMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleAttoMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleFemtoMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleAttoMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleFemtoMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleFemtoMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScalePicoMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleFemtoMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScalePicoMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleFemtoMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScalePicoMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScalePicoMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleNanoMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScalePicoMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleNanoMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScalePicoMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleNanoMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleNanoMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMicroMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleNanoMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMicroMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleNanoMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMicroMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMicroMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMilliMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMicroMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMilliMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMicroMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMilliMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMilliMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMilliMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMilliMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleKiloMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleKiloMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleKiloMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleKiloMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMegaMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleKiloMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMegaMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleKiloMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMegaMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMegaMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleGigaMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMegaMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleGigaMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleMegaMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleGigaMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleGigaMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleTeraMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleGigaMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleTeraMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleGigaMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleTeraMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleTeraMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScalePetaMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleTeraMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScalePetaMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleTeraMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScalePetaMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScalePetaMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleExaMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScalePetaMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleExaMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScalePetaMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleExaMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleExaMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleZettaMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleExaMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleZettaMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleExaMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleZettaMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleZettaMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleYottaMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleZettaMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleYottaMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleZettaMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleYottaMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleYottaMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleRonnaMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleYottaMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleRonnaMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleYottaMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleRonnaMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleRonnaMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuettaMeter1::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleRonnaMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuettaMeter10::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleRonnaMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuettaMeter100::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuettaMeter1::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuettaMeter1000::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuettaMeter10::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuettaMeter10000::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuettaMeter100::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuettaMeter100000::default()))
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuettaMeter1000::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        None
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        Some(Box::new(ScaleQuettaMeter10000::default()))
    }
}

#[derive(Clone, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
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
    fn up(&self) -> Option<Box<dyn DynScale>> {
        None
    }
    fn down(&self) -> Option<Box<dyn DynScale>> {
        None
    }
}
