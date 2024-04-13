// mod allows use of custom module
mod icmp_client;
use icmp_client::get_client_interfaces;

use pnet::packet::{Packet, MutablePacket};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};

fn main() {
    let client_interfaces = get_client_interfaces();
    println!("{:?}", client_interfaces)
}
