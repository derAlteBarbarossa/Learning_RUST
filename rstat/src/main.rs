use std::path::PathBuf;
use structopt::StructOpt;

mod srcstats;
use crate::srcstats::get_summary_src_stats;

mod errors;


#[derive(Debug, StructOpt)]
#[structopt(
    name = "rstat",
    about = "This is a tool to generate statistics on Rust codes"
)]
struct Opt{
    #[structopt(name = "source directory",
                parse(from_os_str))]
    in_dir: PathBuf,
    #[structopt(name = "mode", short)]
    mode: String,
}


fn main() -> Result<(), errors::StatsError> {
    let opt = Opt::from_args();
    let mode = &opt.mode[..];

    match mode {
        "src" => {
            let stats = get_summary_src_stats(&opt.in_dir);
            println!("Summary stats: {:?}", stats);
        },
        _ => println!("Sorry, no stats"),
    }

    Ok(())
}
