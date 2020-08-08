use argh::FromArgs;
use serde::Deserialize;
use std::error::Error;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "pickup")]
/// Pick up files
struct PickUp {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "dropoff")]
/// Drop off files
struct DropOff {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommands {
    PickUp(PickUp),
    DropOff(DropOff),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Create, apply, and undo database migrations
struct Args {
    #[argh(option)]
    /// where to read the plan from
    plan: String,

    #[argh(subcommand)]
    sub_commands: SubCommands,
}

#[derive(Debug, Deserialize)]
struct Plan {
    name: String,
    files: Vec<String>,
}

impl Plan {
    fn load(path: String) -> Result<Self, Box<dyn Error>> {
        let yaml_str = std::fs::read_to_string(path)?;
        let ret: Self = serde_yaml::from_str(&yaml_str)?;
        Ok(ret)
    }
}

fn pick_up(plan: Plan) {
    println!("We are copying   {:?}", plan)
}

fn drop_off(plan: Plan) {
    println!("We are pasting   {:?}", plan)
}

fn main() {
    let args: Args = argh::from_env();
    match Plan::load(args.plan) {
        Ok(config) => match args.sub_commands {
            SubCommands::PickUp(_) => pick_up(config),
            SubCommands::DropOff(_) => drop_off(config),
        },
        Err(e) => {
            eprintln!("Could not load config");
            eprintln!("{:?}", e);
        }
    }
    println!("Done");
}
