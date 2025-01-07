use clap::Parser;
use pcap::Device;

mod args;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::PkdumpArgs::parse();

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
