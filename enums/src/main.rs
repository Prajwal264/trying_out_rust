enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    #![allow(unused)]
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("192.168.1.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("::4"),
    };
}
