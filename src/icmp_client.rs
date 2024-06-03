use pnet::datalink;
use pnet::datalink::Channel::Ethernet;

pub fn get_client_ips() {
    /* 
    Return the ipv4 and/or ipv6 addresses of the client machine, filtering out localhost and link local addresses.

    Parameters
        None
    */

    // get the interfaces of the client machine
    let interfaces = datalink::interfaces();

    // define a data struct to contain the IP addresses for each interface
    let mut addresses: Vec<String> = Vec::new();

    // populate the vec with the IPv4 / IPv6 addresses
    for interface in interfaces {
        for i in interface.ips {
            // convert to a string
            let ip_string = i.to_string();
            // filter out localhost and link local addresses
            // iterate the array, filter out addresses containing any of the substrings and add to vec
            if !["127", "fe", "::1/128"].iter().any(|&sub_string| ip_string.contains(sub_string)) {
                addresses.push(ip_string);
            };
        }
    }
    // all ips on the system
    println!("{:?}", addresses)


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