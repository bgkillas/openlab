use crate::units::{
    AmountUnit, AngelUnit, CurrencyUnit, CurrentUnit, DistanceUnit, LuminousIntensityUnit,
    MassUnit, TemperatureUnit, TimeUnit,
};
use std::path::Path;
#[derive(Clone, Copy)]
pub struct ParserOptions<'a> {
    pub path: Option<&'a Path>,
    pub units: UnitOptions,
}
#[derive(Clone, Copy, Default)]
pub struct UnitOptions {
    pub time: TimeUnit,
    pub distance: DistanceUnit,
    pub mass: MassUnit,
    pub current: CurrentUnit,
    pub temperature: TemperatureUnit,
    pub amount: AmountUnit,
    pub luminous_intensity: LuminousIntensityUnit,
    pub currency: CurrencyUnit,
    pub angel: AngelUnit,
}
impl<'a> ParserOptions<'a> {
    pub fn new(path: Option<&'a Path>) -> Self {
        Self {
            path,
            units: UnitOptions::default(),
        }
    }
}
