use std::net::IpAddr;

use anyhow::{anyhow, Result};

use libc::{IPPROTO_ICMP, IPPROTO_ICMPV6, IPPROTO_SCTP, IPPROTO_TCP, IPPROTO_UDP};

use crate::{ipv4, ipv6};

pub fn calculate_community_id(
    seed: u16,
    src_ip: IpAddr,
    dst_ip: IpAddr,
    src_port: Option<u16>,
    dst_port: Option<u16>,
    ip_proto: i32,
    disable_base64: bool,
) -> Result<String> {
    match ip_proto {
        IPPROTO_ICMP | IPPROTO_ICMPV6 | IPPROTO_TCP | IPPROTO_UDP | IPPROTO_SCTP => {
            if src_port.is_none() || dst_port.is_none() {
                return Err(anyhow!(
                    "src port and dst port should be set when protocol is icmp/icmp6/tcp/udp/sctp"
                ));
            }
        }
        _ => {}
    }

    match (src_ip, dst_ip) {
        (IpAddr::V4(src_ip), IpAddr::V4(dst_ip)) => ipv4::calculate_ipv4_community_id(
            seed,
            src_ip,
            dst_ip,
            src_port,
            dst_port,
            ip_proto,
            disable_base64,
        ),
        (IpAddr::V6(src_ip), IpAddr::V6(dst_ip)) => ipv6::calculate_ipv6_community_id(
            seed,
            src_ip,
            dst_ip,
            src_port,
            dst_port,
            ip_proto,
            disable_base64,
        ),
        _ => return Err(anyhow!("src ip and dst ip should be same version!")),
    }
}
