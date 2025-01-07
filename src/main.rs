use clap::Parser;
use pcap::Device;

#[derive(Parser, Debug)]
#[command(version, about = "pkdump is a packet sniffer written in Rust", long_about = None)]
struct PkdumpArgs {
    /// Count
    #[arg(short, long)]
    count: Option<usize>,

    /// Interface name
    #[arg(short, long)]
    interface: Option<String>,

    /// List all devices
    #[arg(short, long)]
    list: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = PkdumpArgs::parse();

    if let Some(i) = args.interface {
        println!("Capturing on {}", i);
    }

    if let Some(count) = args.count {
        println!("{} packets requested", count);
    }

    if args.list {
        let devices = Device::list()?;

        println!("List of devices:");
        for device in devices.iter() {
            println!("  {}", device.name);
        }

        return Ok(())
    }

    Ok(())
}
