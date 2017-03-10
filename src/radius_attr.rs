use nom::{IResult,be_u8,be_u32};
use std::net::Ipv4Addr;

enum_from_primitive! {
#[derive(Debug,PartialEq)]
#[repr(u8)]
pub enum RadiusAttributeType {
    UserName = 1,
    UserPassword = 2,
    ChapPassword = 3,
    NasIPAddress = 4,
    NasPort = 5,
}
}

#[derive(Debug,PartialEq)]
pub enum RadiusAttribute<'a> {
    UserName(&'a[u8]),
    UserPassword(&'a[u8]),
    ChapPassword(u8,&'a[u8]),
    NasIPAddress(Ipv4Addr),
    NasPort(u32),

    Unknown(u8,&'a[u8]),
}


fn parse_attribute_content(i:&[u8], t:u8) -> IResult<&[u8],RadiusAttribute> {
    match t {
        1 => value!(i, RadiusAttribute::UserName(i)),
        2 => value!(i, RadiusAttribute::UserPassword(i)),
        3 => value!(i, RadiusAttribute::ChapPassword(i[0],&i[1..])),
        4 => map!(i, take!(4), |v:&[u8]| RadiusAttribute::NasIPAddress(Ipv4Addr::new(v[0],v[1],v[2],v[3]))),
        5 => map!(i, be_u32, |v| RadiusAttribute::NasPort(v)),
        _ => value!(i, RadiusAttribute::Unknown(t,i)),
    }
}

pub fn parse_radius_attribute(i:&[u8]) -> IResult<&[u8],RadiusAttribute> {
    do_parse!(i,
        t: be_u8 >>
        l: be_u8 >>
        v: flat_map!(take!(l-2),call!(parse_attribute_content,t)) >>
        ( v )
    )
}

