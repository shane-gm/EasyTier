use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// 测试 IP 地址比较逻辑，验证 ACL 链类型判断中的问题
fn main() {
    println!("=== IP 地址比较测试 ===\n");
    
    // 模拟 EasyTier 节点的实际场景
    test_normal_case();
    test_none_ipv4_case();
    test_mixed_ipv4_ipv6_case();
    test_unspecified_edge_cases();
}

/// 测试正常情况：节点有配置的 IPv4 地址
fn test_normal_case() {
    println!("1. 正常情况测试：节点有配置 IPv4 地址");
    
    let my_ipv4 = Some(Ipv4Addr::new(10, 144, 144, 1));
    let my_ipv6: Option<Ipv6Addr> = None;
    
    // 测试各种数据包目标地址
    let test_cases = vec![
        ("发往本节点", IpAddr::V4(Ipv4Addr::new(10, 144, 144, 1))),
        ("发往其他节点", IpAddr::V4(Ipv4Addr::new(10, 144, 144, 2))),
        ("发往 0.0.0.0", IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
        ("发往 127.0.0.1", IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
    ];
    
    for (desc, packet_dst_ip) in test_cases {
        let chain_type = determine_chain_type_current_logic(packet_dst_ip, my_ipv4, my_ipv6);
        let expected = determine_chain_type_correct_logic(packet_dst_ip, my_ipv4, my_ipv6);
        
        println!("  {}: {} (预期: {})", desc, chain_type, expected);
        if chain_type != expected {
            println!("    ❌ 错误！当前逻辑与预期不符");
        }
    }
    println!();
}

/// 测试问题情况：节点没有配置 IPv4 地址
fn test_none_ipv4_case() {
    println!("2. 问题情况测试：节点没有配置 IPv4 地址 (my_ipv4 = None)");
    
    let my_ipv4: Option<Ipv4Addr> = None;  // 这是问题的根源
    let my_ipv6: Option<Ipv6Addr> = None;
    
    let test_cases = vec![
        ("发往某个节点", IpAddr::V4(Ipv4Addr::new(10, 144, 144, 1))),
        ("发往 0.0.0.0", IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))),
        ("发往 192.168.1.1", IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))),
    ];
    
    for (desc, packet_dst_ip) in test_cases {
        let chain_type = determine_chain_type_current_logic(packet_dst_ip, my_ipv4, my_ipv6);
        let expected = determine_chain_type_correct_logic(packet_dst_ip, my_ipv4, my_ipv6);
        
        println!("  {}: {} (预期: {})", desc, chain_type, expected);
        if chain_type != expected {
            println!("    ❌ 错误！当前逻辑与预期不符");
        }
    }
    println!();
}

/// 测试 IPv4/IPv6 混合情况
fn test_mixed_ipv4_ipv6_case() {
    println!("3. IPv4/IPv6 混合情况测试");
    
    let my_ipv4 = Some(Ipv4Addr::new(10, 144, 144, 1));
    let my_ipv6 = Some(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1));
    
    let test_cases = vec![
        ("IPv4 发往本节点", IpAddr::V4(Ipv4Addr::new(10, 144, 144, 1))),
        ("IPv6 发往本节点", IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 1))),
        ("IPv4 发往其他节点", IpAddr::V4(Ipv4Addr::new(10, 144, 144, 2))),
        ("IPv6 发往其他节点", IpAddr::V6(Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 2))),
    ];
    
    for (desc, packet_dst_ip) in test_cases {
        let chain_type = determine_chain_type_current_logic(packet_dst_ip, my_ipv4, my_ipv6);
        let expected = determine_chain_type_correct_logic(packet_dst_ip, my_ipv4, my_ipv6);
        
        println!("  {}: {} (预期: {})", desc, chain_type, expected);
        if chain_type != expected {
            println!("    ❌ 错误！当前逻辑与预期不符");
        }
    }
    println!();
}

