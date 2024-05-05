use clap::{self, arg, command, value_parser, ArgAction};
use count_write::CountWrite;
use english_numbers;
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

const DEFAULT_MIN: i64 = 1;
const DEFAULT_MAX: i64 = 1_000_000;
fn main() -> std::io::Result<()> {
    let matches = command!() // requires `cargo` feature
        .disable_version_flag(true)
        .arg(arg!(-c --count "Show the count in bytes").action(ArgAction::SetTrue))
        .arg(arg!(-H --humanize "Human readable byte count").action(ArgAction::SetTrue))
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

    if *matches.get_one("count").unwrap() {
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
    } else {
        type_numbers(&mut io::stdout().lock(), &langs, min, |_, i| i < max)?;
    }
    Ok(())
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
