#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8), 
    V6(String),
}

fn route(ip_type: IpAddrKind) {
    println!("route to {:#?}", ip_type);
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);
}