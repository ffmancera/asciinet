mod asciiifaces;

use crate::asciiifaces::AsciiIface;
use nispor::NetState;

pub struct AsciiState {
    pub ifaces: Vec<AsciiIface>,
}

fn main() {
    let net_state = match NetState::retrieve() {
        Ok(state) => {
            let ascii_ifaces = asciiifaces::nispor_ifaces_to_ascii_ifaces(state.ifaces);
            println!("State retrieved")
        }
        Err(_) => println!("Error when ")
    };
}
