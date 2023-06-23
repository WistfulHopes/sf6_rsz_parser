use nom::{
    combinator::*,
    sequence::tuple,
    IResult,
};
use nom::multi::count;
use nom::number::complete::{le_i32, le_u32, le_u64};
use serde::{Deserialize, Serialize};

use crate::rsz::{parse_rsz, RSZ, GameObjectInfo, UserDataInfo, GameObjectRefInfo, ResourceInfo, parse_gobject_info, parse_gobject_ref_info, parse_resource_info, parse_userdata_info};

#[derive(Serialize, Deserialize)]
pub struct PrefabHeader {
    #[serde(skip)]
    pub magic: u32,
    pub info_count: i32,
    pub resource_count: i32,
    pub gameobject_ref_info_count: i32,
    pub userdata_count: i32,
    #[serde(skip)]
    pub reserved: i32,
    #[serde(skip)]
    pub gameobject_ref_info_tbl: u64,
    #[serde(skip)]
    pub resource_info_tbl: u64,
    #[serde(skip)]
    pub userdata_info_tbl: u64,
    #[serde(skip)]
    pub data_offset: u64,
}

fn parse_prefab_header(input: &[u8]) -> IResult<&[u8], PrefabHeader>
{
    map(
        tuple((
            le_u32,
            le_i32,
            le_i32,
            le_i32,
            le_i32,
            le_i32,
            le_u64,
            le_u64,
            le_u64,
            le_u64,
        )),
        |(
             magic,
             info_count,
             resource_count,
             gameobject_ref_info_count,
             userdata_count,
             reserved,
             gameobject_ref_info_tbl,
             resource_info_tbl,
             userdata_info_tbl,
             data_offset,
         )|{
            PrefabHeader {
                magic,
                info_count,
                resource_count,
                gameobject_ref_info_count,
                userdata_count,
                reserved,
                gameobject_ref_info_tbl,
                resource_info_tbl,
                userdata_info_tbl,
                data_offset,
            }
        }
    )(input)
}

#[derive(Serialize, Deserialize)]
pub struct Prefab {
    pub header: PrefabHeader,
    pub gameobject_infos: Vec<GameObjectInfo>,
    pub userdata_infos: Vec<UserDataInfo>,
    pub gameobject_ref_infos: Vec<GameObjectRefInfo>,
    pub resource_infos: Vec<ResourceInfo>,
    pub gameobject: RSZ,
}

pub fn parse_prefab(input: &[u8]) -> IResult<&[u8], Prefab> {
    let (remainder, header) = parse_prefab_header(input).unwrap();
    let (remainder, gameobject_infos) = count(parse_gobject_info, header.info_count as usize)(remainder).unwrap();
    let (mut remainder, gameobject_ref_infos) = count(parse_gobject_ref_info, header.gameobject_ref_info_count as usize)(remainder).unwrap();
    let alignment_remainder = (16 -(input.len() - remainder.len()) % 16) % 16;
    if alignment_remainder != 0 {
        remainder = &remainder[alignment_remainder..];
    }
    let mut resource_infos: Vec<ResourceInfo> = vec![];
    for _ in 0..header.resource_count {
        let offset = input.len() - remainder.len();
        let (new_remainder, resource_info) = parse_resource_info(input, offset).unwrap();
        remainder = new_remainder;
        resource_infos.push(resource_info);
    }
    let alignment_remainder = (16 -(input.len() - remainder.len()) % 16) % 16;
    if alignment_remainder != 0 {
        remainder = &remainder[alignment_remainder..];
    }
    let mut userdata_infos: Vec<UserDataInfo> = vec![];
    for _ in 0..header.userdata_count {
        let offset = input.len() - remainder.len();
        let (new_remainder, userdata_info) = parse_userdata_info(input, offset).unwrap();
        remainder = new_remainder;
        userdata_infos.push(userdata_info);
    }
    
    let (remainder, gameobject) = parse_rsz(input, header.data_offset as usize).unwrap();
    
    Ok((
        remainder, Prefab {
            header,
            gameobject_infos,
            userdata_infos,
            gameobject_ref_infos,
            resource_infos,
            gameobject,
        }
    ))
}