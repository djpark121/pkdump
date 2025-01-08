
use pcap:: {
    Capture,
    Device
};

pub fn listen_to_5_packages() {

    println!("ListenTo5packages");

    // get the default Device
    let device = Device::lookup()
        .expect("device lookup failed")
        .expect("no device available");

    println!("  Using device: {}\n", device.name);

    let mut cap = Capture::from_device(device)
        .unwrap()
        .immediate_mode(true)
        .open()
        .unwrap();

    let mut nn: i32 ;
    nn = 5;

    while let Ok(packet) = cap.next_packet() {
        println!("received packet! {:?}\n", packet);

        nn = nn - 1;

        if nn <= 0 {
            break;
        }
    }
}
