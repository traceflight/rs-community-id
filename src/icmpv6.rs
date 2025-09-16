use std::collections::HashMap;
use std::sync::LazyLock;

const ECHO_REQUEST: u16 = 128;
const ECHO_REPLY: u16 = 129;
const MLD_LISTENER_QUERY: u16 = 130;
const MLD_LISTENER_REPORT: u16 = 131;
const ND_ROUTER_SOLICIT: u16 = 133;
const ND_ROUTER_ADVERT: u16 = 134;
const ND_NEIGHBOR_SOLICIT: u16 = 135;
const ND_NEIGHBOR_ADVERT: u16 = 136;
const WRU_REQUEST: u16 = 139;
const WRU_REPLY: u16 = 140;
const HAAD_REQUEST: u16 = 144;
const HAAD_REPLY: u16 = 145;

// https://github.com/corelight/pycommunityid/blob/master/communityid/icmp6.py
static ICMP_TYPE_MAPPING: LazyLock<HashMap<u16, u16>> = LazyLock::new(||HashMap::from([
    (ECHO_REQUEST, ECHO_REPLY),
    (ECHO_REPLY, ECHO_REQUEST),
    (MLD_LISTENER_QUERY, MLD_LISTENER_REPORT),
    (MLD_LISTENER_REPORT, MLD_LISTENER_QUERY),
    (ND_ROUTER_SOLICIT, ND_ROUTER_ADVERT),
    (ND_ROUTER_ADVERT, ND_ROUTER_SOLICIT),
    (ND_NEIGHBOR_SOLICIT, ND_NEIGHBOR_ADVERT),
    (ND_NEIGHBOR_ADVERT, ND_NEIGHBOR_SOLICIT),
    (WRU_REQUEST, WRU_REPLY),
    (WRU_REPLY, WRU_REQUEST),
    (HAAD_REQUEST, HAAD_REPLY),
    (HAAD_REPLY, HAAD_REQUEST),
]));

pub(crate) fn get_port_equivalents(mtype: u16, mcode: u16) -> (u16, u16, bool) {
    match ICMP_TYPE_MAPPING.get(&mtype) {
        Some(v) => (mtype, *v, false),
        None => (mtype, mcode, true),
    }
}
