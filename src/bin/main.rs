#![deny(clippy::all)]

use argh::FromArgs;
use std::io::Write;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use filetruck::commands::{drop_off, pick_up};
use filetruck::error::Error;
use filetruck::plan::Plan;

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

fn run(args: Args) -> Result<(), Error> {
    let plan = Plan::load(&args.plan)?;
    match args.sub_commands {
        SubCommands::PickUp(options) => pick_up(&plan, &options.from),
        SubCommands::DropOff(options) => drop_off(&plan, &options.to),
    }
}

fn make_stderr(color_choice: ColorChoice) -> StandardStream {
    let mut color_spec = ColorSpec::new();
    color_spec
        .set_fg(Some(Color::Red))
        .set_intense(true)
        .set_bold(true);
    let mut stderr = StandardStream::stderr(color_choice);
    if let Err(e) = stderr.set_color(&color_spec) {
        eprintln!("Error setting terminal color {}", e);
    }
    stderr
}

fn write(stream: &mut StandardStream, s: impl std::fmt::Display) {
    if let Err(e) = writeln!(stream, "{}", s) {
        eprintln!("Error writing to terminal {}", e);
    }
}

fn main() {
    let args: Args = argh::from_env();
    let mut stderr = make_stderr(args.color.make_color_choice());
    if let Err(e) = run(args) {
        write(&mut stderr, e);
        std::process::exit(1);
    }
}
