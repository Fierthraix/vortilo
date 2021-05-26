use std::path::PathBuf;

//use atty::Stream;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "vortilo")]
struct Argujo {
    /// Kiun dosieron traduki.
    #[structopt(short, long, parse(from_os_str))]
    dosiero: PathBuf,
}
