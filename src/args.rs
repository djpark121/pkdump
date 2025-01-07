
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
}
