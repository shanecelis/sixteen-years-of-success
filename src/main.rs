use numbers::*;
use count_write::CountWrite;
use std::io::{self, Write};
use std::env;
use clap::{self, arg, command, value_parser, ArgAction};

#[derive(clap::ValueEnum, Debug, Clone)]
#[clap(rename_all = "kebab_case")]
enum LanguageArg {
    // #[default]
    English,
    French,
    Chinese,
}

impl From<LanguageArg> for Language {
    fn from(this: LanguageArg) -> Self {
        match this {
            LanguageArg::English => Self::English,
            LanguageArg::French => Self::French,
            LanguageArg::Chinese => Self::Chinese,
        }
    }
}

const DEFAULT_MIN: i64 = 1;
const DEFAULT_MAX: i64 = 1_000_000;
fn main() -> std::io::Result<()> {
    let matches = command!() // requires `cargo` feature
        .disable_version_flag(true)
        .arg(arg!(-c --count "Show the count in bytes")
            .action(ArgAction::SetTrue))
        .arg(arg!(-l --language [lang] "Choose language")
             .value_parser(clap::builder::EnumValueParser::<LanguageArg>::new())
        )

        .arg(arg!(-u --upto [upto] "Only count up to so many bytes")
             .num_args(1)
             .value_parser(value_parser!(u64)))
        .arg(arg!([range] "Provide range [min max] or [max]")
             .num_args(0..=2)
             .value_parser(value_parser!(i64)))
        .get_matches();
    let (min, max) = match matches.get_many::<i64>("range") {
        None => (DEFAULT_MIN, DEFAULT_MAX),
        Some(mut values) => {
            let first = *values.next().unwrap();
            match values.next() {
                None => (DEFAULT_MIN, first),
                Some(v) => (first, *v)
            }
        }
    };
    let upto: Option<u64> = matches.get_one("upto").cloned();
    let langs: Vec<Language> = matches
        .get_many("language")
        .map(|x: clap::parser::ValuesRef<LanguageArg>|
             x
             .map(|y: &LanguageArg| Language::from(y.clone()))
             .collect())
        .unwrap_or(vec![Language::English]);
    let lang = Language::English;

    if *matches.get_one("count").unwrap() {
        let mut byte_counter = CountWrite::from(io::sink());
        match upto {
            None => type_numbers(&mut byte_counter, &langs, min, max)?,
            Some(max_bytes) => {
                let number = type_numbers_cond(&mut byte_counter, &langs, min, |w| w.count() <= max_bytes)?;
                println!("Reached number {number} in {} bytes.", byte_counter.count());
                return Ok(());
            }
        }
        println!("{}", byte_counter.count());
        Ok(())
    } else {
        type_numbers(&mut io::stdout().lock(), &langs, min, max)
    }
}

fn type_numbers<W: Write>(output: &mut W, langs: &[Language], min: i64, max: i64) -> io::Result<()> {
    // let format = Formatting {
    //     title_case: false,
    //     conjunctions: false,
    //     .. Formatting::all()
    // };
    for i in min..max {
        for lang in langs {
            writeln!(output, "{},", convert(*lang, i))?;
        }
        // writeln!(output, "{},", french_number(&i))?;
    }
    for lang in langs {
        writeln!(output, "{}.", convert(*lang, max))?;
    }
    // writeln!(output, "{},", french_number(&max))?;
    Ok(())
}

fn type_numbers_cond<W: Write, F: FnMut(&W) -> bool>(output: &mut W, langs: &[Language], min: i64, mut cond: F) -> io::Result<i64> {
    // let format = Formatting {
    //     title_case: false,
    //     conjunctions: false,
    //     .. Formatting::all()
    // };
    let mut i = min;
    loop {
        if ! cond(&output) {
            break;
        };
        for lang in langs {
            writeln!(output, "{},", convert(*lang, i))?;
        }
        i += 1;
    }
    Ok(i)
}
