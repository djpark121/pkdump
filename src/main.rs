
use anyhow;
use clap::Parser;
use pcap::Device;

mod args;
pub mod capture;


fn main() -> anyhow::Result<()> {
    let args = args::PkdumpArgs::parse();

    if args.list {
        let devices = Device::list()?;

        println!("List of devices:");
        for device in devices.iter() {
            println!("  {}", device.name);
        }

        return Ok(());
    }

    if let Some(device_name) = args.interface {
        if let Some(count) = args.count {    
            let _ = capture::listen_to_n_packages(device_name, count);

            return Ok(());
        }
    }

    if let Some(count) = args.count {
        println!("{} packets requested", count);
    }

    Ok(())
}
