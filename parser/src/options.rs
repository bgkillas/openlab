use crate::parser::Float;
use crate::units::{
    AmountUnit, AngelUnit, CurrencyUnit, CurrentUnit, Dimension, DistanceUnit,
    LuminousIntensityUnit, MassUnit, TemperatureUnit, TimeUnit, UnitBucket as _,
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
    pub fn get(self, dimension: Dimension) -> Option<Float> {
        match dimension {
            Dimension::Time => self.units.time.get(self),
            Dimension::Distance => self.units.distance.get(self),
            Dimension::Mass => self.units.mass.get(self),
            Dimension::Current => self.units.current.get(self),
            Dimension::Temperature => self.units.temperature.get(self),
            Dimension::Amount => self.units.amount.get(self),
            Dimension::LuminousIntensity => self.units.luminous_intensity.get(self),
            Dimension::Currency => self.units.currency.get(self),
        }
    }
}
