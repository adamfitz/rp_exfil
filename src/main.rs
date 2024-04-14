// mod allows use of custom module
mod icmp_client;
use icmp_client::get_client_interfaces;


fn main() {
    get_client_interfaces();
    
}
