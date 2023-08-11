mod calc;
mod icmpv4;
mod icmpv6;
mod ipv4;
mod ipv6;

const PADDING: u8 = 0;
const IPPROTO_ICMP: u8 = 1;
const IPPROTO_ICMPV6: u8 = 58;
const IPPROTO_SCTP: u8 = 132;
const IPPROTO_TCP: u8 = 6;
const IPPROTO_UDP: u8 = 17;

pub use calc::calculate_community_id;
