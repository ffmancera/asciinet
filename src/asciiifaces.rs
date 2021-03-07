use nispor::{Iface};
use std::collections::HashMap;

pub struct AsciiIface {
    pub name: String,
    pub depth: i32,
}

pub fn nispor_ifaces_to_ascii_ifaces(ifaces: HashMap<String, Iface>) -> Vec<AsciiIface> {
    let mut ascii_ifaces: Vec<AsciiIface> = Vec::new();
    for (ifname, iface) in ifaces.iter() {
        let ascii_iface = generate_ascii_iface(iface, &ifaces);
        ascii_ifaces.push(ascii_iface);
    }
    ascii_ifaces
}

fn generate_ascii_iface(iface: &Iface, iface_states: &HashMap<String, Iface>) -> AsciiIface {
    let depth_level = 0;
    let ascii_iface = AsciiIface { name: String::from(iface.name.to_string()), depth: depth_level };

    ascii_iface
}
