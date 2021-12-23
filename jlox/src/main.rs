use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "jlox")]
struct Opt {
    /// Input file
    #[structopt(short, long, parse(from_os_str))]
    file: Option<PathBuf>,
}

pub fn main() -> Result<()> {
    env_logger::init();

    if let Some(file) = Opt::from_args().file {
        log::info!("Reading input form file {:#?}.", &file);
        let file_contents = fs::read_to_string(file)?;

        log::debug!("File contents: {:#?}", file_contents);
    } else {
        log::info!("Running in interactive mode.");
    }

    Ok(())
}
