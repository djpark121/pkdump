
use std::error;

use pcap::{
    Capture, 
    Device, 
    Packet, 
    PacketCodec, 
    PacketHeader
};


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

pub fn listen_to_n_packages(device_name: String, count: usize) -> Result<(), Box<dyn error::Error>>
{
    println!("ListenTo5packages");
    
    let device = Device::lookup()?.ok_or("no device available")?;

    if device.name == device_name {
        println!("  Using device: {}\n", device.name);
 
        let cap = Capture::from_device(device)?
                    .immediate_mode(true)
                    .open()?;

        let mut n: usize = count;
    
        for packet in cap.iter(Codec) {
            let packet = packet?;

            println!("Received packet!\nHeader: {:?}\nData: {:02X?}\n",
                    packet.header, packet.data);

            n = n - 1;

            if n <= 0 {
                break;
            }
        }
    }
    else {
        // TODO(ER) - improve the error handeling in this function
        println!("Device not found!\n");
    }

    Ok(())
}
