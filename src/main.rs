use english_numbers::*;
use count_write::CountWrite;
use std::io::{self, Write, Sink};
use std::env;
use std::path::PathBuf;
use clap::{arg, command, value_parser, Arg, ArgAction, Command};

const DEFAULT_MIN: i64 = 1;
const DEFAULT_MAX: i64 = 1_000_000;
fn main() -> std::io::Result<()> {
    let matches = command!() // requires `cargo` feature
        // .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --count "Show the count in bytes"
            )
            // .num_args(0)
            .action(ArgAction::SetTrue)
        )
        // .arg(Arg::new("range").num_args(0..2).value_parser(value_parser!(i64)))
        .arg(arg!([range] "Provide range [min max] or [max]").num_args(0..=2)
             .value_parser(value_parser!(i64)))
        .arg(arg!(
            -d --debug ... "Turn debugging information on"
        ))
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

    if *matches.get_one("count").unwrap() {
        let mut byte_counter = CountWrite::from(io::sink());
        type_numbers(&mut byte_counter, min, max)?;
        println!("{}", byte_counter.count());
        Ok(())
    } else {
        type_numbers(&mut io::stdout().lock(), min, max)
    }
}

fn type_numbers<W: Write>(output: &mut W, min: i64, max: i64) -> io::Result<()> {
    let format = Formatting {
        title_case: false,
        .. Formatting::all()
    };
    for i in min..max {
        writeln!(output, "{},", convert(i, format))?;
    }
    writeln!(output, "{}.", convert(max, format))
}
