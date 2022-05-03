use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Start mdns service
    #[clap(short, long)]
    pub service: bool,

    /// Discovery mdns service (Default option)
    #[clap(short, long)]
    pub discovery: bool
}

pub fn parse() -> Args {
    Args::parse()
}