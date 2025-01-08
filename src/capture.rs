
use std::error;

// use pcap:: { Capture, Device };

use pcap::{Capture, Device, Packet, PacketCodec, PacketHeader};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketOwned {
    pub header: PacketHeader,
    pub data: Box<[u8]>,
}

pub struct Codec;

impl PacketCodec for Codec {
    type Item = PacketOwned;

    fn decode(&mut self, packet: Packet) -> Self::Item {
        PacketOwned {
            header: *packet.header,
            data: packet.data.into(),
        }
    }
}

pub fn listen_to_5_packages() -> Result<(), Box<dyn error::Error>>
{
    println!("ListenTo5packages");

    // get the default Device
    let device = Device::lookup()?.ok_or("no device available")?;

    println!("  Using device: {}\n", device.name);

    let cap = Capture::from_device(device)?
        .immediate_mode(true)
        .open()?;

    let mut nn: i32 ;
    nn = 5;

    for packet in cap.iter(Codec) {
        let packet = packet?;

        println!("received packet! {:?}\n", packet);

        nn = nn - 1;

        if nn <= 0 {            break;        }
    }

    Ok(())
}
