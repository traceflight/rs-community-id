use std::collections::HashMap;

use phf::phf_map;

const ECHO_REPLY: u16 = 0;
const ECHO: u16 = 8;
const RTR_ADVERT: u16 = 9;
const RTR_SOLICIT: u16 = 10;
const TSTAMP: u16 = 13;
const TSTAMP_REPLY: u16 = 14;
const INFO: u16 = 15;
const INFO_REPLY: u16 = 16;
const MASK: u16 = 17;
const MASK_REPLY: u16 = 18;

// https://github.com/corelight/pycommunityid/blob/master/communityid/icmp.py
static ICMP_TYPE_MAPPING: phf::Map<u16, u16> = phf_map! {
    ECHO => ECHO_REPLY,
    ECHO_REPLY => ECHO,
    TSTAMP => TSTAMP_REPLY,
    TSTAMP_REPLY => TSTAMP,
    INFO => INFO_REPLY,
    INFO_REPLY => INFO,
    RTR_SOLICIT => RTR_ADVERT,
    RTR_ADVERT => RTR_SOLICIT,
    MASK => MASK_REPLY,
    MASK_REPLY => MASK,
};

pub(crate) fn get_port_equivalents(mtype: u16, mcode: u16) -> (u16, u16, bool) {
    match ICMP_TYPE_MAPPING.get(&mtype) {
        Ok(v) => (mtype, *v, false),
        Err(_) => (mtype, mcode, true),
    }
}

#[cfg(test)]
mod tests {
    use std::net::Ipv4Addr;

    use crate::calculate_community_id;

    #[test]
    fn test_icmp_with_ports() {
        let id = calculate_community_id(
            0,
            Ipv4Addr::new(10, 10, 10, 10).into(),
            Ipv4Addr::new(10, 10, 10, 10).into(),
            Some(0),
            Some(8),
            1,
            Default::default(),
        );

        assert_eq!("1:4MHSMLtBw+4q7Wke3ztBRVwtgt0=", id.unwrap());
    }

    #[test]
    fn test_icmp_without_ports() {
        let id = calculate_community_id(
            0,
            Ipv4Addr::new(10, 10, 10, 10).into(),
            Ipv4Addr::new(10, 10, 10, 10).into(),
            None,
            None,
            1,
            Default::default(),
        );

        assert_eq!("1:4MHSMLtBw+4q7Wke3ztBRVwtgt0=", id.unwrap());
    }
}
