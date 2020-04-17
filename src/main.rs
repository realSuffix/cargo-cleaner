mod utils;
mod data;
mod error;

use std::path::PathBuf;
use structopt::StructOpt;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::get_cargo_directories;
use crate::error::CleanError;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Start directory, can be ommitted when starting from the current directory
    #[structopt(parse(from_os_str), short, long)]
    start: Option<PathBuf>,
}

fn main() -> Result<(), CleanError> {
    let opt = Opt::from_args();
    let Opt {start} = opt;
    let start:PathBuf = start.unwrap_or_else(|| PathBuf::from("./"));

    let dirs = get_cargo_directories(start, Rc::new(RefCell::new(vec![])));
    println!("{:?}", dirs);
    Ok(())
}
