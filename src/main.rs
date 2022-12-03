use clap::Parser;
use color_eyre::eyre::bail;
use color_eyre::Result;
use input::Input;
use std::path::PathBuf;
use std::time::Instant;

days! {day1, day2, day3}
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Config path
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,
    /// Days to run (defaults to last)
    days: Option<Vec<u32>>,
}
pub struct Solution {
    pub first: String,
    pub second: String,
}

fn run_method(input: &Input, day: usize, days: &Vec<fn(&str) -> Result<Solution>>) -> Result<()> {
    if let Some(method) = days.get(day - 1) {
        let input = input.get(day as u8)?;
        let now = Instant::now();
        let result = method(input.as_str())?;
        let total = now.elapsed();
        println!("Day {} {:?} Solution - {} {}", day, total, result.first, result.second);
        Ok(())
    } else {
        bail!("Invalid day {}", day);
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = Input::open(args.config)?;
    let day_methods = days();
    if let Some(days) = args.days {
        for day in days {
            run_method(&input, day as usize, &day_methods)?;
        }
    } else {
        run_method(&input, day_methods.len(), &day_methods)?;
    }
    Ok(())
}

#[macro_export]
macro_rules! days {
    ($($day:ident),+) => {
        $(
        mod $day;
        )+

        fn days() -> Vec<fn(&str) ->  Result<Solution>> {
            let mut rtn: Vec<fn(&str) -> Result<Solution>> = Vec::new();
            $(
                rtn.push($day::solve);
            )+
            rtn
        }
    };
}
