
#[allow(unused_imports)]

use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Parser, Debug)]
#[command(version, about = "pkdump is a packet sniffer written in Rust", long_about = None)]
pub struct PkdumpArgs {
    /// Number of packages
    #[arg(short, long)]
    pub count: Option<usize>,

    /// Interface name
    #[arg(short, long)]
    pub interface: Option<String>,

    /// List all devices
    #[arg(short, long)]
    pub list: bool,

    /// Find Traffic by IP
    #[arg(long)]
    pub host: Option<String>,

    /// Capture packages from xxxx
    #[arg(short)]
    pub n: Option<String>,

    /// Filter by Source
    #[arg(short, long)]
    pub src: Option<String>,

    /// Filter by Destination
    #[arg(short, long)]
    pub dst: Option<String>,
}
