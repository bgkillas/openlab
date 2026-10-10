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
            Self::Quetta => 1e30,
            Self::Ronna => 1e27,
            Self::Yotta => 1e24,
            Self::Zetta => 1e21,
            Self::Exa => 1e18,
            Self::Peta => 1e15,
            Self::Tera => 1e12,
            Self::Giga => 1e9,
            Self::Mega => 1e6,
            Self::Kilo => 1e3,
            Self::Hecto => 1e2,
            Self::Deca => 1e1,
            Self::None => 1e0,
            Self::Deci => 1e-1,
            Self::Centi => 1e-2,
            Self::Milli => 1e-3,
            Self::Micro => 1e-6,
            Self::Nano => 1e-9,
            Self::Pico => 1e-12,
            Self::Femto => 1e-15,
            Self::Atto => 1e-18,
            Self::Zepto => 1e-21,
            Self::Yocto => 1e-24,
            Self::Ronto => 1e-27,
            Self::Quecto => 1e-30,
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
