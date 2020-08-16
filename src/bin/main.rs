use argh::FromArgs;
use std::io::Write;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use filetruck::commands::{drop_off, pick_up};
use filetruck::error::Error;
use filetruck::plan::Plan;

#[derive(FromArgs, Debug)]
#[argh(subcommand, name = "pickup")]
/// Pick up files
struct PickUp {
    #[argh(option)]
    /// where to pick the files up from
    from: PathBuf,
}

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

fn default_color() -> String {
    "auto".to_string()
}

#[derive(FromArgs, Debug)]
/// Beep beep. Truck to move file freight around.
struct Args {
    #[argh(option)]
    /// where to read the plan from
    plan: PathBuf,

    #[argh(option, default = "default_color()")]
    /// allow color output? Available options: always, ansi, auto. Default is auto
    color: String,

    #[argh(subcommand)]
    sub_commands: SubCommands,
}

fn run(args: Args) -> Result<(), Error> {
    let plan = Plan::load(&args.plan)?;
    match args.sub_commands {
        SubCommands::PickUp(options) => pick_up(plan, options.from),
        SubCommands::DropOff(options) => drop_off(plan, options.to),
    }
}

fn get_color_choice(args: &Args) -> ColorChoice {
    match args.color.as_str() {
        "always" => ColorChoice::Always,
        "ansi" => ColorChoice::AlwaysAnsi,
        "auto" => {
            if atty::is(atty::Stream::Stdout) {
                ColorChoice::Auto
            } else {
                ColorChoice::Never
            }
        }
        _ => ColorChoice::Never,
    }
}

fn main() {
    let args: Args = argh::from_env();
    let choice = get_color_choice(&args);
    let mut color_spec = ColorSpec::new();
    color_spec
        .set_fg(Some(Color::Red))
        .set_intense(true)
        .set_bold(true);
    let mut stderr = StandardStream::stderr(choice);
    stderr.set_color(&color_spec).unwrap();

    match run(args) {
        Ok(_) => {}
        Err(e) => {
            writeln!(&mut stderr, "{}", e).unwrap();
            std::process::exit(1);
        }
    }
}