/// 测试 UNSPECIFIED 地址的边缘情况
fn test_unspecified_edge_cases() {
    println!("4. UNSPECIFIED 地址边缘情况测试");
    
    println!("  Ipv4Addr::UNSPECIFIED = {:?}", Ipv4Addr::UNSPECIFIED);
    println!("  Ipv6Addr::UNSPECIFIED = {:?}", Ipv6Addr::UNSPECIFIED);
    
    // 测试当节点地址就是 UNSPECIFIED 时会怎样
    let my_ipv4 = Some(Ipv4Addr::UNSPECIFIED);
    let my_ipv6: Option<Ipv6Addr> = None;
    
    let packet_dst_ip = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
    let chain_type = determine_chain_type_current_logic(packet_dst_ip, my_ipv4, my_ipv6);
    
    println!("  节点地址是 0.0.0.0，数据包也发往 0.0.0.0: {}", chain_type);
    println!();
}

/// 当前 EasyTier 中的链类型判断逻辑（有问题的版本）
fn determine_chain_type_current_logic(
    packet_dst_ip: IpAddr, 
    my_ipv4: Option<Ipv4Addr>, 
    my_ipv6: Option<Ipv6Addr>
) -> &'static str {
    // 这是 easytier/src/peers/acl_filter.rs:318-319 的逻辑
    if packet_dst_ip == my_ipv4.unwrap_or(Ipv4Addr::UNSPECIFIED)
        || packet_dst_ip == my_ipv6.unwrap_or(Ipv6Addr::UNSPECIFIED)
    {
        "Inbound"
    } else {
        "Forward"
    }
}

/// 正确的链类型判断逻辑
fn determine_chain_type_correct_logic(
    packet_dst_ip: IpAddr, 
    my_ipv4: Option<Ipv4Addr>, 
    my_ipv6: Option<Ipv6Addr>
) -> &'static str {
    match packet_dst_ip {
        IpAddr::V4(addr) => {
            if my_ipv4.map_or(false, |my_addr| addr == my_addr) {
                "Inbound"
            } else {
                "Forward"
            }
        }
        IpAddr::V6(addr) => {
            if my_ipv6.map_or(false, |my_addr| addr == my_addr) {
                "Inbound"
            } else {
                "Forward"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current_logic_bug() {
        // 这个测试展示当前逻辑的 bug
        let my_ipv4: Option<Ipv4Addr> = None;
        let my_ipv6: Option<Ipv6Addr> = None;
        
        let packet_to_zero = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let packet_to_real_ip = IpAddr::V4(Ipv4Addr::new(10, 144, 144, 1));
        
        // Bug: 0.0.0.0 被错误地判断为 Inbound
        assert_eq!(
            determine_chain_type_current_logic(packet_to_zero, my_ipv4, my_ipv6),
            "Inbound"
        );
        
        // Bug: 真实 IP 被错误地判断为 Forward
        assert_eq!(
            determine_chain_type_current_logic(packet_to_real_ip, my_ipv4, my_ipv6),
            "Forward"
        );
        
        // 正确的逻辑应该都是 Forward（因为节点没有配置地址）
        assert_eq!(
            determine_chain_type_correct_logic(packet_to_zero, my_ipv4, my_ipv6),
            "Forward"
        );
        assert_eq!(
            determine_chain_type_correct_logic(packet_to_real_ip, my_ipv4, my_ipv6),
            "Forward"
        );
    }

    #[test]
    fn test_normal_case() {
        let my_ipv4 = Some(Ipv4Addr::new(10, 144, 144, 1));
        let my_ipv6: Option<Ipv6Addr> = None;
        
        let packet_to_me = IpAddr::V4(Ipv4Addr::new(10, 144, 144, 1));
        let packet_to_other = IpAddr::V4(Ipv4Addr::new(10, 144, 144, 2));
        
        // 正常情况下，两种逻辑应该得到相同结果
        assert_eq!(
            determine_chain_type_current_logic(packet_to_me, my_ipv4, my_ipv6),
            determine_chain_type_correct_logic(packet_to_me, my_ipv4, my_ipv6)
        );
        
        assert_eq!(
            determine_chain_type_current_logic(packet_to_other, my_ipv4, my_ipv6),
            determine_chain_type_correct_logic(packet_to_other, my_ipv4, my_ipv6)
        );
    }
}