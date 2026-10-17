use crate::units::{
    AmountUnit, AngleUnit, BinaryPrefix, CurrencyUnit, CurrentUnit, DistanceUnit,
    LuminousIntensityUnit, MassUnit, MetricPrefix, Prefix, TemperatureUnit, TimeUnit, UnitBucket,
};
use core::mem::{transmute_copy, variant_count};
#[test]
fn unit_conversion() {
    convert::<TimeUnit>();
    convert::<DistanceUnit>();
    convert::<MassUnit>();
    convert::<CurrentUnit>();
    convert::<TemperatureUnit>();
    convert::<AmountUnit>();
    convert::<LuminousIntensityUnit>();
    convert::<CurrencyUnit>();
    convert::<AngleUnit>();
}
fn convert<E: UnitBucket>() {
    for e in iter::<E>() {
        assert_eq!(Some(e), E::parse(e.to_short()));
        assert_eq!(Some(e), E::parse(e.to_long()));
        //assert_eq!(Some(e), Prefix::parse(e.to_short()));
        //assert_eq!(Some(e), Prefix::parse(e.to_long()));
        for e in iter::<BinaryPrefix>() {
            todo!()
        }
        for e in iter::<MetricPrefix>() {
            todo!()
        }
    }
}
pub fn iter<E>() -> impl Iterator<Item = E> {
    (0..variant_count::<E>()).map(|i| unsafe { transmute_copy(&i) })
}
