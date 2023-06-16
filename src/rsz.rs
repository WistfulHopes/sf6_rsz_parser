use indicatif::ProgressBar;
use nom::bytes::complete::take;
use nom::combinator::map;
use nom::IResult;
use nom::multi::count;
use nom::number::complete::{le_f32, le_f64, le_i16, le_i32, le_i64, le_i8, le_u16, le_u32, le_u64, le_u8};
use nom::sequence::tuple;
use serde::{Deserialize, Serialize};
use crate::rsz::json_parser::{get_field_array_state, get_field_count, get_field_size, get_field_type, TypeIDs};

pub mod json_parser;

#[derive(Serialize, Deserialize)]
pub struct InstanceInfo {
    pub hash: u32,
    pub crc: u32,
}

fn parse_instance_info(input: &[u8]) -> IResult<&[u8], InstanceInfo> {
    map(
        tuple((
            le_u32,
            le_u32,
        )),
        |(
            hash,
            crc,
         )|{
            InstanceInfo {
                hash,
                crc,
            }
        }
    )(input)
}

#[derive(Serialize, Deserialize)]
pub struct PlaneXZ {
    pub x: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Int2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Int3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Int4 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub w: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UInt2 {
    pub x: u32,
    pub y: u32,
}

#[derive(Serialize, Deserialize)]
pub struct UInt3 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Serialize, Deserialize)]
pub struct UInt4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub w: u32,
}

#[derive(Serialize, Deserialize)]
pub struct GUID {
    uuid: [u8; 16],
}

#[derive(Serialize, Deserialize)]
pub enum RSZField {
    Object(RSZData),
    UserData(RSZUserDataInfo),
    Bool(bool),
    Float(f32),
    Double(f64),
    PlaneXZ(PlaneXZ),
    Float2(Float2),
    Float3(Float3),
    Float4(Float4),
    GUID(GUID),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Int2(Int2),
    Int3(Int3),
    Int4(Int4),
    UInt2(UInt2),
    UInt3(UInt3),
    UInt4(UInt4),
    String(String),
    Unk(Vec<u8>)
}

#[derive(Serialize, Deserialize)]
pub struct RSZData {
    pub name: String,
    pub fields: Vec<RSZField>,
}

