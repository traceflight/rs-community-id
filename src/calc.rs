use std::net::IpAddr;

use anyhow::{anyhow, Result};

use crate::{ipv4, ipv6, IPPROTO_ICMP, IPPROTO_ICMPV6, IPPROTO_SCTP, IPPROTO_TCP, IPPROTO_UDP};

pub fn calculate_community_id(
    seed: u16,
    src_ip: IpAddr,
    dst_ip: IpAddr,
    src_port: Option<u16>,
    dst_port: Option<u16>,
    ip_proto: u8,
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

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use super::*;

    #[test]
    fn test_calc() {
        let id = calculate_community_id(
            0,
            Ipv4Addr::new(1, 2, 3, 4).into(),
            Ipv4Addr::new(5, 6, 7, 8).into(),
            Some(1122),
            Some(3344),
            6,
            Default::default(),
        );
        assert_eq!("1:wCb3OG7yAFWelaUydu0D+125CLM=", id.unwrap());
    }

    #[test]
    fn test_tcp_without_ports() {
        let id = calculate_community_id(
            0,
            Ipv4Addr::new(1, 2, 3, 4).into(),
            Ipv4Addr::new(5, 6, 7, 8).into(),
            None,
            None,
            6,
            Default::default(),
        );
        assert!(id.is_err());
        assert_eq!("src port and dst port should be set when protocol is icmp/icmp6/tcp/udp/sctp", id.err().unwrap().to_string());
    }
}
