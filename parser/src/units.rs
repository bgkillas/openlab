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
    Time(TimeUnit),
    Distance(DistanceUnit),
    Mass(MassUnit),
    Current(CurrentUnit),
    Temperature(TemperatureUnit),
    Amount(AmountUnit),
    LuminousIntensity(LuminousIntensityUnit),
    Currency(CurrencyUnit),
    Angel(AngelUnit),
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
        fn strip_or<'a>(s: &'a str, m1: &str, m2: &str) -> Option<&'a str> {
            s.strip_prefix(m1).or_else(|| s.strip_prefix(m2))
        }
        fn parse<'a, const N: usize>(
            s: &'a str,
            ms: [(MetricPrefix, &str, &str); N],
        ) -> Option<(MetricPrefix, &'a str)> {
            for (p, a, b) in ms {
                if let Some(t) = strip_or(s, a, b)
                    && !t.is_empty()
                {
                    return Some((p, t));
                }
            }
            None
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
        fn strip_or<'a>(s: &'a str, m1: &str, m2: &str) -> Option<&'a str> {
            s.strip_prefix(m1).or_else(|| s.strip_prefix(m2))
        }
        fn parse<'a, const N: usize>(
            s: &'a str,
            ms: [(BinaryPrefix, &str, &str); N],
        ) -> Option<(BinaryPrefix, &'a str)> {
            for (p, a, b) in ms {
                if let Some(t) = strip_or(s, a, b)
                    && !t.is_empty()
                {
                    return Some((p, t));
                }
            }
            None
        }
        parse(
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
            _ if let Some(m) = get_currency_rate(options, postfix) => {
                arr[Self::Currency.discriminant()] = 1.0;
                mult = m;
            }
            _ => return None,
        }
        let number = NumberBase::new_real(Real(mult * prefix.multiplier()));
        Some(NumberInner::new(number, units))
    }
}
impl CurrencyUnit {
    pub fn discriminant(self) -> usize {
        unsafe { core::mem::transmute::<Self, u8>(self) }.strict_cast()
    }
    pub fn get(s: &str) -> Option<Self> {
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
}
static CURRENCY: Mutex<Option<[Float; variant_count::<CurrencyUnit>()]>> = Mutex::new(None);
fn get_currency_rate(options: ParserOptions<'_>, code_str: &str) -> Option<Float> {
    let code = CurrencyUnit::get(code_str)?;
    if let Some(currency) = CURRENCY.lock().unwrap().as_ref() {
        return Some(currency[code.discriminant()]);
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
                if let Some(unit) = CurrencyUnit::get(code_temp) {
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
                if let Some(unit) = CurrencyUnit::get(code_temp) {
                    arr[unit.discriminant()] = rate.parse().ok()?;
                }
            }
        }
    } else {
        download_currency_rates(|code_temp, rate| {
            if let Some(unit) = CurrencyUnit::get(code_temp) {
                arr[unit.discriminant()] = rate;
            }
        })?;
    }
    *CURRENCY.lock().unwrap() = Some(arr);
    if let Some(currency) = CURRENCY.lock().unwrap().as_ref() {
        return Some(currency[code.discriminant()]);
    }
    None
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
