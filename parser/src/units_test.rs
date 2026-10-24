use crate::units::{BinaryPrefix, Length, MetricPrefix, Prefix, Unit, UnitBucket};
use core::mem::{transmute_copy, variant_count};
#[test]
#[cfg(not(miri))]
fn unit_conversion() {
    convert(Unit::TimeUnit);
    convert(Unit::DistanceUnit);
    convert(Unit::MassUnit);
    convert(Unit::CurrentUnit);
    convert(Unit::TemperatureUnit);
    convert(Unit::AmountUnit);
    convert(Unit::LuminousIntensityUnit);
    convert(Unit::CurrencyUnit);
    convert(Unit::AngleUnit);
}
#[cfg(not(miri))]
fn convert<E: UnitBucket, F: Fn(E) -> Unit>(f: F) {
    for e in iter::<E>() {
        for l in iter::<Length>() {
            assert_eq!(Some(e), E::parse(e.str(l)), "{}", e.str(l));
            assert_eq!(
                Some((Prefix::None, f(e))),
                Prefix::parse(e.str(l)),
                "{}",
                e.str(l)
            );
            for pl in iter::<Length>() {
                for p in iter::<BinaryPrefix>() {
                    assert_eq!(
                        Some((Prefix::Binary(p), f(e))),
                        Prefix::parse(format!("{}{}", p.str(pl), e.str(l)).as_str()),
                        "{}{}",
                        p.str(pl),
                        e.str(l)
                    );
                }
                for p in iter::<MetricPrefix>() {
                    assert_eq!(
                        Some((Prefix::Metric(p), f(e))),
                        Prefix::parse(format!("{}{}", p.str(pl), e.str(l)).as_str()),
                        "{}{}",
                        p.str(pl),
                        e.str(l)
                    );
                }
            }
        }
    }
}
#[cfg(not(miri))]
pub fn iter<E>() -> impl Iterator<Item = E> {
    (0..variant_count::<E>()).map(|i| unsafe { transmute_copy(&i) })
}
