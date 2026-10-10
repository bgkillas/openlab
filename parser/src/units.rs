use crate::parser::{Float, NumberBase, NumberInner};
use enum_map::{Enum, EnumMap};
use number::float::real::Real;
use number::traits::complex::ComplexImpl as _;
use number::units::Units;
pub type UnitType = f16;
pub const UNIT_COUNT: usize = 8;
#[derive(Enum)]
pub enum Unit {
    Second = 0,
    Meter = 1,
    Gram = 2,
    Ampere = 3,
    Kelvin = 4,
    Mole = 5,
    Candela = 6,
    Usd = 7,
}
#[derive(Enum)]
pub enum Prefix {
    Quetta,
    Ronna,
    Yotta,
    Zetta,
    Exa,
    Peta,
    Tera,
    Giga,
    Mega,
    Kilo,
    Hecto,
    Deca,
    None,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
    Zepto,
    Yocto,
    Ronto,
    Quecto,
}
impl Prefix {
    pub fn multiplier(self) -> Float {
        match self {
            Self::Quetta => 1000000000000000000000000000000.0,
            Self::Ronna => 1000000000000000000000000000.0,
            Self::Yotta => 1000000000000000000000000.0,
            Self::Zetta => 1000000000000000000000.0,
            Self::Exa => 1000000000000000000.0,
            Self::Peta => 1000000000000000.0,
            Self::Tera => 1000000000000.0,
            Self::Giga => 1000000000.0,
            Self::Mega => 1000000.0,
            Self::Kilo => 1000.0,
            Self::Hecto => 100.0,
            Self::Deca => 10.0,
            Self::None => 1.0,
            Self::Deci => 0.1,
            Self::Centi => 0.01,
            Self::Milli => 0.001,
            Self::Micro => 0.000001,
            Self::Nano => 0.000000001,
            Self::Pico => 0.000000000001,
            Self::Femto => 0.000000000000001,
            Self::Atto => 0.000000000000000001,
            Self::Zepto => 0.000000000000000000001,
            Self::Yocto => 0.000000000000000000000001,
            Self::Ronto => 0.000000000000000000000000001,
            Self::Quecto => 0.000000000000000000000000000001,
        }
    }
    pub fn parse(s: &str) -> (Self, &str) {
        fn strip_or<'a>(s: &'a str, m1: &str, m2: &str) -> Option<&'a str> {
            s.strip_prefix(m1).or_else(|| s.strip_prefix(m2))
        }
        fn parse<'a, const N: usize>(
            s: &'a str,
            ms: [(Prefix, &str, &str); N],
        ) -> (Prefix, &'a str) {
            for (p, a, b) in ms {
                if let Some(t) = strip_or(s, a, b) {
                    return (p, t);
                }
            }
            (Prefix::None, s)
        }
        parse(
            s,
            [
                (Self::Quetta, "quetta", "Q"),
                (Self::Ronna, "ronna", "R"),
                (Self::Yotta, "yotta", "Y"),
                (Self::Zetta, "zetta", "Z"),
                (Self::Exa, "exa", "E"),
                (Self::Peta, "peta", "P"),
                (Self::Tera, "tera", "T"),
                (Self::Giga, "giga", "G"),
                (Self::Mega, "mega", "M"),
                (Self::Kilo, "kilo", "k"),
                (Self::Hecto, "hecto", "h"),
                (Self::Deca, "deca", "da"),
                (Self::Deci, "deci", "d"),
                (Self::Centi, "centi", "c"),
                (Self::Milli, "milli", "m"),
                (Self::Micro, "micro", "μ"),
                (Self::Nano, "nano", "n"),
                (Self::Pico, "pico", "p"),
                (Self::Femto, "femto", "f"),
                (Self::Atto, "atto", "a"),
                (Self::Zepto, "zepto", "z"),
                (Self::Yocto, "yocto", "y"),
                (Self::Ronto, "ronto", "r"),
                (Self::Quecto, "quecto", "q"),
            ],
        )
    }
}
impl Unit {
    pub fn parse(s: &str) -> Option<NumberInner> {
        let (prefix, postfix) = Prefix::parse(s);
        let mut units: EnumMap<Self, UnitType> = EnumMap::default();
        match postfix {
            "s" => units[Self::Second] = 1.0,
            "m" => units[Self::Meter] = 1.0,
            "g" => units[Self::Gram] = 1.0,
            "A" => units[Self::Ampere] = 1.0,
            "K" => units[Self::Kelvin] = 1.0,
            "mol" => units[Self::Mole] = 1.0,
            "cd" => units[Self::Candela] = 1.0,
            "USD" => units[Self::Usd] = 1.0,
            _ => return None,
        }
        let number = NumberBase::new_real(Real(prefix.multiplier()));
        let units_val = Units::from(units.into_array());
        Some(NumberInner::new(number, units_val))
    }
}
