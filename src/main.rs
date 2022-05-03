mod command_line;

extern "C" {
    pub fn send_sd();
    pub fn start_mdns_service();
}
fn main() {
    let args = command_line::parse();
    if args.service {
        println!("Start service");
        unsafe { start_mdns_service(); }
    }
    else {
        println!("Discover service");
        unsafe { send_sd(); }
    }
}
