enum IPAddrKind {
    V4,
    V6
}

enum IPAddr {
    V4(String),
    V6(u16, u16, u16, u16,
       u16, u16, u16, u16)
}

impl IPAddr {
    fn is_v4(&self) -> bool {
        // We use _ to denote ignoring one placeholder
        if matches!(self, IPAddr::V4(_)) {
            true
        }
        else {
            false
        }

    }

    fn is_v6(&self) -> bool {
        // We use _ to denote ignoring multiple placesholders
        // from current place (which is the first parameter) to the end
        if matches!(self, IPAddr::V6(..)) {
            true
        }
        else {
            false
        }

    }
}

fn main() {
    let ip_v4: IPAddr = IPAddr::V4("127.0.01".to_string());
    let ip_v6: IPAddr = IPAddr::V6(0x01, 0x23, 0x45, 0x67,
                                   0x89, 0xab, 0xcd, 0xef);

    println!("ip_v4 is V4?: {}", ip_v4.is_v4());
    println!("ip_v4 is V6?: {}", ip_v4.is_v6());

    println!("ip_v6 is V4?: {}", ip_v6.is_v4());
    println!("ip_v6 is V6?: {}", ip_v6.is_v6());
}
