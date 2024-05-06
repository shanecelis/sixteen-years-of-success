use clap::{self, arg, command, value_parser, ArgAction};
use std::fmt;
use std::ops::Range;
use std::time::Duration;
use count_write::CountWrite;
use english_numbers;
use human_duration::human_duration;
use time_humanize::HumanTime;
use human_repr::HumanCount;
use numbers;
use std::env;
use std::io::{self, Write};

#[derive(clap::ValueEnum, Debug, Clone, Copy)]
#[clap(rename_all = "kebab_case")]
enum Language {
    // #[default]
    English,
    French,
    Chinese,
    Numeric
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Language::English => "in English",
            Language::French => "in French",
            Language::Chinese => "in Chinese",
            Language::Numeric => "numerically",
        })
    }
}

impl From<Language> for numbers::Language {
    fn from(this: Language) -> Self {
        match this {
            Language::English => Self::English,
            Language::French => Self::French,
            Language::Chinese => Self::Chinese,
            x => panic!("No conversion from {x:?}")
        }
    }
}

#[derive(Debug, Clone)]
struct Feat {
    name: String,
    duration: Duration,
    languages: Vec<Language>,
    extrapolated: bool,
    range: Range<i64>,
    sheets: Option<usize>,
    typewriters: Option<u8>,
    ink_ribbons: Option<usize>,
    bytes: Option<usize>,
}

impl std::ops::Mul<f64> for &Feat {
    type Output = Feat;

    // Required method
    fn mul(self, factor: f64) -> Self::Output {
        let range_size = self.range.end - self.range.start;
        Feat {
            name: self.name.clone(),
            duration: Duration::from_secs((self.duration.as_secs_f64() * factor) as u64),
            range: self.range.clone(),
            // range: Range {
            //     start: self.range.start,
            //     end: self.range.start + (range_size as f64 * factor) as i64
            // },
            sheets: self.sheets.map(|x| (x as f64 * factor) as usize),
            typewriters: self.typewriters.map(|x| (x as f64 * factor) as u8),
            ink_ribbons: self.ink_ribbons.map(|x| (x as f64 * factor) as usize),
            bytes: self.bytes.map(|x| (x as f64 * factor) as usize),
            extrapolated: true,
            languages: self.languages.clone(),
        }
    }
}

impl fmt::Display for Feat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//Seven manual typewriters, 1000 ink ribbons, 19,890 pages, 16 years and seven months later, he finished with the lines
        write!(f, "{} {}used ", self.name, if self.extrapolated { "would have " } else { "" })?;
        if let Some(typewriters) = self.typewriters {
            write!(f, "{} manual typewriters, ", typewriters)?;
        }
        if let Some(ribbons) = self.ink_ribbons {
            write!(f, "{} ink ribbons, ", ribbons)?;
        }
        if let Some(pages) = self.sheets {
            write!(f, "{} pages, ", pages)?;
        }
        let time = format!("{}", HumanTime::from(self.duration));
        write!(f, "and {} {} ", if self.extrapolated { "taken" } else { "took" }, time.strip_prefix("in ").unwrap_or(&time))?;

        write!(f, "to type numbers ")?;
        let mut i = 0;
        for lang in &self.languages {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", lang)?;
            i += 1;
        }
        write!(f, " from {} to {}.", self.range.start, self.range.end)
    }
}

impl Feat {
    fn extrapolate(&self, bytes: usize) -> Feat {
        let original_bytes = self.bytes.unwrap();
        let factor = bytes as f64 / original_bytes as f64;
        if (1.0 - factor).abs() < 0.01 {
            self.clone()
        } else {
            self * factor
        }
    }
}

fn years(y: f32) -> Duration {
    let k: u32 = 365 * 24 * 60 * 60;
    Duration::from_secs((k as f32 * y) as u64)
}

fn feats() -> impl Iterator<Item = Feat> {
    [Feat {
        name: "Les Stewart".into(),
        duration: years(16.58), // 16 years and 7 months
        languages: vec![Language::English],
        range: 1..1_000_000,
        sheets: Some(19_890),
        typewriters: Some(7),
        ink_ribbons: Some(1_000),
        bytes: Some(62_017_013),
        extrapolated: false,
    },
     Feat {
        name: "Danny Johnson".into(),
        languages: vec![Language::Numeric, Language::English],
        duration: years(12.0),
        range: 1..1_000_000,
        sheets: Some(20_000),
        typewriters: None,
        ink_ribbons: Some(3_000), // 30 km of ribbon, 10m/ribbon
        bytes: Some(62_017_013),
        extrapolated: false,
    }
    ].into_iter()
}

