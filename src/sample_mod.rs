mod server {
    pub fn echo() {
        println!("Server");
    }
}

mod client {
    pub fn echo() {
        println!("Client")
    }
}

mod network;

fn main() {
    server::echo();
    client::echo();
    network::ping();
}