use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

fn main() {
    let my_ipv4 = Some(Ipv4Addr::new(10, 144, 144, 1));
    let my_ipv6: Option<Ipv6Addr> = None;
    
    let packet_dst_ip = IpAddr::V4(Ipv4Addr::new(10, 144, 144, 1));
    
    // 测试当前的逻辑
    let condition1 = packet_dst_ip == my_ipv4.unwrap_or(Ipv4Addr::UNSPECIFIED);
    let condition2 = packet_dst_ip == my_ipv6.unwrap_or(Ipv6Addr::UNSPECIFIED);
    
    println!("packet_dst_ip: {:?}", packet_dst_ip);
    println!("my_ipv4.unwrap_or(UNSPECIFIED): {:?}", my_ipv4.unwrap_or(Ipv4Addr::UNSPECIFIED));
    println!("my_ipv6.unwrap_or(UNSPECIFIED): {:?}", my_ipv6.unwrap_or(Ipv6Addr::UNSPECIFIED));
    
    println!("condition1 (IPv4 match): {}", condition1);
    println!("condition2 (IPv6 match): {}", condition2);
    println!("Overall result: {}", condition1 || condition2);
    
    // 测试类型
    println!("\nType analysis:");
    println!("packet_dst_ip type: IpAddr::V4");
    println!("my_ipv4.unwrap_or(...) type: Ipv4Addr");
    println!("Can IpAddr::V4 == Ipv4Addr? Let's see...");
    
    // 这个比较实际上会怎样？
    let ipv4_addr = Ipv4Addr::new(10, 144, 144, 1);
    let ip_addr = IpAddr::V4(Ipv4Addr::new(10, 144, 144, 1));
    
    println!("IpAddr::V4(10.144.144.1) == Ipv4Addr(10.144.144.1): {}", ip_addr == IpAddr::V4(ipv4_addr));
}
