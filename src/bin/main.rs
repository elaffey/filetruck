use argh::FromArgs;

use filetruck::commands::{drop_off, pick_up};
use filetruck::plan::Plan;
use filetruck::error::Error;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "pickup")]
/// Pick up files
struct PickUp {
    #[argh(option)]
    /// where to pick the files up from
    from: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "dropoff")]
/// Drop off files
struct DropOff {
    #[argh(option)]
    /// where to drop the files off to
    to: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommands {
    PickUp(PickUp),
    DropOff(DropOff),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Beep beep. Truck to move file freight around.
struct Args {
    #[argh(option)]
    /// where to read the plan from
    plan: String,

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

fn main() {
    let args: Args = argh::from_env();
    match run(args) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