fn parse_rsz_data(input: &[u8], hash: u32) -> IResult<&[u8], RSZData> {
    let json = json_parser::parse_json().unwrap();
    let name = json_parser::get_rsz_class_name(&json, &hash).unwrap();
    let mut fields: Vec<RSZField> = vec![];
    let mut remainder: &[u8] = input;
    for n in 0..get_field_count(&json, &hash)
    {
        let field_type = get_field_type(&json, &hash, &n);
        let field_size = get_field_size(&json, &hash, &n);
        //let is_list = get_field_array_state(&json, &hash, &n);
        fields.push(
            match field_type
            {
                TypeIDs::ResourceTid => {
                    /*let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }*/
                    let mut data: &[u8] = &[];
                    (remainder, data) = take::<usize, &[u8], nom::error::Error<&[u8]>>(field_size)(remainder).unwrap();
                    RSZField::Unk(data.to_vec())
                }
                TypeIDs::UserDataTid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let (remaining_new, data) = parse_userdata_info(remainder).unwrap();
                    remainder = remaining_new;
                    RSZField::UserData(data)
                }
                TypeIDs::BoolTid => {
                    let mut bool = 0u8;
                    (remainder, bool) = le_u8::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Bool(bool > 0)
                }
                TypeIDs::S8Tid => {
                    let mut byte = 0i8;
                    (remainder, byte) = le_i8::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Int8(byte.clone())
                }
                TypeIDs::U8Tid => {
                    let mut ubyte = 0u8;
                    (remainder, ubyte) = le_u8::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::UInt8(ubyte.clone())
                }
                TypeIDs::S16Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut short = 0i16;
                    (remainder, short) = le_i16::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Int16(short.clone())
                }
                TypeIDs::U16Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut ushort = 0u16;
                    (remainder, ushort) = le_u16::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::UInt16(ushort.clone())
                }
                TypeIDs::S32Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut int = 0i32;
                    (remainder, int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Int32(int.clone())
                }
                TypeIDs::U32Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut uint = 0u32;
                    (remainder, uint) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::UInt32(uint.clone())
                }
                TypeIDs::S64Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut long = 0i64;
                    (remainder, long) = le_i64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Int64(long.clone())
                }
                TypeIDs::U64Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut ulong = 0u64;
                    (remainder, ulong) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::UInt64(ulong.clone())
                }
                TypeIDs::F32Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut float = 0f32;
                    (remainder, float) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Float(float.clone())
                }
                TypeIDs::F64Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut double = 0f64;
                    (remainder, double) = le_f64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Double(double.clone())
                }
                TypeIDs::StringTid => {
                    /*let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }*/
                    let mut data: &[u8] = &[];
                    (remainder, data) = take::<usize, &[u8], nom::error::Error<&[u8]>>(field_size)(remainder).unwrap();
                    RSZField::Unk(data.to_vec())
                }
                TypeIDs::MBStringTid => {
                    /*let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }*/
                    let mut data: &[u8] = &[];
                    (remainder, data) = take::<usize, &[u8], nom::error::Error<&[u8]>>(field_size)(remainder).unwrap();
                    RSZField::Unk(data.to_vec())
                }
                TypeIDs::EnumTid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut int = 0i32;
                    (remainder, int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Int32(int.clone())
                }
                TypeIDs::Uint2Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0u32;
                    (remainder, x) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0u32;
                    (remainder, y) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::UInt2(UInt2{
                        x: x.clone(),
                        y: y.clone(),
                    })
                }
                TypeIDs::Uint3Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0u32;
                    (remainder, x) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0u32;
                    (remainder, y) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut z = 0u32;
                    (remainder, z) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::UInt3(UInt3{
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone(),
                    })
                }
                TypeIDs::Uint4Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0u32;
                    (remainder, x) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0u32;
                    (remainder, y) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut z = 0u32;
                    (remainder, z) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut w = 0u32;
                    (remainder, w) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::UInt4(UInt4{
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone(),
                        w: w.clone(),
                    })
                }
                TypeIDs::Int2Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0i32;
                    (remainder, x) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0i32;
                    (remainder, y) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Int2(Int2{
                        x: x.clone(),
                        y: y.clone(),
                    })
                }
                TypeIDs::Int3Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0i32;
                    (remainder, x) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0i32;
                    (remainder, y) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut z = 0i32;
                    (remainder, z) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Int3(Int3{
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone(),
                    })
                }
                TypeIDs::Int4Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0i32;
                    (remainder, x) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0i32;
                    (remainder, y) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut z = 0i32;
                    (remainder, z) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut w = 0i32;
                    (remainder, w) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Int4(Int4{
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone(),
                        w: w.clone(),
                    })
                }
                TypeIDs::Float2Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0f32;
                    (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0f32;
                    (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Float2(Float2{
                        x: x.clone(),
                        y: y.clone(),
                    })
                }
                TypeIDs::Float3Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0f32;
                    (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0f32;
                    (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut z = 0f32;
                    (remainder, z) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Float3(Float3{
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone(),
                    })
                }
                TypeIDs::Float4Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0f32;
                    (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0f32;
                    (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut z = 0f32;
                    (remainder, z) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut w = 0f32;
                    (remainder, w) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Float4(Float4{
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone(),
                        w: w.clone(),
                    })
                }
                TypeIDs::Vec2Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0f32;
                    (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0f32;
                    (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Float2(Float2{
                        x: x.clone(),
                        y: y.clone(),
                    })
                }
                TypeIDs::Vec3Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0f32;
                    (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0f32;
                    (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut z = 0f32;
                    (remainder, z) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Float3(Float3{
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone(),
                    })
                }
                TypeIDs::Vec4Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0f32;
                    (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0f32;
                    (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut z = 0f32;
                    (remainder, z) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut w = 0f32;
                    (remainder, w) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Float4(Float4{
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone(),
                        w: w.clone(),
                    })
                }
                TypeIDs::QuaternionTid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0f32;
                    (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0f32;
                    (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut z = 0f32;
                    (remainder, z) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut w = 0f32;
                    (remainder, w) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Float4(Float4{
                        x: x.clone(),
                        y: y.clone(),
                        z: z.clone(),
                        w: w.clone(),
                    })
                }
                TypeIDs::GuidTid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 8;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut data: &[u8] = &[];
                    (remainder, data) = take::<usize, &[u8], nom::error::Error<&[u8]>>(field_size)(remainder).unwrap();
                    RSZField::GUID(GUID {
                        uuid: data.try_into().unwrap()
                    })
                }
                TypeIDs::ColorTid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut uint = 0u32;
                    (remainder, uint) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::UInt32(uint.clone())
                }
                TypeIDs::DateTimeTid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut long = 0i64;
                    (remainder, long) = le_i64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Int64(long.clone())
                }
                TypeIDs::PlaneXZTid => {

                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0f32;
                    (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut z = 0f32;
                    (remainder, z) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::PlaneXZ(PlaneXZ{
                        x: x.clone(),
                        z: z.clone(),
                    })
                }
                TypeIDs::PointTid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0f32;
                    (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0f32;
                    (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Float2(Float2{
                        x: x.clone(),
                        y: y.clone(),
                    })
                }
                TypeIDs::RangeTid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0f32;
                    (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0f32;
                    (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::Float2(Float2{
                        x: x.clone(),
                        y: y.clone(),
                    })
                }
                TypeIDs::RangeITid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x = 0u32;
                    (remainder, x) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let mut y = 0u32;
                    (remainder, y) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    RSZField::UInt2(UInt2{
                        x: x.clone(),
                        y: y.clone(),
                    })
                }
                TypeIDs::UriTid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 8;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut data: &[u8] = &[];
                    (remainder, data) = take::<usize, &[u8], nom::error::Error<&[u8]>>(field_size)(remainder).unwrap();
                    RSZField::GUID(GUID {
                        uuid: data.try_into().unwrap()
                    })
                }
                TypeIDs::GameObjectRefTid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 8;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut data: &[u8] = &[];
                    (remainder, data) = take::<usize, &[u8], nom::error::Error<&[u8]>>(field_size)(remainder).unwrap();
                    RSZField::GUID(GUID {
                        uuid: data.try_into().unwrap()
                    })
                }
                TypeIDs::SfixTid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut fix = 0i32;
                    (remainder, fix) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let float = fix.clone() as f32 / 65536.0f32;
                    RSZField::Float(float)
                }
                TypeIDs::Sfix2Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x_int = 0i32;
                    (remainder, x_int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let x = x_int.clone() as f32 / 65536.0f32;
                    let mut y_int = 0i32;
                    (remainder, y_int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let y = y_int.clone() as f32 / 65536.0f32;
                    RSZField::Float2(Float2{
                        x,
                        y,
                    })
                }
                TypeIDs::Sfix3Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x_int = 0i32;
                    (remainder, x_int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let x = x_int.clone() as f32 / 65536.0f32;
                    let mut y_int = 0i32;
                    (remainder, y_int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let y = y_int.clone() as f32 / 65536.0f32;
                    let mut z_int = 0i32;
                    (remainder, z_int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let z = z_int.clone() as f32 / 65536.0f32;
                    RSZField::Float3(Float3{
                        x,
                        y,
                        z,
                    })
                }
                TypeIDs::Sfix4Tid => {
                    let alignment_remainder = (input.len() - remainder.len()) % 4;
                    if alignment_remainder != 0 {
                        remainder = &remainder[alignment_remainder..];
                    }
                    let mut x_int = 0i32;
                    (remainder, x_int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let x = x_int.clone() as f32 / 65536.0f32;
                    let mut y_int = 0i32;
                    (remainder, y_int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let y = y_int.clone() as f32 / 65536.0f32;
                    let mut z_int = 0i32;
                    (remainder, z_int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let z = z_int.clone() as f32 / 65536.0f32;
                    let mut w_int = 0i32;
                    (remainder, w_int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
                    let w = w_int.clone() as f32 / 65536.0f32;
                    RSZField::Float4(Float4{
                        x,
                        y,
                        z,
                        w,
                    })
                }
                _ => {
                    let mut data: &[u8] = &[];
                    (remainder, data) = take::<usize, &[u8], nom::error::Error<&[u8]>>(field_size)(remainder).unwrap();
                    RSZField::Unk(data.to_vec())
                }
            }
        );
    };
    Ok((input, RSZData{
        name,
        fields,
    }))
}

#[derive(Serialize, Deserialize)]
pub struct RSZUserDataInfo {
    pub instance_id: u32,
    pub type_id: u32,
}

fn parse_userdata_info(input: &[u8]) -> IResult<&[u8], RSZUserDataInfo> {
    map(
        tuple((
            le_u32,
            le_u32,
        )),
        |(
             instance_id,
             type_id,
         )|{
            RSZUserDataInfo {
                instance_id,
                type_id,
            }
        }
    )(input)
}

#[derive(Serialize, Deserialize)]
pub struct RSZHeader {
    pub magic: u32,
    pub version: u32,
    pub object_count: i32,
    pub instance_count: i32,
    pub userdata_count: i32,
    pub reserved: i32,
    pub instance_offsets: i64,
    pub data_offset: i64,
    pub userdata_offset: i64,
}

fn parse_rsz_header(input: &[u8]) -> IResult<&[u8], RSZHeader> {
    map(
        tuple((
            le_u32,
            le_u32,
            le_i32,
            le_i32,
            le_i32,
            le_i32,
            le_i64,
            le_i64,
            le_i64,
        )),
        |(
             magic,
             version,
             object_count,
             instance_count,
             userdata_count,
             reserved,
             instance_offsets,
             data_offset,
             userdata_offset,
         )|{
            RSZHeader {
                magic,
                version,
                object_count,
                instance_count,
                userdata_count,
                reserved,
                instance_offsets,
                data_offset,
                userdata_offset,
            }
        }
    )(input)
}

#[derive(Serialize, Deserialize)]
pub struct RSZ {
    pub header: RSZHeader,
    pub object_table: Vec<i32>,
    pub instance_infos: Vec<InstanceInfo>,
    pub userdata_infos: Vec<RSZUserDataInfo>,
    pub data: Vec<RSZData>,
}

pub fn parse_rsz(input: &[u8], log: bool) -> IResult<&[u8], RSZ> {
    let (remainder, header) = parse_rsz_header(input).unwrap();
    let (remainder, object_table) = count(le_i32::<&[u8], nom::error::Error<&[u8]>>, header.object_count as usize)(remainder).unwrap();
    let (mut remainder, instance_infos) = count(parse_instance_info, header.instance_count as usize)(remainder).unwrap();
    let alignment_remainder = (input.len() - remainder.len()) % 16;
    if alignment_remainder != 0 {
        remainder = &remainder[alignment_remainder..];
    }
    let (mut remainder, userdata_infos) = count(parse_userdata_info, header.userdata_count as usize)(remainder).unwrap();
    let mut data: Vec<RSZData> = vec![];
    if log {
        println!("Parsing RSZ data...");
        let bar = ProgressBar::new(header.instance_count.clone() as u64);
        for n in 1..header.instance_count {
            let (remainder_new, cur_data) = parse_rsz_data(remainder, instance_infos[n as usize].hash.clone()).unwrap();
            data.push(cur_data);
            remainder = remainder_new;
            if log {
                bar.inc(1);
            }
        }
        bar.finish();
        println!("RSZ data parsed!");
    }
    else {
        for n in 1..header.instance_count {
            let (remainder_new, cur_data) = parse_rsz_data(remainder, instance_infos[n as usize].hash.clone()).unwrap();
            data.push(cur_data);
            remainder = remainder_new;
        }
    }

    Ok((remainder,
        RSZ {
            header,
            object_table,
            instance_infos,
            userdata_infos,
            data
        }
    ))
}