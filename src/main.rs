use clap::Parser;
use pcap::Device;

#[derive(Parser, Debug)]
#[command(version, about = "pkdump is a packet sniffer written in Rust", long_about = None)]
struct Args {
    #[arg(short, long)]
    count: Option<usize>,

    #[arg(short, long)]
    interface: Option<String>,

    #[arg(short, long)]
    list: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if let Some(i) = args.interface {
        println!("Capturing on {}", i);
    }

    if let Some(count) = args.count {
        println!("{} packets requested", count);
    }

    let devices = Device::list()?;

    if args.list {
        for device in devices.iter() {
            println!("{}", device.name);
        }
    }

    Ok(())
}
