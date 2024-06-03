// mod allows use of custom module
mod icmp_client;
use icmp_client::get_client_ips;


fn main() {
    get_client_ips();
    
}
