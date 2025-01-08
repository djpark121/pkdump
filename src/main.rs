use anyhow;
use clap::Parser;
use pcap::{Capture, Device};

mod args;

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

    if let Some(i) = args.interface {
        println!("Capturing on {}", i);
    }

    if let Some(count) = args.count {
        println!("{} packets requested", count);
    }

    let mut cap = Capture::from_device(Device::lookup().unwrap().unwrap()) // open the "default" interface
        .unwrap() // assume the device exists and we are authorized to open it
        .open() // activate the handle
        .unwrap(); // assume activation worked

    while let Ok(packet) = cap.next_packet() {
        println!("received packet! {:?}", packet);
    }

    Ok(())
}
