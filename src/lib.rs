mod calc;
mod icmpv4;
mod icmpv6;
mod ipv4;
mod ipv6;

const PADDING: u8 = 0;

pub use calc::calculate_community_id;
