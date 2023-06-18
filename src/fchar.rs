use nom::{
    combinator::*,
    sequence::tuple,
    IResult,
};
use nom::multi::count;
use nom::number::complete::{le_i32, le_u32, le_u64};
use serde::{Deserialize, Serialize};
use num_derive::FromPrimitive;

use crate::rsz::{parse_rsz, RSZ};

#[derive(Serialize, Deserialize)]
pub struct CharacterAssetHeader {
    pub version: u32,
    #[serde(skip)]
    pub magic: u32,
    #[serde(skip)]
    pub id_table_offset: u64,
    #[serde(skip)]
    pub parent_id_table_offset: u64,
    #[serde(skip)]
    pub action_list_table_offset: u64,
    #[serde(skip)]
    pub data_id_table_offset: u64,
    #[serde(skip)]
    pub data_list_table_offset: u64,
    #[serde(skip)]
    pub string_object_offset: u64,
    #[serde(skip)]
    pub string_offset: u64,
    #[serde(skip)]
    pub object_table_rsz_offset: u64,
    #[serde(skip)]
    pub object_table_rsz_end: u64,
    #[serde(skip)]
    pub object_count: u32,
    #[serde(skip)]
    pub style_count: u32,
    #[serde(skip)]
    pub data_count: u32,
    #[serde(skip)]
    pub string_count: u32,
}

fn parse_fchar_header(input: &[u8]) -> IResult<&[u8], CharacterAssetHeader>
{
    map(
        tuple((
            le_u32,
            le_u32,
            le_u64,
            le_u64,
            le_u64,
            le_u64,
            le_u64,
            le_u64,
            le_u64,
            le_u64,
            le_u64,
            le_u32,
            le_u32,
            le_u32,
            le_u32,
        )),
        |(
             version,
             magic,
             id_table_offset,
             parent_id_table_offset,
             action_list_table_offset,
             data_id_table_offset,
             data_list_table_offset,
             string_object_offset,
             string_offset,
             object_table_rsz_offset,
             object_table_rsz_end,
             object_count,
             style_count,
             data_count,
             string_count,
         )|{
            CharacterAssetHeader {
                version,
                magic,
                id_table_offset,
                parent_id_table_offset,
                action_list_table_offset,
                data_id_table_offset,
                data_list_table_offset,
                string_object_offset,
                string_offset,
                object_table_rsz_offset,
                object_table_rsz_end,
                object_count,
                style_count,
                data_count,
                string_count,
            }
        }
    )(input)
}

#[derive(Serialize, Deserialize, Default)]
pub struct ActionListTable {
    pub action_list_table_offset: u64,
    pub style_data_offset: Vec<u64>,
    pub action_list_offset: u64,
    pub action_rsz: u64,
    pub data_id_table_offset: u64,
    pub action_list_count: u32,
    pub object_count: u32,
}