const DEFAULT_MIN: i64 = 1;
const DEFAULT_MAX: i64 = 1_000_000;
fn main() -> std::io::Result<()> {
    let matches = command!() // requires `cargo` feature
        .disable_version_flag(true)
        .arg(arg!(-c --count "Show the count in bytes").action(ArgAction::SetTrue))
        .arg(arg!(-H --humanize "Human readable byte count").action(ArgAction::SetTrue))
        .arg(arg!(-e --extrapolate [person] "Compare with typing feat")
             .action(ArgAction::Append))
        .arg(
            arg!(-l --language [lang] "Choose language")
                .action(ArgAction::Append)
                .value_parser(clap::builder::EnumValueParser::<Language>::new()),
        )
        .arg(
            arg!(-u --upto [upto] "Only count up to so many bytes")
                .num_args(1)
                .value_parser(value_parser!(u64)),
        )
        .arg(
            arg!([range] "Provide range [min max] or [max]")
                .num_args(0..=2)
                .value_parser(value_parser!(i64)),
        )
        .get_matches();
    let (min, max) = match matches.get_many::<i64>("range") {
        None => (DEFAULT_MIN, DEFAULT_MAX),
        Some(mut values) => {
            let first = *values.next().unwrap();
            match values.next() {
                None => (DEFAULT_MIN, first),
                Some(v) => (first, *v),
            }
        }
    };
    let humanize: bool = *matches.get_one("humanize").unwrap();
    let upto: Option<u64> = matches.get_one("upto").cloned();
    let langs: Vec<Language> = matches
        .get_many("language")
        .map(|x: clap::parser::ValuesRef<Language>| {
            x.map(|y: &Language| Language::from(y.clone())).collect()
        })
        .unwrap_or(vec![Language::English]);

    let byte_count = if *matches.get_one("count").unwrap() {
        let mut byte_counter = CountWrite::from(io::sink());
        match upto {
            None => {
                type_numbers(&mut byte_counter, &langs, min, |_, i| i < max)?;
            }
            Some(max_bytes) => {
                let number = type_numbers(&mut byte_counter, &langs, min, |w, _| {
                    w.count() <= max_bytes
                })?;
                println!("Reached number {number} in {} bytes.", byte_counter.count());
                return Ok(());
            }
        }
        if humanize {
            println!("{}", byte_counter.count().human_count_bytes());
        } else {
            println!("{}", byte_counter.count());
        }
        byte_counter.count()
    } else {
        let mut stdout = io::stdout().lock();
        let mut byte_counter = CountWrite::from(&mut stdout);
        type_numbers(&mut byte_counter, &langs, min, |_, i| i < max)?;
        byte_counter.count()
    } as usize;

    if let Some(people) = matches.get_many::<String>("extrapolate") {
        for person in people {
            if let Some(person) = find_case_insensitive(feats(), |f| f.name.clone(), person) {
                let mut extrapolated_feat = person.extrapolate(byte_count);
                extrapolated_feat.languages = langs.clone();
                println!("{}", extrapolated_feat);

            } else {
                println!("No feat found for '{}'.", person);
            }
        }
    }
    Ok(())
}

fn find_case_insensitive<T, F: Fn(&T) -> String>(mut iter: impl Iterator<Item = T>, key: F, search: &str) -> Option<T> {
    // Convert search term to lowercase for case-insensitive comparison
    let search_lower = search.to_lowercase();

    // Find an item in the vec with case-insensitive match
    iter
        .find(|item| key(item).to_lowercase().contains(&search_lower))
}


fn convert(lang: Language, number: i64) -> String {
    match lang {
        Language::English => {
            let format = english_numbers::Formatting {
                title_case: false,
                // conjunctions: false,
                ..english_numbers::Formatting::all()
            };
            english_numbers::convert(number, format)
        }
        Language::Numeric => {
            number.to_string()
        }
        lang => {
            numbers::convert(lang.into(), number)
        }
    }
}

fn type_numbers<W: Write, F: FnMut(&W, i64) -> bool>(
    output: &mut W,
    langs: &[Language],
    min: i64,
    mut cond: F,
) -> io::Result<i64> {
    let mut i = min;
    loop {
        if !cond(&output, i) {
            break;
        };
        for lang in langs {
            writeln!(output, "{},", convert(*lang, i))?;
        }
        i += 1;
    }
    for lang in langs {
        writeln!(output, "{}.", convert(*lang, i))?;
    }
    Ok(i)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_display() {
        let feat = feats().next().unwrap();
        assert_eq!(format!("{feat}"), "Les Stewart used 7 manual typewriters, 1000 ink ribbons, 19890 pages, and took 16 years to type numbers from 1 to 1000000.");

    }

}
