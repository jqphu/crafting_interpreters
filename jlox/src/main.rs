use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "jlox")]
struct Opt {
    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    file: Option<PathBuf>,
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    if let Some(file) = opt.file {
        log::info!("Reading input form file {:#?}.", &file);
    } else {
        log::info!("Running in interactive mode.");
    }
}
