use argh::FromArgs;

mod ops;
mod plan;

use ops::{drop_off, pick_up};
use plan::Plan;

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

fn main() {
    let args: Args = argh::from_env();
    match Plan::load(args.plan) {
        Ok(config) => {
            let res = match args.sub_commands {
                SubCommands::PickUp(options) => pick_up(config, options.from),
                SubCommands::DropOff(options) => drop_off(config, options.to),
            };
            match res {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Got error here");
                    eprintln!("{}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Could not load config");
            eprintln!("{:?}", e);
        }
    }
    println!("Done");
}
