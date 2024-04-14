use pnet::datalink;
use pnet::datalink::Channel::Ethernet;

pub fn get_client_interfaces() {
    /* 
    Return the index of ethernet interfaces present on client machine.  Only if the interface has an IPv4 address that
    is not localhost

    Parameters
        None
    */

    let interfaces = datalink::interfaces();

    let mut index: Vec<u32> = Vec::new();

    for interface in interfaces {
        for i in interface.ips {
            println!("{i}")
        }
    }
}


pub fn create_l2_channel(index: u32) {
    /*
    Create a connection to the network interface to send and receive packets.  
    
    Parameters
        The index number of the ethernet NIC attached to the client machine.
    */
    

}

pub fn create_icmp_packet() {
    /*
    Create an icmp packet.  
    
    Parameters
        TBC.
    */

}