use crate::options::ParserOptions;
use crate::parser::{Float, NumberBase, NumberInner};
use core::f64::consts::PI;
use number::float::real::Real;
use number::traits::complex::ComplexImpl as _;
use number::traits::number::NumberImpl as _;
use number::units::Units;
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
pub enum Dimensions {
    Time,
    Distance,
    Mass,
    Current,
    Temperature,
    Amount,
    LuminousIntensity,
    Currency,
}
#[derive(Debug)]
pub enum Unit {
    TimeUnit(TimeUnit),
    DistanceUnit(DistanceUnit),
    MassUnit(MassUnit),
    CurrentUnit(CurrentUnit),
    TemperatureUnit(TemperatureUnit),
    AmountUnit(AmountUnit),
    LuminousIntensityUnit(LuminousIntensityUnit),
    CurrencyUnit(CurrencyUnit),
    AngelUnit(AngelUnit),
}
#[derive(Debug, Default, Clone, Copy)]
pub enum TimeUnit {
    Planck,
    #[default]
    Second,
    Minute,
    Hour,
    Day,
    Week,
    ///30 days
    Month,
    ///365.25 days
    Year,
    Decade,
    Century,
    Millennium,
}
#[derive(Debug, Default, Clone, Copy)]
pub enum DistanceUnit {
    Planck,
    Inch,
    Foot,
    Yard,
    #[default]
    Metre,
    Mile,
    AstronomicalUnit,
    LightYear,
    Parsec,
}
#[derive(Debug, Default, Clone, Copy)]
pub enum MassUnit {
    ElectronVolt,
    Planck,
    #[default]
    Gram,
    Ounce,
    Pound,
    Tonne,
    Solar,
}
#[derive(Debug, Default, Clone, Copy)]
pub enum CurrentUnit {
    #[default]
    Amperage,
}
#[derive(Debug, Default, Clone, Copy)]
pub enum TemperatureUnit {
    #[default]
    Kelvin,
    Rankine,
    Celsius,
    Fahrenheit,
}
#[derive(Debug, Default, Clone, Copy)]
pub enum AmountUnit {
    Unit,
    #[default]
    Mole,
    Bit,
    Byte,
}
#[derive(Debug, Default, Clone, Copy)]
pub enum LuminousIntensityUnit {
    #[default]
    Candela,
}
#[derive(Debug, Default, Clone, Copy)]
#[repr(u8)]
pub enum CurrencyUnit {
    #[default]
    USD,
    AUD,
    CAD,
    CHF,
    CNY,
    EUR,
    GBP,
    HKD,
    IDR,
    INR,
    JPY,
    KRW,
    MYR,
    NZD,
    PGK,
    PHP,
    SGD,
    THB,
    TWD,
    VND,
    AED,
    QAR,
    SAR,
    DKK,
    EGP,
    ILS,
    JOD,
    NOK,
    SEK,
    ZAR,
    BRL,
    CLP,
    CZK,
    HUF,
    ISK,
    MXN,
    PLN,
    RON,
    TRY,
    UAH,
    MDL,
    RSD,
    RUB,
    AZN,
    BDT,
    DZD,
    GEL,
    KZT,
    TND,
    XAF,
    XOF,
    BYN,
    PKR,
    AFN,
    ALL,
    AMD,
    AOA,
    ARS,
    AWG,
    BAM,
    BBD,
    BHD,
    BIF,
    BND,
    BOB,
    BSD,
    BWP,
    BZD,
    CDF,
    COP,
    CRC,
    CUP,
    CVE,
    DJF,
    DOP,
    ERN,
    ETB,
    FJD,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HNL,
    HTG,
    IQD,
    JMD,
    KES,
    KGS,
    KHR,
    KMF,
    KWD,
    KYD,
    LBP,
    LKR,
    LRD,
    LSL,
    LYD,
    MAD,
    MGA,
    MKD,
    MMK,
    MNT,
    MOP,
    MRU,
    MUR,
    MVR,
    MWK,
    MZN,
    NAD,
    NGN,
    NIO,
    NPR,
    OMR,
    PAB,
    PEN,
    PYG,
    RWF,
    SBD,
    SCR,
    SDG,
    SOS,
    SRD,
    SSP,
    STN,
    SVC,
    SZL,
    TJS,
    TMT,
    TOP,
    TTD,
    TZS,
    UGX,
    UYU,
    UZS,
    VES,
    VUV,
    WST,
    XCD,
    XCG,
    XPF,
    YER,
    ZMW,
    IRR,
    LAK,
    SYP,
}
#[derive(Debug, Default, Clone, Copy)]
pub enum AngelUnit {
    #[default]
    Rad,
    Deg,
    Grad,
}
#[derive(Debug)]
pub enum Prefix {
    None,
    Metric(MetricPrefix),
    Binary(BinaryPrefix),
}
#[derive(Debug)]
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
#[derive(Debug)]
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
impl Prefix {
    pub fn multiplier(self) -> Float {
        match self {
            Self::None => 1.0,
            Self::Metric(m) => m.multiplier(),
            Self::Binary(b) => b.multiplier(),
        }
    }
    pub fn parse(s: &str) -> (Self, &str) {
        if let Some((pre, post)) = BinaryPrefix::parse(s) {
            (Self::Binary(pre), post)
        } else if let Some((pre, post)) = MetricPrefix::parse(s) {
            (Self::Metric(pre), post)
        } else {
            (Self::None, s)
        }
    }
}
fn strip_or<'a>(s: &'a str, m1: &str, m2: &str) -> Option<&'a str> {
    s.strip_prefix(m1).or_else(|| s.strip_prefix(m2))
}
fn prefix_parse<'a, T, const N: usize>(
    s: &'a str,
    ms: [(T, &str, &str); N],
) -> Option<(T, &'a str)> {
    for (p, a, b) in ms {
        if let Some(t) = strip_or(s, a, b)
            && !t.is_empty()
        {
            return Some((p, t));
        }
    }
    None
}
impl MetricPrefix {
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
    pub fn parse(s: &str) -> Option<(Self, &str)> {
        prefix_parse(
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
impl BinaryPrefix {
    pub fn multiplier(self) -> Float {
        match self {
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
        }
    }
    pub fn parse(s: &str) -> Option<(Self, &str)> {
        prefix_parse(
            s,
            [
                (Self::Kibi, "kibi", "Ki"),
                (Self::Mebi, "mebi", "Mi"),
                (Self::Gibi, "gibi", "Gi"),
                (Self::Tebi, "tebi", "Ti"),
                (Self::Pebi, "pebi", "Pi"),
                (Self::Exbi, "exbi", "Ei"),
                (Self::Zebi, "zebi", "Zi"),
                (Self::Yobi, "yobi", "Yi"),
                (Self::Robi, "robi", "Ri"),
                (Self::Quebi, "quebi", "Qi"),
            ],
        )
    }
}
pub trait UnitBucket: Sized {
    fn parse(s: &str) -> Option<Self>;
    fn get(self, options: ParserOptions<'_>) -> Option<Float>;
    fn to_short(self) -> &'static str;
    fn to_long(self) -> &'static str;
}
impl UnitBucket for TimeUnit {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "second" | "s" => Self::Second,
            _ => return None,
        })
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Planck => todo!(),
            Self::Second => 1.0,
            Self::Minute => 60.0,
            Self::Hour => 3600.0,
            Self::Day => todo!(),
            Self::Week => todo!(),
            Self::Month => todo!(),
            Self::Year => todo!(),
            Self::Decade => todo!(),
            Self::Century => todo!(),
            Self::Millennium => todo!(),
        })
    }
    fn to_short(self) -> &'static str {
        match self {
            Self::Planck => todo!(),
            Self::Second => todo!(),
            Self::Minute => todo!(),
            Self::Hour => todo!(),
            Self::Day => todo!(),
            Self::Week => todo!(),
            Self::Month => todo!(),
            Self::Year => todo!(),
            Self::Decade => todo!(),
            Self::Century => todo!(),
            Self::Millennium => todo!(),
        }
    }
    fn to_long(self) -> &'static str {
        match self {
            Self::Planck => todo!(),
            Self::Second => todo!(),
            Self::Minute => todo!(),
            Self::Hour => todo!(),
            Self::Day => todo!(),
            Self::Week => todo!(),
            Self::Month => todo!(),
            Self::Year => todo!(),
            Self::Decade => todo!(),
            Self::Century => todo!(),
            Self::Millennium => todo!(),
        }
    }
}
impl UnitBucket for DistanceUnit {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "metre" | "meter" | "m" => Self::Metre,
            _ => return None,
        })
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Planck => todo!(),
            Self::Inch => todo!(),
            Self::Foot => todo!(),
            Self::Yard => todo!(),
            Self::Metre => todo!(),
            Self::Mile => todo!(),
            Self::AstronomicalUnit => todo!(),
            Self::LightYear => todo!(),
            Self::Parsec => todo!(),
        })
    }
    fn to_short(self) -> &'static str {
        match self {
            Self::Planck => todo!(),
            Self::Inch => todo!(),
            Self::Foot => todo!(),
            Self::Yard => todo!(),
            Self::Metre => todo!(),
            Self::Mile => todo!(),
            Self::AstronomicalUnit => todo!(),
            Self::LightYear => todo!(),
            Self::Parsec => todo!(),
        }
    }
    fn to_long(self) -> &'static str {
        match self {
            Self::Planck => todo!(),
            Self::Inch => todo!(),
            Self::Foot => todo!(),
            Self::Yard => todo!(),
            Self::Metre => todo!(),
            Self::Mile => todo!(),
            Self::AstronomicalUnit => todo!(),
            Self::LightYear => todo!(),
            Self::Parsec => todo!(),
        }
    }
}
impl UnitBucket for MassUnit {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "gram" | "g" => Self::Gram,
            _ => return None,
        })
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::ElectronVolt => todo!(),
            Self::Planck => todo!(),
            Self::Gram => todo!(),
            Self::Ounce => todo!(),
            Self::Pound => todo!(),
            Self::Tonne => todo!(),
            Self::Solar => todo!(),
        })
    }
    fn to_short(self) -> &'static str {
        match self {
            Self::ElectronVolt => todo!(),
            Self::Planck => todo!(),
            Self::Gram => todo!(),
            Self::Ounce => todo!(),
            Self::Pound => todo!(),
            Self::Tonne => todo!(),
            Self::Solar => todo!(),
        }
    }
    fn to_long(self) -> &'static str {
        match self {
            Self::ElectronVolt => todo!(),
            Self::Planck => todo!(),
            Self::Gram => todo!(),
            Self::Ounce => todo!(),
            Self::Pound => todo!(),
            Self::Tonne => todo!(),
            Self::Solar => todo!(),
        }
    }
}
impl UnitBucket for TemperatureUnit {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "kelvin" | "K" => Self::Kelvin,
            _ => return None,
        })
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Kelvin => todo!(),
            Self::Rankine => todo!(),
            Self::Celsius => todo!(),
            Self::Fahrenheit => todo!(),
        })
    }
    fn to_short(self) -> &'static str {
        match self {
            Self::Kelvin => todo!(),
            Self::Rankine => todo!(),
            Self::Celsius => todo!(),
            Self::Fahrenheit => todo!(),
        }
    }
    fn to_long(self) -> &'static str {
        match self {
            Self::Kelvin => todo!(),
            Self::Rankine => todo!(),
            Self::Celsius => todo!(),
            Self::Fahrenheit => todo!(),
        }
    }
}
impl UnitBucket for AmountUnit {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "mole" | "mol" => Self::Mole,
            _ => return None,
        })
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Unit => todo!(),
            Self::Mole => todo!(),
            Self::Bit => todo!(),
            Self::Byte => todo!(),
        })
    }
    fn to_short(self) -> &'static str {
        match self {
            Self::Unit => todo!(),
            Self::Mole => todo!(),
            Self::Bit => todo!(),
            Self::Byte => todo!(),
        }
    }
    fn to_long(self) -> &'static str {
        match self {
            Self::Unit => todo!(),
            Self::Mole => todo!(),
            Self::Bit => todo!(),
            Self::Byte => todo!(),
        }
    }
}
impl UnitBucket for LuminousIntensityUnit {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "candela" | "cd" => Self::Candela,
            _ => return None,
        })
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Candela => todo!(),
        })
    }
    fn to_short(self) -> &'static str {
        match self {
            Self::Candela => todo!(),
        }
    }
    fn to_long(self) -> &'static str {
        match self {
            Self::Candela => todo!(),
        }
    }
}
impl UnitBucket for CurrencyUnit {
    fn parse(s: &str) -> Option<Self> {
        Some(match s.to_ascii_uppercase().as_str() {
            "USD" => Self::USD,
            "AUD" => Self::AUD,
            "CAD" => Self::CAD,
            "CHF" => Self::CHF,
            "CNY" => Self::CNY,
            "EUR" => Self::EUR,
            "GBP" => Self::GBP,
            "HKD" => Self::HKD,
            "IDR" => Self::IDR,
            "INR" => Self::INR,
            "JPY" => Self::JPY,
            "KRW" => Self::KRW,
            "MYR" => Self::MYR,
            "NZD" => Self::NZD,
            "PGK" => Self::PGK,
            "PHP" => Self::PHP,
            "SGD" => Self::SGD,
            "THB" => Self::THB,
            "TWD" => Self::TWD,
            "VND" => Self::VND,
            "AED" => Self::AED,
            "QAR" => Self::QAR,
            "SAR" => Self::SAR,
            "DKK" => Self::DKK,
            "EGP" => Self::EGP,
            "ILS" => Self::ILS,
            "JOD" => Self::JOD,
            "NOK" => Self::NOK,
            "SEK" => Self::SEK,
            "ZAR" => Self::ZAR,
            "BRL" => Self::BRL,
            "CLP" => Self::CLP,
            "CZK" => Self::CZK,
            "HUF" => Self::HUF,
            "ISK" => Self::ISK,
            "MXN" => Self::MXN,
            "PLN" => Self::PLN,
            "RON" => Self::RON,
            "TRY" => Self::TRY,
            "UAH" => Self::UAH,
            "MDL" => Self::MDL,
            "RSD" => Self::RSD,
            "RUB" => Self::RUB,
            "AZN" => Self::AZN,
            "BDT" => Self::BDT,
            "DZD" => Self::DZD,
            "GEL" => Self::GEL,
            "KZT" => Self::KZT,
            "TND" => Self::TND,
            "XAF" => Self::XAF,
            "XOF" => Self::XOF,
            "BYN" => Self::BYN,
            "PKR" => Self::PKR,
            "AFN" => Self::AFN,
            "ALL" => Self::ALL,
            "AMD" => Self::AMD,
            "AOA" => Self::AOA,
            "ARS" => Self::ARS,
            "AWG" => Self::AWG,
            "BAM" => Self::BAM,
            "BBD" => Self::BBD,
            "BHD" => Self::BHD,
            "BIF" => Self::BIF,
            "BND" => Self::BND,
            "BOB" => Self::BOB,
            "BSD" => Self::BSD,
            "BWP" => Self::BWP,
            "BZD" => Self::BZD,
            "CDF" => Self::CDF,
            "COP" => Self::COP,
            "CRC" => Self::CRC,
            "CUP" => Self::CUP,
            "CVE" => Self::CVE,
            "DJF" => Self::DJF,
            "DOP" => Self::DOP,
            "ERN" => Self::ERN,
            "ETB" => Self::ETB,
            "FJD" => Self::FJD,
            "GHS" => Self::GHS,
            "GIP" => Self::GIP,
            "GMD" => Self::GMD,
            "GNF" => Self::GNF,
            "GTQ" => Self::GTQ,
            "GYD" => Self::GYD,
            "HNL" => Self::HNL,
            "HTG" => Self::HTG,
            "IQD" => Self::IQD,
            "JMD" => Self::JMD,
            "KES" => Self::KES,
            "KGS" => Self::KGS,
            "KHR" => Self::KHR,
            "KMF" => Self::KMF,
            "KWD" => Self::KWD,
            "KYD" => Self::KYD,
            "LBP" => Self::LBP,
            "LKR" => Self::LKR,
            "LRD" => Self::LRD,
            "LSL" => Self::LSL,
            "LYD" => Self::LYD,
            "MAD" => Self::MAD,
            "MGA" => Self::MGA,
            "MKD" => Self::MKD,
            "MMK" => Self::MMK,
            "MNT" => Self::MNT,
            "MOP" => Self::MOP,
            "MRU" => Self::MRU,
            "MUR" => Self::MUR,
            "MVR" => Self::MVR,
            "MWK" => Self::MWK,
            "MZN" => Self::MZN,
            "NAD" => Self::NAD,
            "NGN" => Self::NGN,
            "NIO" => Self::NIO,
            "NPR" => Self::NPR,
            "OMR" => Self::OMR,
            "PAB" => Self::PAB,
            "PEN" => Self::PEN,
            "PYG" => Self::PYG,
            "RWF" => Self::RWF,
            "SBD" => Self::SBD,
            "SCR" => Self::SCR,
            "SDG" => Self::SDG,
            "SOS" => Self::SOS,
            "SRD" => Self::SRD,
            "SSP" => Self::SSP,
            "STN" => Self::STN,
            "SVC" => Self::SVC,
            "SZL" => Self::SZL,
            "TJS" => Self::TJS,
            "TMT" => Self::TMT,
            "TOP" => Self::TOP,
            "TTD" => Self::TTD,
            "TZS" => Self::TZS,
            "UGX" => Self::UGX,
            "UYU" => Self::UYU,
            "UZS" => Self::UZS,
            "VES" => Self::VES,
            "VUV" => Self::VUV,
            "WST" => Self::WST,
            "XCD" => Self::XCD,
            "XCG" => Self::XCG,
            "XPF" => Self::XPF,
            "YER" => Self::YER,
            "ZMW" => Self::ZMW,
            "IRR" => Self::IRR,
            "LAK" => Self::LAK,
            "SYP" => Self::SYP,
            _ => return None,
        })
    }
    fn get(self, options: ParserOptions<'_>) -> Option<Float> {
        if let Some(currency) = CURRENCY.lock().unwrap().as_ref() {
            return Some(currency[self.discriminant()]);
        }
        let mut arr = [Float::one(); variant_count::<CurrencyUnit>()];
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
                    if let Some(unit) = CurrencyUnit::parse(code_temp) {
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
                    if let Some(unit) = CurrencyUnit::parse(code_temp) {
                        arr[unit.discriminant()] = rate.parse().ok()?;
                    }
                }
            }
        } else {
            download_currency_rates(|code_temp, rate| {
                if let Some(unit) = CurrencyUnit::parse(code_temp) {
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
    fn to_short(self) -> &'static str {
        match self {
            Self::USD => "USD",
            Self::AUD => "AUD",
            Self::CAD => "CAD",
            Self::CHF => "CHF",
            Self::CNY => "CNY",
            Self::EUR => "EUR",
            Self::GBP => "GBP",
            Self::HKD => "HKD",
            Self::IDR => "IDR",
            Self::INR => "INR",
            Self::JPY => "JPY",
            Self::KRW => "KRW",
            Self::MYR => "MYR",
            Self::NZD => "NZD",
            Self::PGK => "PGK",
            Self::PHP => "PHP",
            Self::SGD => "SGD",
            Self::THB => "THB",
            Self::TWD => "TWD",
            Self::VND => "VND",
            Self::AED => "AED",
            Self::QAR => "QAR",
            Self::SAR => "SAR",
            Self::DKK => "DKK",
            Self::EGP => "EGP",
            Self::ILS => "ILS",
            Self::JOD => "JOD",
            Self::NOK => "NOK",
            Self::SEK => "SEK",
            Self::ZAR => "ZAR",
            Self::BRL => "BRL",
            Self::CLP => "CLP",
            Self::CZK => "CZK",
            Self::HUF => "HUF",
            Self::ISK => "ISK",
            Self::MXN => "MXN",
            Self::PLN => "PLN",
            Self::RON => "RON",
            Self::TRY => "TRY",
            Self::UAH => "UAH",
            Self::MDL => "MDL",
            Self::RSD => "RSD",
            Self::RUB => "RUB",
            Self::AZN => "AZN",
            Self::BDT => "BDT",
            Self::DZD => "DZD",
            Self::GEL => "GEL",
            Self::KZT => "KZT",
            Self::TND => "TND",
            Self::XAF => "XAF",
            Self::XOF => "XOF",
            Self::BYN => "BYN",
            Self::PKR => "PKR",
            Self::AFN => "AFN",
            Self::ALL => "ALL",
            Self::AMD => "AMD",
            Self::AOA => "AOA",
            Self::ARS => "ARS",
            Self::AWG => "AWG",
            Self::BAM => "BAM",
            Self::BBD => "BBD",
            Self::BHD => "BHD",
            Self::BIF => "BIF",
            Self::BND => "BND",
            Self::BOB => "BOB",
            Self::BSD => "BSD",
            Self::BWP => "BWP",
            Self::BZD => "BZD",
            Self::CDF => "CDF",
            Self::COP => "COP",
            Self::CRC => "CRC",
            Self::CUP => "CUP",
            Self::CVE => "CVE",
            Self::DJF => "DJF",
            Self::DOP => "DOP",
            Self::ERN => "ERN",
            Self::ETB => "ETB",
            Self::FJD => "FJD",
            Self::GHS => "GHS",
            Self::GIP => "GIP",
            Self::GMD => "GMD",
            Self::GNF => "GNF",
            Self::GTQ => "GTQ",
            Self::GYD => "GYD",
            Self::HNL => "HNL",
            Self::HTG => "HTG",
            Self::IQD => "IQD",
            Self::JMD => "JMD",
            Self::KES => "KES",
            Self::KGS => "KGS",
            Self::KHR => "KHR",
            Self::KMF => "KMF",
            Self::KWD => "KWD",
            Self::KYD => "KYD",
            Self::LBP => "LBP",
            Self::LKR => "LKR",
            Self::LRD => "LRD",
            Self::LSL => "LSL",
            Self::LYD => "LYD",
            Self::MAD => "MAD",
            Self::MGA => "MGA",
            Self::MKD => "MKD",
            Self::MMK => "MMK",
            Self::MNT => "MNT",
            Self::MOP => "MOP",
            Self::MRU => "MRU",
            Self::MUR => "MUR",
            Self::MVR => "MVR",
            Self::MWK => "MWK",
            Self::MZN => "MZN",
            Self::NAD => "NAD",
            Self::NGN => "NGN",
            Self::NIO => "NIO",
            Self::NPR => "NPR",
            Self::OMR => "OMR",
            Self::PAB => "PAB",
            Self::PEN => "PEN",
            Self::PYG => "PYG",
            Self::RWF => "RWF",
            Self::SBD => "SBD",
            Self::SCR => "SCR",
            Self::SDG => "SDG",
            Self::SOS => "SOS",
            Self::SRD => "SRD",
            Self::SSP => "SSP",
            Self::STN => "STN",
            Self::SVC => "SVC",
            Self::SZL => "SZL",
            Self::TJS => "TJS",
            Self::TMT => "TMT",
            Self::TOP => "TOP",
            Self::TTD => "TTD",
            Self::TZS => "TZS",
            Self::UGX => "UGX",
            Self::UYU => "UYU",
            Self::UZS => "UZS",
            Self::VES => "VES",
            Self::VUV => "VUV",
            Self::WST => "WST",
            Self::XCD => "XCD",
            Self::XCG => "XCG",
            Self::XPF => "XPF",
            Self::YER => "YER",
            Self::ZMW => "ZMW",
            Self::IRR => "IRR",
            Self::LAK => "LAK",
            Self::SYP => "SYP",
        }
    }
    fn to_long(self) -> &'static str {
        Self::to_short(self)
    }
}
impl UnitBucket for AngelUnit {
    fn parse(s: &str) -> Option<Self> {
        Some(match s {
            "radian" | "rad" => Self::Rad,
            _ => return None,
        })
    }
    fn get(self, _: ParserOptions<'_>) -> Option<Float> {
        Some(match self {
            Self::Rad => todo!(),
            Self::Deg => todo!(),
            Self::Grad => todo!(),
        })
    }
    fn to_short(self) -> &'static str {
        match self {
            Self::Rad => todo!(),
            Self::Deg => todo!(),
            Self::Grad => todo!(),
        }
    }
    fn to_long(self) -> &'static str {
        match self {
            Self::Rad => todo!(),
            Self::Deg => todo!(),
            Self::Grad => todo!(),
        }
    }
}
impl Unit {
    pub fn parse(s: &str) -> Option<Self> {
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
            TemperatureUnit,
            AmountUnit,
            LuminousIntensityUnit,
            CurrencyUnit,
            AngelUnit,
        );
        None
    }
}
impl Dimensions {
    pub fn discriminant(self) -> usize {
        unsafe { core::mem::transmute::<Self, u8>(self) }.strict_cast()
    }
    pub fn parse(options: ParserOptions<'_>, s: &str) -> Option<NumberInner> {
        let (prefix, postfix) = Prefix::parse(s);
        let mut units = Units::<UnitType, UNIT_COUNT>::default();
        let arr = units.units.as_mut_array();
        let mut mult = 1.0;
        match postfix {
            "s" | "second" => arr[Self::Time.discriminant()] = 1.0,
            "m" | "metre" | "meter" => arr[Self::Distance.discriminant()] = 1.0,
            "g" | "gram" => arr[Self::Mass.discriminant()] = 1.0,
            "A" | "ampere" => arr[Self::Current.discriminant()] = 1.0,
            "K" | "kelvin" => arr[Self::Temperature.discriminant()] = 1.0,
            "mol" | "mole" => arr[Self::Amount.discriminant()] = 1.0,
            "cd" | "candela" => arr[Self::LuminousIntensity.discriminant()] = 1.0,
            //add option on how to store angels
            "deg" | "degree" => mult = PI / 180.0,
            "grad" | "gradian" => mult = PI / 200.0,
            "rad" | "radian" => {}
            _ if let Some(code) = CurrencyUnit::parse(postfix)
                && let Some(m) = get_currency_rate(options, code) =>
            {
                arr[Self::Currency.discriminant()] = 1.0;
                mult = m;
            }
            _ => return None,
        }
        let number = NumberBase::new_real(Real(mult * prefix.multiplier()));
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
