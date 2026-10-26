use crate::options::ParserOptions;
use crate::parser::{Float, NumberBase, NumberInner};
use core::f64::consts::PI;
use core::fmt::Debug;
use number::float::real::Real;
use number::traits::complex::ComplexImpl as _;
use number::traits::number::NumberImpl as _;
use number::units::Units;
use parser_macros::units;
use reqwest::blocking;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead as _, BufReader, Write as _};
use std::mem::variant_count;
use std::sync::Mutex;
use std::time::SystemTime;
pub type UnitType = f32;
pub const UNIT_COUNT: usize = 8;
#[derive(Debug)]
#[repr(u8)]
pub enum Dimension {
    Time,
    Distance,
    Mass,
    Current,
    Temperature,
    Amount,
    LuminousIntensity,
    Currency,
    Angle,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    TimeUnit(TimeUnit),
    DistanceUnit(DistanceUnit),
    MassUnit(MassUnit),
    CurrentUnit(CurrentUnit),
    TemperatureUnit(TemperatureUnit),
    AmountUnit(AmountUnit),
    LuminousIntensityUnit(LuminousIntensityUnit),
    CurrencyUnit(CurrencyUnit),
    AngleUnit(AngleUnit),
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum TimeUnit {
    Planck,
    #[default]
    Second,
    Minute,
    Hour,
    Day,
    Week,
    ///year / 12
    Month,
    ///365.25 days
    Year,
    Decade,
    Century,
    Millennium,
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DistanceUnit {
    Planck,
    Inch,
    Foot,
    Yard,
    #[default]
    Meter,
    Mile,
    AstronomicalUnit,
    LightYear,
    Parsec,
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MassUnit {
    Planck,
    #[default]
    Gram,
    Ounce,
    Pound,
    Tonne,
    Solar,
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum CurrentUnit {
    #[default]
    Amperage,
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum TemperatureUnit {
    Planck,
    #[default]
    Kelvin,
    Rankine,
    Celsius,
    Fahrenheit,
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum AmountUnit {
    Unit,
    #[default]
    Mole,
    Bit,
    Byte,
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum LuminousIntensityUnit {
    #[default]
    Candela,
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(u8)]
#[rustfmt::skip]
pub enum CurrencyUnit {
    #[default]
    USD, AUD, CAD, CHF, CNY, EUR, GBP, HKD,
    IDR, INR, JPY, KRW, MYR, NZD, PGK, PHP,
    SGD, THB, TWD, VND, AED, QAR, SAR, DKK,
    EGP, ILS, JOD, NOK, SEK, ZAR, BRL, CLP,
    CZK, HUF, ISK, MXN, PLN, RON, TRY, UAH,
    MDL, RSD, RUB, AZN, BDT, DZD, GEL, KZT,
    TND, XAF, XOF, BYN, PKR, AFN, ALL, AMD,
    AOA, ARS, AWG, BAM, BBD, BHD, BIF, BND,
    BOB, BSD, BWP, BZD, CDF, COP, CRC, CUP,
    CVE, DJF, DOP, ERN, ETB, FJD, GHS, GIP,
    GMD, GNF, GTQ, GYD, HNL, HTG, IQD, JMD,
    KES, KGS, KHR, KMF, KWD, KYD, LBP, LKR,
    LRD, LSL, LYD, MAD, MGA, MKD, MMK, MNT,
    MOP, MRU, MUR, MVR, MWK, MZN, NAD, NGN,
    NIO, NPR, OMR, PAB, PEN, PYG, RWF, SBD,
    SCR, SDG, SOS, SRD, SSP, STN, SVC, SZL,
    TJS, TMT, TOP, TTD, TZS, UGX, UYU, UZS,
    VES, VUV, WST, XCD, XCG, XPF, YER, ZMW,
    IRR, LAK, SYP,
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum AngleUnit {
    #[default]
    Rad,
    Deg,
    Grad,
}
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Prefix {
    #[default]
    None,
    Metric(MetricPrefix),
    Binary(BinaryPrefix),
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MetricPrefix {
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
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum BinaryPrefix {
    Kibi,
    Mebi,
    Gibi,
    Tebi,
    Pebi,
    Exbi,
    Zebi,
    Yobi,
    Robi,
    Quebi,
}
impl From<u8> for Dimension {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Time,
            1 => Self::Distance,
            2 => Self::Mass,
            3 => Self::Current,
            4 => Self::Temperature,
            5 => Self::Amount,
            6 => Self::LuminousIntensity,
            7 => Self::Currency,
            _ => unreachable!(),
        }
    }
}
impl Prefix {
    pub fn get(self, options: ParserOptions<'_>) -> Option<Float> {
        match self {
            Self::None => Some(1.0),
            Self::Metric(m) => m.get(options),
            Self::Binary(b) => b.get(options),
        }
    }
    pub fn parse(s: &str) -> Option<(Self, Unit)> {
        if let Some((pre, post)) = BinaryPrefix::parse_prefix(s, |str| Unit::parse(str).is_some())
            && let Some(unit) = Unit::parse(post)
        {
            Some((Self::Binary(pre), unit))
        } else if let Some((pre, post)) =
            MetricPrefix::parse_prefix(s, |str| Unit::parse(str).is_some())
            && let Some(unit) = Unit::parse(post)
        {
            Some((Self::Metric(pre), unit))
        } else {
            Unit::parse(s).map(|u| (Self::None, u))
        }
    }
}
fn prefix_parse<T, F, const N: usize>(s: &str, f: F, ms: [T; N]) -> Option<(T, &str)>
where
    T: UnitBucket,
    F: Fn(&str) -> bool,
{
    for p in ms {
        if let Some(t) = s.strip_prefix(p.to_long())
            && f(t)
        {
            return Some((p, t));
        }
    }
    for p in ms {
        if let Some(t) = s.strip_prefix(p.to_short())
            && f(t)
        {
            return Some((p, t));
        }
    }
    None
}
impl UnitBucket for MetricPrefix {
    units!(
        "quetta" | "Q" => Self::Quetta,
        "ronna" | "R" => Self::Ronna,
        "yotta" | "Y" => Self::Yotta,
        "zetta" | "Z" => Self::Zetta,
        "exa" | "E" => Self::Exa,
        "peta" | "P" => Self::Peta,
        "tera" | "T" => Self::Tera,
        "giga" | "G" => Self::Giga,
        "mega" | "M" => Self::Mega,
        "kilo" | "k" => Self::Kilo,
        "hecto" | "h" => Self::Hecto,
        "deca" | "da" => Self::Deca,
        "deci" | "d" => Self::Deci,
        "centi" | "c" => Self::Centi,
        "milli" | "m" => Self::Milli,
        "micro" | "μ" => Self::Micro,
        "nano" | "n" => Self::Nano,
        "pico" | "p" => Self::Pico,
        "femto" | "f" => Self::Femto,
        "atto" | "a" => Self::Atto,
        "zepto" | "z" => Self::Zepto,
        "yocto" | "y" => Self::Yocto,
        "ronto" | "r" => Self::Ronto,
        "quecto" | "q" => Self::Quecto,
    );
    fn units(self) -> Units<UnitType, UNIT_COUNT> {
        Units::default()
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
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
        })
    }
}
impl MetricPrefix {
    pub fn parse_prefix<F>(s: &str, f: F) -> Option<(Self, &str)>
    where
        F: Fn(&str) -> bool,
    {
        prefix_parse(
            s,
            f,
            [
                Self::Quetta,
                Self::Ronna,
                Self::Yotta,
                Self::Zetta,
                Self::Exa,
                Self::Peta,
                Self::Tera,
                Self::Giga,
                Self::Mega,
                Self::Kilo,
                Self::Hecto,
                Self::Deca,
                Self::Deci,
                Self::Centi,
                Self::Milli,
                Self::Micro,
                Self::Nano,
                Self::Pico,
                Self::Femto,
                Self::Atto,
                Self::Zepto,
                Self::Yocto,
                Self::Ronto,
                Self::Quecto,
            ],
        )
    }
}
impl UnitBucket for BinaryPrefix {
    units!(
        "kibi" | "Ki" => Self::Kibi,
        "mebi" | "Mi" => Self::Mebi,
        "gibi" | "Gi" => Self::Gibi,
        "tebi" | "Ti" => Self::Tebi,
        "pebi" | "Pi" => Self::Pebi,
        "exbi" | "Ei" => Self::Exbi,
        "zebi" | "Zi" => Self::Zebi,
        "yobi" | "Yi" => Self::Yobi,
        "robi" | "Ri" => Self::Robi,
        "quebi" | "Qi" => Self::Quebi,
    );
    fn units(self) -> Units<UnitType, UNIT_COUNT> {
        Units::default()
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Kibi => Float::powi(1024.0, 1),
            Self::Mebi => Float::powi(1024.0, 2),
            Self::Gibi => Float::powi(1024.0, 3),
            Self::Tebi => Float::powi(1024.0, 4),
            Self::Pebi => Float::powi(1024.0, 5),
            Self::Exbi => Float::powi(1024.0, 6),
            Self::Zebi => Float::powi(1024.0, 7),
            Self::Yobi => Float::powi(1024.0, 8),
            Self::Robi => Float::powi(1024.0, 9),
            Self::Quebi => Float::powi(1024.0, 10),
        })
    }
}
impl BinaryPrefix {
    pub fn parse_prefix<F>(s: &str, f: F) -> Option<(Self, &str)>
    where
        F: Fn(&str) -> bool,
    {
        prefix_parse(
            s,
            f,
            [
                Self::Kibi,
                Self::Mebi,
                Self::Gibi,
                Self::Tebi,
                Self::Pebi,
                Self::Exbi,
                Self::Zebi,
                Self::Yobi,
                Self::Robi,
                Self::Quebi,
            ],
        )
    }
}
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Length {
    Long,
    Short,
}
pub trait UnitBucket: Copy + PartialEq + Debug {
    fn parse(s: &str) -> Option<Self>;
    fn units(self) -> Units<UnitType, UNIT_COUNT>;
    fn get(self, options: ParserOptions<'_>) -> Option<Float>;
    fn get_scaled(self, options: ParserOptions<'_>) -> Option<Float> {
        let mut float = self.get(options)?;
        let units = self.units();
        for (i, &u) in units.units.as_array().iter().enumerate() {
            float /= options
                .get(Dimension::from(u8::try_from(i).unwrap()))?
                .powf(Float::from(u));
        }
        Some(float)
    }
    fn to_short(self) -> &'static str;
    fn to_long(self) -> &'static str;
    fn str(self, length: Length) -> &'static str {
        match length {
            Length::Long => self.to_long(),
            Length::Short => self.to_short(),
        }
    }
}
impl UnitBucket for Unit {
    fn parse(s: &str) -> Option<Self> {
        macro_rules! run {
            ($($ident:ident,)*) => {
                $(
                    if let Some(unit) = $ident::parse(s) {
                        return Some(Self::$ident(unit));
                    }
                )*
            };
        }
        run!(
            TimeUnit,
            DistanceUnit,
            MassUnit,
            CurrentUnit,
            TemperatureUnit,
            AmountUnit,
            LuminousIntensityUnit,
            CurrencyUnit,
            AngleUnit,
        );
        None
    }
    fn units(self) -> Units<UnitType, UNIT_COUNT> {
        match self {
            Self::TimeUnit(u) => u.units(),
            Self::DistanceUnit(u) => u.units(),
            Self::MassUnit(u) => u.units(),
            Self::CurrentUnit(u) => u.units(),
            Self::TemperatureUnit(u) => u.units(),
            Self::AmountUnit(u) => u.units(),
            Self::LuminousIntensityUnit(u) => u.units(),
            Self::CurrencyUnit(u) => u.units(),
            Self::AngleUnit(u) => u.units(),
        }
    }
    fn get(self, options: ParserOptions<'_>) -> Option<Float> {
        match self {
            Self::TimeUnit(u) => u.get(options),
            Self::DistanceUnit(u) => u.get(options),
            Self::MassUnit(u) => u.get(options),
            Self::CurrentUnit(u) => u.get(options),
            Self::TemperatureUnit(u) => u.get(options),
            Self::AmountUnit(u) => u.get(options),
            Self::LuminousIntensityUnit(u) => u.get(options),
            Self::CurrencyUnit(u) => u.get(options),
            Self::AngleUnit(u) => u.get(options),
        }
    }
    fn get_scaled(self, options: ParserOptions<'_>) -> Option<Float> {
        match self {
            Self::TimeUnit(u) => u.get_scaled(options),
            Self::DistanceUnit(u) => u.get_scaled(options),
            Self::MassUnit(u) => u.get_scaled(options),
            Self::CurrentUnit(u) => u.get_scaled(options),
            Self::TemperatureUnit(u) => u.get_scaled(options),
            Self::AmountUnit(u) => u.get_scaled(options),
            Self::LuminousIntensityUnit(u) => u.get_scaled(options),
            Self::CurrencyUnit(u) => u.get_scaled(options),
            Self::AngleUnit(u) => u.get_scaled(options),
        }
    }
    fn to_short(self) -> &'static str {
        match self {
            Self::TimeUnit(u) => u.to_short(),
            Self::DistanceUnit(u) => u.to_short(),
            Self::MassUnit(u) => u.to_short(),
            Self::CurrentUnit(u) => u.to_short(),
            Self::TemperatureUnit(u) => u.to_short(),
            Self::AmountUnit(u) => u.to_short(),
            Self::LuminousIntensityUnit(u) => u.to_short(),
            Self::CurrencyUnit(u) => u.to_short(),
            Self::AngleUnit(u) => u.to_short(),
        }
    }
    fn to_long(self) -> &'static str {
        match self {
            Self::TimeUnit(u) => u.to_long(),
            Self::DistanceUnit(u) => u.to_long(),
            Self::MassUnit(u) => u.to_long(),
            Self::CurrentUnit(u) => u.to_long(),
            Self::TemperatureUnit(u) => u.to_long(),
            Self::AmountUnit(u) => u.to_long(),
            Self::LuminousIntensityUnit(u) => u.to_long(),
            Self::CurrencyUnit(u) => u.to_long(),
            Self::AngleUnit(u) => u.to_long(),
        }
    }
}
impl UnitBucket for TimeUnit {
    units!(
        "planck_time" | "t_P" => Self::Planck,
        "second" | "s" => Self::Second,
        "minute" => Self::Minute,
        "hour" => Self::Hour,
        "day" => Self::Day,
        "week" => Self::Week,
        "month" => Self::Month,
        "year" => Self::Year,
        "decade" => Self::Decade,
        "century" => Self::Century,
        "millennium" => Self::Millennium,
    );
    fn units(self) -> Units<UnitType, UNIT_COUNT> {
        let mut simd = Units::<UnitType, UNIT_COUNT>::default();
        let arr = simd.units.as_mut_array();
        arr[Dimension::Time.discriminant()] = 1.0;
        simd
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Planck => 5.39124760e-44,
            Self::Second => 1.0,
            Self::Minute => 60.0,
            Self::Hour => 60.0 * 60.0,
            Self::Day => 24.0 * 60.0 * 60.0,
            Self::Week => 7.0 * 24.0 * 60.0 * 60.0,
            Self::Month => 365.25 / 12.0 * 24.0 * 60.0 * 60.0,
            Self::Year => 365.25 * 24.0 * 60.0 * 60.0,
            Self::Decade => 10.0 * 365.25 * 24.0 * 60.0 * 60.0,
            Self::Century => 100.0 * 365.25 * 24.0 * 60.0 * 60.0,
            Self::Millennium => 1000.0 * 365.25 * 24.0 * 60.0 * 60.0,
        })
    }
}
impl UnitBucket for DistanceUnit {
    units!(
        "planck_length" | "l_P" => Self::Planck,
        "inch" | "in" => Self::Inch,
        "foot" | "ft" => Self::Foot,
        "yard" | "yd" => Self::Yard,
        "meter" | "m" => Self::Meter,
        "mile" | "mi" => Self::Mile,
        "astronomical_unit" | "au" => Self::AstronomicalUnit,
        "light_year" | "ly" => Self::LightYear,
        "parsec" | "pc" => Self::Parsec,
    );
    fn units(self) -> Units<UnitType, UNIT_COUNT> {
        let mut simd = Units::<UnitType, UNIT_COUNT>::default();
        let arr = simd.units.as_mut_array();
        arr[Dimension::Distance.discriminant()] = 1.0;
        simd
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Planck => 1.61625518e-35,
            Self::Inch => 0.0254,
            Self::Foot => 0.0254 * 12.0,
            Self::Yard => 0.0254 * 12.0 * 3.0,
            Self::Meter => 1.0,
            Self::Mile => 0.0254 * 63360.0,
            Self::AstronomicalUnit => 149597870700.0,
            Self::LightYear => 9460730472580800.0,
            Self::Parsec => 149597870700.0 * 648000.0 / PI,
        })
    }
}
impl UnitBucket for MassUnit {
    units!(
        "planck_mass" | "m_P" => Self::Planck,
        "gram" | "g" => Self::Gram,
        "ounce" | "oz" => Self::Ounce,
        "pound" | "lb" => Self::Pound,
        "tonne" => Self::Tonne,
        "solar_mass" => Self::Solar,
    );
    fn units(self) -> Units<UnitType, UNIT_COUNT> {
        let mut simd = Units::<UnitType, UNIT_COUNT>::default();
        let arr = simd.units.as_mut_array();
        arr[Dimension::Mass.discriminant()] = 1.0;
        simd
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Planck => 2.17643424e-8,
            Self::Gram => 1.0,
            Self::Ounce => 28.349523125,
            Self::Pound => 28.349523125 * 16.0,
            Self::Tonne => 1000.0 * 1000.0,
            Self::Solar => 1.988416e33,
        })
    }
}
impl UnitBucket for CurrentUnit {
    units!(
        "ampere" | "A" => Self::Amperage,
    );
    fn units(self) -> Units<UnitType, UNIT_COUNT> {
        let mut simd = Units::<UnitType, UNIT_COUNT>::default();
        let arr = simd.units.as_mut_array();
        arr[Dimension::Current.discriminant()] = 1.0;
        simd
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Amperage => 1.0,
        })
    }
}
impl UnitBucket for TemperatureUnit {
    units!(
        "planck_temperature" | "T_P" => Self::Planck,
        "kelvin" | "K" => Self::Kelvin,
        "rankine" | "R" => Self::Rankine,
        "celsius" | "C" => Self::Celsius,
        "fahrenheit" | "F" => Self::Fahrenheit,
    );
    fn units(self) -> Units<UnitType, UNIT_COUNT> {
        let mut simd = Units::<UnitType, UNIT_COUNT>::default();
        let arr = simd.units.as_mut_array();
        arr[Dimension::Temperature.discriminant()] = 1.0;
        simd
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Planck => 1.41678416e32,
            //TODO convert
            Self::Kelvin | Self::Celsius => 1.0,
            Self::Rankine | Self::Fahrenheit => 5.0 / 9.0,
        })
    }
}
impl UnitBucket for AmountUnit {
    units!(
        "unit" => Self::Unit,
        "mole" | "mol" => Self::Mole,
        "bit" | "b" => Self::Bit,
        "byte" | "B" => Self::Byte,
    );
    fn units(self) -> Units<UnitType, UNIT_COUNT> {
        let mut simd = Units::<UnitType, UNIT_COUNT>::default();
        let arr = simd.units.as_mut_array();
        arr[Dimension::Amount.discriminant()] = 1.0;
        simd
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Unit | Self::Mole | Self::Bit => 1.0,
            Self::Byte => 8.0,
        })
    }
}
impl UnitBucket for LuminousIntensityUnit {
    units!(
        "candela" | "cd" => Self::Candela,
    );
    fn units(self) -> Units<UnitType, UNIT_COUNT> {
        let mut simd = Units::<UnitType, UNIT_COUNT>::default();
        let arr = simd.units.as_mut_array();
        arr[Dimension::LuminousIntensity.discriminant()] = 1.0;
        simd
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Candela => 1.0,
        })
    }
}
impl UnitBucket for CurrencyUnit {
    #[rustfmt::skip]
    fn parse(s: &str) -> Option<Self> {
        Some(match s.to_ascii_uppercase().as_str() {
            "USD" => Self::USD, "AUD" => Self::AUD, "CAD" => Self::CAD, "CHF" => Self::CHF, "CNY" => Self::CNY, "EUR" => Self::EUR, "GBP" => Self::GBP, "HKD" => Self::HKD,
            "IDR" => Self::IDR, "INR" => Self::INR, "JPY" => Self::JPY, "KRW" => Self::KRW, "MYR" => Self::MYR, "NZD" => Self::NZD, "PGK" => Self::PGK, "PHP" => Self::PHP,
            "SGD" => Self::SGD, "THB" => Self::THB, "TWD" => Self::TWD, "VND" => Self::VND, "AED" => Self::AED, "QAR" => Self::QAR, "SAR" => Self::SAR, "DKK" => Self::DKK,
            "EGP" => Self::EGP, "ILS" => Self::ILS, "JOD" => Self::JOD, "NOK" => Self::NOK, "SEK" => Self::SEK, "ZAR" => Self::ZAR, "BRL" => Self::BRL, "CLP" => Self::CLP,
            "CZK" => Self::CZK, "HUF" => Self::HUF, "ISK" => Self::ISK, "MXN" => Self::MXN, "PLN" => Self::PLN, "RON" => Self::RON, "TRY" => Self::TRY, "UAH" => Self::UAH,
            "MDL" => Self::MDL, "RSD" => Self::RSD, "RUB" => Self::RUB, "AZN" => Self::AZN, "BDT" => Self::BDT, "DZD" => Self::DZD, "GEL" => Self::GEL, "KZT" => Self::KZT,
            "TND" => Self::TND, "XAF" => Self::XAF, "XOF" => Self::XOF, "BYN" => Self::BYN, "PKR" => Self::PKR, "AFN" => Self::AFN, "ALL" => Self::ALL, "AMD" => Self::AMD,
            "AOA" => Self::AOA, "ARS" => Self::ARS, "AWG" => Self::AWG, "BAM" => Self::BAM, "BBD" => Self::BBD, "BHD" => Self::BHD, "BIF" => Self::BIF, "BND" => Self::BND,
            "BOB" => Self::BOB, "BSD" => Self::BSD, "BWP" => Self::BWP, "BZD" => Self::BZD, "CDF" => Self::CDF, "COP" => Self::COP, "CRC" => Self::CRC, "CUP" => Self::CUP,
            "CVE" => Self::CVE, "DJF" => Self::DJF, "DOP" => Self::DOP, "ERN" => Self::ERN, "ETB" => Self::ETB, "FJD" => Self::FJD, "GHS" => Self::GHS, "GIP" => Self::GIP,
            "GMD" => Self::GMD, "GNF" => Self::GNF, "GTQ" => Self::GTQ, "GYD" => Self::GYD, "HNL" => Self::HNL, "HTG" => Self::HTG, "IQD" => Self::IQD, "JMD" => Self::JMD,
            "KES" => Self::KES, "KGS" => Self::KGS, "KHR" => Self::KHR, "KMF" => Self::KMF, "KWD" => Self::KWD, "KYD" => Self::KYD, "LBP" => Self::LBP, "LKR" => Self::LKR,
            "LRD" => Self::LRD, "LSL" => Self::LSL, "LYD" => Self::LYD, "MAD" => Self::MAD, "MGA" => Self::MGA, "MKD" => Self::MKD, "MMK" => Self::MMK, "MNT" => Self::MNT,
            "MOP" => Self::MOP, "MRU" => Self::MRU, "MUR" => Self::MUR, "MVR" => Self::MVR, "MWK" => Self::MWK, "MZN" => Self::MZN, "NAD" => Self::NAD, "NGN" => Self::NGN,
            "NIO" => Self::NIO, "NPR" => Self::NPR, "OMR" => Self::OMR, "PAB" => Self::PAB, "PEN" => Self::PEN, "PYG" => Self::PYG, "RWF" => Self::RWF, "SBD" => Self::SBD,
            "SCR" => Self::SCR, "SDG" => Self::SDG, "SOS" => Self::SOS, "SRD" => Self::SRD, "SSP" => Self::SSP, "STN" => Self::STN, "SVC" => Self::SVC, "SZL" => Self::SZL,
            "TJS" => Self::TJS, "TMT" => Self::TMT, "TOP" => Self::TOP, "TTD" => Self::TTD, "TZS" => Self::TZS, "UGX" => Self::UGX, "UYU" => Self::UYU, "UZS" => Self::UZS,
            "VES" => Self::VES, "VUV" => Self::VUV, "WST" => Self::WST, "XCD" => Self::XCD, "XCG" => Self::XCG, "XPF" => Self::XPF, "YER" => Self::YER, "ZMW" => Self::ZMW,
            "IRR" => Self::IRR, "LAK" => Self::LAK, "SYP" => Self::SYP,
            _ => return None,
        })
    }
    fn units(self) -> Units<UnitType, UNIT_COUNT> {
        let mut simd = Units::<UnitType, UNIT_COUNT>::default();
        let arr = simd.units.as_mut_array();
        arr[Dimension::Currency.discriminant()] = 1.0;
        simd
    }
    fn get(self, options: ParserOptions<'_>) -> Option<Float> {
        if let Some(currency) = CURRENCY.lock().unwrap().as_ref() {
            return Some(currency[self.discriminant()]);
        }
        let mut arr = [Float::one(); variant_count::<Self>()];
        if let Some(dir) = options.path {
            let path = dir.join("currency.csv");
            if fs::metadata(&path).map_or(true, |a| {
                if let Some(n) = a
                    .modified()
                    .map(|m| SystemTime::now().duration_since(m).ok())
                    .ok()
                    .flatten()
                {
                    n.as_secs() > 7 * 24 * 3600
                } else {
                    true
                }
            }) {
                fs::create_dir_all(dir).ok()?;
                let mut file = OpenOptions::new()
                    .read(false)
                    .write(true)
                    .truncate(true)
                    .create(true)
                    .open(&path)
                    .ok()?;
                _ = file.write("USD,1\n".as_bytes());
                if download_currency_rates(|code_temp, rate| {
                    if let Some(unit) = Self::parse(code_temp) {
                        _ = file.write(format!("{code_temp},{rate}\n").as_bytes());
                        arr[unit.discriminant()] = rate;
                    }
                })
                .is_none()
                {
                    fs::remove_file(path).unwrap();
                    return None;
                }
            } else {
                let file = OpenOptions::new()
                    .read(true)
                    .write(false)
                    .truncate(false)
                    .create(false)
                    .open(path)
                    .ok()?;
                let reader = BufReader::new(file);
                for line in reader.lines().map_while(Result::ok) {
                    let Some((code_temp, rate)) = line.split_once(',') else {
                        continue;
                    };
                    if let Some(unit) = Self::parse(code_temp) {
                        arr[unit.discriminant()] = rate.parse().ok()?;
                    }
                }
            }
        } else {
            download_currency_rates(|code_temp, rate| {
                if let Some(unit) = Self::parse(code_temp) {
                    arr[unit.discriminant()] = rate;
                }
            })?;
        }
        *CURRENCY.lock().unwrap() = Some(arr);
        if let Some(currency) = CURRENCY.lock().unwrap().as_ref() {
            return Some(currency[self.discriminant()]);
        }
        None
    }
    #[rustfmt::skip]
    fn to_short(self) -> &'static str {
        match self {
            Self::USD => "USD", Self::AUD => "AUD", Self::CAD => "CAD", Self::CHF => "CHF", Self::CNY => "CNY", Self::EUR => "EUR", Self::GBP => "GBP", Self::HKD => "HKD",
            Self::IDR => "IDR", Self::INR => "INR", Self::JPY => "JPY", Self::KRW => "KRW", Self::MYR => "MYR", Self::NZD => "NZD", Self::PGK => "PGK", Self::PHP => "PHP",
            Self::SGD => "SGD", Self::THB => "THB", Self::TWD => "TWD", Self::VND => "VND", Self::AED => "AED", Self::QAR => "QAR", Self::SAR => "SAR", Self::DKK => "DKK",
            Self::EGP => "EGP", Self::ILS => "ILS", Self::JOD => "JOD", Self::NOK => "NOK", Self::SEK => "SEK", Self::ZAR => "ZAR", Self::BRL => "BRL", Self::CLP => "CLP",
            Self::CZK => "CZK", Self::HUF => "HUF", Self::ISK => "ISK", Self::MXN => "MXN", Self::PLN => "PLN", Self::RON => "RON", Self::TRY => "TRY", Self::UAH => "UAH",
            Self::MDL => "MDL", Self::RSD => "RSD", Self::RUB => "RUB", Self::AZN => "AZN", Self::BDT => "BDT", Self::DZD => "DZD", Self::GEL => "GEL", Self::KZT => "KZT",
            Self::TND => "TND", Self::XAF => "XAF", Self::XOF => "XOF", Self::BYN => "BYN", Self::PKR => "PKR", Self::AFN => "AFN", Self::ALL => "ALL", Self::AMD => "AMD",
            Self::AOA => "AOA", Self::ARS => "ARS", Self::AWG => "AWG", Self::BAM => "BAM", Self::BBD => "BBD", Self::BHD => "BHD", Self::BIF => "BIF", Self::BND => "BND",
            Self::BOB => "BOB", Self::BSD => "BSD", Self::BWP => "BWP", Self::BZD => "BZD", Self::CDF => "CDF", Self::COP => "COP", Self::CRC => "CRC", Self::CUP => "CUP",
            Self::CVE => "CVE", Self::DJF => "DJF", Self::DOP => "DOP", Self::ERN => "ERN", Self::ETB => "ETB", Self::FJD => "FJD", Self::GHS => "GHS", Self::GIP => "GIP",
            Self::GMD => "GMD", Self::GNF => "GNF", Self::GTQ => "GTQ", Self::GYD => "GYD", Self::HNL => "HNL", Self::HTG => "HTG", Self::IQD => "IQD", Self::JMD => "JMD",
            Self::KES => "KES", Self::KGS => "KGS", Self::KHR => "KHR", Self::KMF => "KMF", Self::KWD => "KWD", Self::KYD => "KYD", Self::LBP => "LBP", Self::LKR => "LKR",
            Self::LRD => "LRD", Self::LSL => "LSL", Self::LYD => "LYD", Self::MAD => "MAD", Self::MGA => "MGA", Self::MKD => "MKD", Self::MMK => "MMK", Self::MNT => "MNT",
            Self::MOP => "MOP", Self::MRU => "MRU", Self::MUR => "MUR", Self::MVR => "MVR", Self::MWK => "MWK", Self::MZN => "MZN", Self::NAD => "NAD", Self::NGN => "NGN",
            Self::NIO => "NIO", Self::NPR => "NPR", Self::OMR => "OMR", Self::PAB => "PAB", Self::PEN => "PEN", Self::PYG => "PYG", Self::RWF => "RWF", Self::SBD => "SBD",
            Self::SCR => "SCR", Self::SDG => "SDG", Self::SOS => "SOS", Self::SRD => "SRD", Self::SSP => "SSP", Self::STN => "STN", Self::SVC => "SVC", Self::SZL => "SZL",
            Self::TJS => "TJS", Self::TMT => "TMT", Self::TOP => "TOP", Self::TTD => "TTD", Self::TZS => "TZS", Self::UGX => "UGX", Self::UYU => "UYU", Self::UZS => "UZS",
            Self::VES => "VES", Self::VUV => "VUV", Self::WST => "WST", Self::XCD => "XCD", Self::XCG => "XCG", Self::XPF => "XPF", Self::YER => "YER", Self::ZMW => "ZMW",
            Self::IRR => "IRR", Self::LAK => "LAK", Self::SYP => "SYP",
        }
    }
    fn to_long(self) -> &'static str {
        Self::to_short(self)
    }
}
impl UnitBucket for AngleUnit {
    units!(
        "radian" | "rad" => Self::Rad,
        "degree" | "deg" => Self::Deg,
        "gradian" | "grad" => Self::Grad,
    );
    fn units(self) -> Units<UnitType, UNIT_COUNT> {
        Units::default()
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Rad => 1.0,
            Self::Deg => PI / 180.0,
            Self::Grad => PI / 200.0,
        })
    }
    fn get_scaled(self, options: ParserOptions<'_>) -> Option<Float> {
        let mut float = self.get(options)?;
        float /= options.get(Dimension::Angle)?;
        Some(float)
    }
}
impl Dimension {
    pub fn discriminant(self) -> usize {
        unsafe { core::mem::transmute::<Self, u8>(self) }.strict_cast()
    }
    pub fn parse(options: ParserOptions<'_>, s: &str) -> Option<NumberInner> {
        let (prefix, unit) = Prefix::parse(s)?;
        Self::number_inner(options, prefix, unit)
    }
    pub fn number_inner(
        options: ParserOptions<'_>,
        prefix: Prefix,
        unit: Unit,
    ) -> Option<NumberInner> {
        let units = unit.units();
        let mult = unit.get_scaled(options)?;
        let number = NumberBase::new_real(Real(mult * prefix.get(options)?));
        Some(NumberInner::new(number, units))
    }
}
static CURRENCY: Mutex<Option<[Float; variant_count::<CurrencyUnit>()]>> = Mutex::new(None);
impl CurrencyUnit {
    fn discriminant(self) -> usize {
        unsafe { core::mem::transmute::<Self, u8>(self) }.strict_cast()
    }
}
fn download_currency_rates<F>(mut f: F) -> Option<()>
where
    F: FnMut(&str, f64),
{
    let request = blocking::get("https://www.floatrates.com/daily/usd.json").ok()?;
    let bytes = request.text().ok()?;
    let json = jzon::parse(&bytes).ok()?;
    for (_, currency) in json.entries() {
        f(
            currency["code"].as_str()?,
            currency["inverseRate"].as_str()?.parse().ok()?,
        );
    }
    Some(())
}
