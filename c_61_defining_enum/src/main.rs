//enum IpAddrKind {
//    V4,
//    V6,
//}

enum IpAddr {
    V4(String),
    V6(String),
}

//enum IpAddr {
//    V4(u8, u8, u8, u8),
//    V6(String),
//}

//struct IpAddr {
//    kind: IpAddrKind::V4,
//    address: String, 
//}

fn main() {
    //let four = IpAddrKind::V4;
    //let six = IpAddrKind::V6;

    //route(IpAddrKind::V4);

    //let home = IpAddr {
    //    kind: IpAddrKind::V4,
    //    address: String::from("127.0.0.1"),
    //};

    //let loopback = IpAddr {
    //    kind: IpAddrKind::V6,
    //    address: String::from("::1"),
    //};

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    //let home = IpAddr::V4(String::from("127.0.0.1"));
    //let loopback = IpAddr::V6(String::from("::1"));

}

//fn route(ip_kind: IpAddrKind) {}
