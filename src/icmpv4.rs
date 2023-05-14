use std::collections::HashMap;

use lazy_static::lazy_static;
use num_enum::{IntoPrimitive, TryFromPrimitive};

// 参考 https://github.com/corelight/pycommunityid/blob/master/communityid/icmp.py
lazy_static! {
    static ref ICMP_TYPE_MAPPING: HashMap<IcmpType, IcmpType> = HashMap::from([
        (IcmpType::Echo, IcmpType::EchoReply),
        (IcmpType::EchoReply, IcmpType::Echo),
        (IcmpType::Tstamp, IcmpType::TstampReply),
        (IcmpType::Info, IcmpType::InfoReplay),
        (IcmpType::RtrSolicit, IcmpType::RtrAdvert),
        (IcmpType::RtrAdvert, IcmpType::RtrSolicit),
        (IcmpType::Mask, IcmpType::MaskReply)
    ]);
}

// 参考 https://github.com/corelight/pycommunityid/blob/master/communityid/icmp.py
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, IntoPrimitive)]
enum IcmpType {
    EchoReply = 0,
    Echo = 8,
    RtrAdvert = 9,
    RtrSolicit = 10,
    Tstamp = 13,
    TstampReply = 14,
    Info = 15,
    InfoReplay = 16,
    Mask = 17,
    MaskReply = 18,
}

pub(crate) fn get_port_equivalents(mtype: u16, mcode: u16) -> (u16, u16, bool) {
    match IcmpType::try_from(mtype) {
        Ok(mtype_obj) => match ICMP_TYPE_MAPPING.get(&mtype_obj) {
            Some(v) => return (mtype, (*v).into(), false),
            None => return (mtype, mcode, true),
        },
        Err(_) => return (mtype, mcode, true),
    }
}
