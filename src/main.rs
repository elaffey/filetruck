#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use argh::FromArgs;
use std::path::PathBuf;
use termcolor::ColorChoice;

use filetruck::commands::{drop_off, pick_up};
use filetruck::error::Error;
use filetruck::plan::Plan;
use filetruck::printer::{Print, Printer};

#[allow(clippy::default_trait_access)]
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "pickup")]
/// Pick up files
struct PickUp {
    #[argh(option)]
    /// where to pick the files up from
    from: PathBuf,
}

#[allow(clippy::default_trait_access)]
#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "dropoff")]
/// Drop off files
struct DropOff {
    #[argh(option)]
    /// where to drop the files off to
    to: PathBuf,
}

#[derive(FromArgs, Debug)]
#[argh(subcommand)]
enum SubCommands {
    PickUp(PickUp),
    DropOff(DropOff),
}

#[derive(Debug)]
enum ColorOption {
    Auto,
    Always,
    Never,
    AlwaysAnsi,
}

impl std::str::FromStr for ColorOption {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(ColorOption::Always),
            "ansi" => Ok(ColorOption::AlwaysAnsi),
            "auto" => {
                if atty::is(atty::Stream::Stdout) {
                    Ok(ColorOption::Auto)
                } else {
                    Ok(ColorOption::Never)
                }
            }
            "never" => Ok(ColorOption::Never),
            _ => Err(Error::new(format!("Unrecognized color option {}", s))),
        }
    }
}

impl ColorOption {
    fn make_color_choice(&self) -> ColorChoice {
        match self {
            ColorOption::Always => ColorChoice::Always,
            ColorOption::AlwaysAnsi => ColorChoice::AlwaysAnsi,
            ColorOption::Auto => ColorChoice::Auto,
            ColorOption::Never => ColorChoice::Never,
        }
    }
}

#[allow(clippy::default_trait_access)]
#[derive(FromArgs, Debug)]
/// Beep beep. Truck to move file freight around.
struct Args {
    #[argh(option)]
    /// where to read the plan from
    plan: PathBuf,

    #[argh(option, default = "ColorOption::Auto")]
    /// allow color output? Available options: always, ansi, auto. Default is auto
    color: ColorOption,

    #[argh(subcommand)]
    sub_commands: SubCommands,
}

fn run(args: Args, stdout: &mut Printer) -> Result<(), Error> {
    let plan = Plan::load(&args.plan)?;
    match args.sub_commands {
        SubCommands::PickUp(options) => pick_up(&plan, &options.from, stdout),
        SubCommands::DropOff(options) => drop_off(&plan, &options.to, stdout),
    }
}

fn main() {
    let args: Args = argh::from_env();
    let color_choice = args.color.make_color_choice();
    let mut stdout = Printer::stdout(color_choice);
    let mut stderr = Printer::stderr(color_choice);
    if let Err(e) = run(args, &mut stdout) {
        stderr.writeln(e);
        stderr.print();
        std::process::exit(1);
    }
}