fn parse_action_list_table(input: &[u8], offset: usize, style_count: u32) -> IResult<&[u8], ActionListTable>
{
    let remainder = &input[offset..];
    let (remainder, action_list_table_offset) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
    let (_, style_data_offset) = count(le_u64::<&[u8], nom::error::Error<&[u8]>>, style_count as usize - 1)(remainder).unwrap();
    let remainder = &input[action_list_table_offset as usize..];
    let (remainder, action_list_offset) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
    let (remainder, action_rsz) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
    let (remainder, data_id_table_offset) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
    let (remainder, action_list_count) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
    let (remainder, object_count) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
    return Ok((remainder, ActionListTable {
        action_list_table_offset,
        style_data_offset,
        action_list_offset,
        action_rsz,
        data_id_table_offset,
        action_list_count,
        object_count,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct StyleData {
    #[serde(skip)]
    pub data_start_offset: u64,
    #[serde(skip)]
    pub rsz_offset: u64,
    #[serde(skip)]
    pub data_end_offset: u64,
    pub rsz: RSZ,
}

fn parse_style_data(input: &[u8], offset: usize) -> IResult<&[u8], StyleData> {
    let remainder = &input[offset..];
    let (remainder, data_start_offset) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
    let (remainder, rsz_offset) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
    let (remainder, data_end_offset) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
    let (_, rsz) = parse_rsz(input, rsz_offset as usize).unwrap();
    return Ok((remainder, StyleData{
        data_start_offset,
        rsz_offset,
        data_end_offset,
        rsz
    }))
}

#[derive(Serialize, Deserialize)]
pub struct ActionData {
    pub action_id: i32,
    pub frames: i32,
    pub key_start_frame: i32,
    pub key_end_frame: i32,
}

fn parse_action_data(input: &[u8]) -> IResult<&[u8], ActionData>
{
    map(
        tuple((
            le_i32,
            le_i32,
            le_i32,
            le_i32,
        )),
        |(
             action_id,
             frames,
             key_start_frame,
             key_end_frame,
         )|{
            ActionData {
                action_id,
                frames,
                key_start_frame,
                key_end_frame,
            }
        }
    )(input)
}

#[derive(Serialize, Deserialize)]
pub struct KeyData {
    pub key_start_frame: i32,
    pub key_end_frame: i32,
}

fn parse_key_data(input: &[u8]) -> IResult<&[u8], KeyData>
{
    map(
        tuple((
            le_i32,
            le_i32,
        )),
        |(
             key_start_frame,
             key_end_frame,
         )|{
            KeyData {
                key_start_frame,
                key_end_frame,
            }
        }
    )(input)
}

#[derive(Serialize, Deserialize)]
pub struct ObjectData {
    pub data_count: i32,
    pub reserved: i32,
    pub key_data: Vec<KeyData>
}

fn parse_object_data(input: &[u8]) -> IResult<&[u8], ObjectData>
{
    let (remainder, data_count) = le_i32::<&[u8], nom::error::Error<&[u8]>>(input).unwrap();
    let (remainder, reserved) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
    let (remainder, key_data) = count(parse_key_data, data_count as usize)(remainder).unwrap();
    Ok((remainder, ObjectData {
        data_count,
        reserved,
        key_data,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct ActionListInfo {
    #[serde(skip)]
    pub action_offset: u64,
    #[serde(skip)]
    pub data_start_offset: u64,
    #[serde(skip)]
    pub rsz_offset: u64,
    #[serde(skip)]
    pub rsz_end: u64,
    #[serde(skip)]
    pub action_count: u32,
    #[serde(skip)]
    pub object_count: u32,
    pub action_data: ActionData,
}

fn parse_action_list_info(input: &[u8], offset: usize) -> IResult<&[u8], ActionListInfo> {
    let remainder = &input[offset..];
    let (remainder, action_offset) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
    let action_start = &input[action_offset as usize..];
    let (remainder_new, data_start_offset) = le_u64::<&[u8], nom::error::Error<&[u8]>>(action_start).unwrap();
    let (remainder_new, rsz_offset) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder_new).unwrap();
    let (remainder_new, rsz_end) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder_new).unwrap();
    let (remainder_new, action_count) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder_new).unwrap();
    let (remainder_new, object_count) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder_new).unwrap();
    let (_, action_data) = parse_action_data(remainder_new).unwrap();
    return Ok((remainder, ActionListInfo{
        action_offset,
        data_start_offset,
        rsz_offset,
        rsz_end,
        action_count,
        object_count,
        action_data
    }))
}

#[derive(Serialize, Deserialize)]
pub struct ObjectInfo {
    #[serde(skip)]
    pub object_offset: u64,
    #[serde(skip)]
    pub data_start_offset: u64,
    #[serde(skip)]
    pub rsz_offset: u64,
    #[serde(skip)]
    pub rsz_end: u64,
    pub object_data: ObjectData,
}

fn parse_object_info(input: &[u8], offset: usize) -> IResult<&[u8], ObjectInfo> {
    let remainder = &input[offset..];
    let (remainder, object_offset) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
    let object_start = &input[object_offset as usize..];
    let (remainder_new, data_start_offset) = le_u64::<&[u8], nom::error::Error<&[u8]>>(object_start).unwrap();
    let (remainder_new, rsz_offset) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder_new).unwrap();
    let (remainder_new, rsz_end) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder_new).unwrap();
    let (_, object_data) = parse_object_data(remainder_new).unwrap();

    Ok((remainder, ObjectInfo{
        object_offset,
        data_start_offset,
        rsz_offset,
        rsz_end,
        object_data,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct Object {
    pub info: ObjectInfo,
    pub action: RSZ,
}

fn parse_object(input: &[u8], offset: usize) -> IResult<&[u8], Object> {
    let (remainder_new, info) = parse_object_info(input, offset).unwrap();
    let (_, action) = parse_rsz(input, info.rsz_offset.clone() as usize).unwrap();

    Ok((remainder_new, Object{
        info,
        action,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct ActionList {
    pub info: ActionListInfo,
    pub action: RSZ,
    pub objects: Vec<Object>,
}

fn parse_action_list(input: &[u8], offset: usize) -> IResult<&[u8], ActionList> {
    let (_, info) = parse_action_list_info(input, offset).unwrap();
    let (remainder_new, action) = parse_rsz(input, info.rsz_offset.clone() as usize).unwrap();
    let mut objects: Vec<Object> = vec![];
    for n in 0..info.object_count.clone() {
        let offset = (info.data_start_offset.clone() + 8 * n as u64) as usize;
        let (_, object) = parse_object(input, offset).unwrap();
        objects.push(object);
    };
    Ok((remainder_new, ActionList{
        info,
        action,
        objects,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct DataListInfo {
    pub data_start_offset: u64,
    pub rsz_offset: u64,
    pub data_end_offset: u64,
    pub data_count: u32,
}

fn parse_data_list_info(input: &[u8]) -> IResult<&[u8], DataListInfo>
{
    map(
        tuple((
            le_u64,
            le_u64,
            le_u64,
            le_u32,
        )),
        |(
             data_start_offset,
             rsz_offset,
             data_end_offset,
             data_count,
         )|{
            DataListInfo {
                data_start_offset,
                rsz_offset,
                data_end_offset,
                data_count,
            }
        }
    )(input)
}

#[derive(Serialize, Deserialize)]
pub struct DataListItem {
    #[serde(skip)]
    pub data_list_offset: u64,
    pub info: DataListInfo,
    pub data_ids: Vec<u32>,
    pub data_rsz: RSZ,
}

fn parse_data_list_item(input: &[u8], offset: usize) -> IResult<&[u8], DataListItem> {
    let remainder = &input[offset..];
    let (remainder, data_list_offset) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
    let data_remainder = &input[data_list_offset as usize..];
    let (data_remainder, info) = parse_data_list_info(data_remainder).unwrap();
    let (_, data_ids) = count(le_u32::<&[u8], nom::error::Error<&[u8]>>, info.data_count as usize)(data_remainder).unwrap();
    let (_, data_rsz) = parse_rsz(input, info.rsz_offset.clone() as usize).unwrap();
    Ok((remainder, DataListItem{
        data_list_offset,
        info,
        data_ids,
        data_rsz,
    }))
}

#[derive(Serialize, Deserialize, FromPrimitive, PartialEq, Eq, Clone)]
pub enum DataId {
    AttackDataParams = 0,
    ChargeParamSub = 1,
    CommandParamSub = 2,
    CommandGroup = 3,
    StrikeData = 4,
    ProjectileData = 15,
    TriggerGroup = 16,
    Trigger = 17,
    StrikeBox = 20,
    ProjectileBox = 21,
    ThrowBox = 22,
    ProximityBox = 23,
    ReflectBox = 24,
    PushBox = 25,
    UniqueBox = 26,
    ThrowHurtBox = 30,
    HurtBox = 31,
    OtherBox = 32,
    GimmickBox = 33,
    CameraBox = 34,
    PartsBox = 35,
    MissionData = 50,
    AttackDataKarma = 70,
    AssistComboRecipeData = 75,
    VoiceFacialData = 80,
    VoiceFacialDataEN = 81,
    CommonOffset = 100,
    AttackOwnerCurve = 101,
    AttackTargetCurve = 102,
    ScreenVibration = 103,
    CameraData = 104,
    AttackDataCommon = 105,
    RectCommon = 106,
}

#[derive(Serialize, Deserialize)]
pub struct CharacterAsset {
    pub header: CharacterAssetHeader,
    #[serde(skip)]
    pub id_table: Vec<i32>,
    #[serde(skip)]
    pub parent_id_table: Vec<i32>,
    #[serde(skip)]
    pub action_list_table: ActionListTable,
    pub default_style_data: RSZ,
    pub style_data: Vec<StyleData>,
    pub action_list: Vec<ActionList>,
    pub data_id_table: Vec<DataId>,
    pub data_list_table: Vec<DataListItem>,
    pub personal_data: RSZ,
}

pub fn parse_fchar(input: &[u8]) -> IResult<&[u8], CharacterAsset> {
    println!("Parsing fchar file...");
    let (remainder, header) = parse_fchar_header(input).unwrap();
    let (remainder, id_table) = count(le_i32::<&[u8], nom::error::Error<&[u8]>>, header.style_count as usize)(remainder).unwrap();
    let (mut remainder, parent_id_table) = count(le_i32::<&[u8], nom::error::Error<&[u8]>>, header.style_count as usize)(remainder).unwrap();
    let alignment_remainder = (16 - (input.len() - remainder.len()) % 16) % 16;
    if alignment_remainder != 0 {
        remainder = &remainder[alignment_remainder..];
    }
    let offset = input.len() - remainder.len();
    let (mut remainder, action_list_table) = parse_action_list_table(input, offset, header.style_count).unwrap();
    println!("Header parsed!");
    println!("Parsing style data...");
    let (_, default_style_data) = parse_rsz(input, action_list_table.action_rsz as usize).unwrap();
    let mut style_data: Vec<StyleData> = vec![];
    for n in 0..header.style_count - 1 {
        let (_, style_data_inst) = parse_style_data(input, action_list_table.style_data_offset[n as usize] as usize).unwrap();
        style_data.push(style_data_inst);
    }
    println!("Style data parsed!");
    let mut action_list: Vec<ActionList> = vec![];
    println!("Parsing action list...");
    for _ in 0..action_list_table.action_list_count {
        let offset = input.len() - remainder.len();
        let (_, action) = parse_action_list(input, offset).unwrap();
        action_list.push(action);
        remainder = &remainder[8..];
    }
    println!("Action list parsed!");

    println!("Parsing data tables...");
    let data_id_remainder = &input[header.data_id_table_offset.clone() as usize..];
    let (_, data_id_u32_table) = count(le_u32::<&[u8], nom::error::Error<&[u8]>>, header.data_count as usize)(data_id_remainder).unwrap();
    let mut data_id_table: Vec<DataId> = vec![];
    for data_id in data_id_u32_table {
        data_id_table.push(num::FromPrimitive::from_u32(data_id).unwrap());
    }
    let mut data_list_remainder = &input[header.data_list_table_offset.clone() as usize..];
    let mut data_list_table: Vec<DataListItem> = vec![];
    for _ in 0..header.data_count {
        let offset = input.len() - data_list_remainder.len();
        let (remainder_new, data_list_item) = parse_data_list_item(input, offset).unwrap();
        data_list_remainder = remainder_new;
        data_list_table.push(data_list_item);
    }
    println!("Data tables parsed!");

    println!("Parsing personal data...");
    let (_, personal_data) = parse_rsz(input, header.object_table_rsz_offset.clone() as usize).unwrap();
    println!("Personal data parsed!");

    println!("Fchar file parsed!");

    Ok((input, CharacterAsset {
        header,
        id_table,
        parent_id_table,
        action_list_table,
        default_style_data,
        style_data,
        action_list,
        data_id_table,
        data_list_table,
        personal_data,
    }))
}