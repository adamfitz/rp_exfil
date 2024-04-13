use pnet::datalink;
use pnet::datalink::Channel::Ethernet;

pub fn get_client_interfaces() {
    let interfaces = datalink::interfaces();
    for interface in interfaces {
        println!("{}: {}", interface.index, interface.name)
    }
}