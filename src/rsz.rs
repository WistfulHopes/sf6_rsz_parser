use nom::bytes::complete::take;
use nom::combinator::map;
use nom::IResult;
use nom::multi::count;
use nom::number::complete::{le_f32, le_f64, le_i16, le_i32, le_i64, le_i8, le_u16, le_u32, le_u64, le_u8};
use nom::sequence::tuple;
use serde::{Deserialize, Serialize};
use crate::rsz::json_parser::{get_field_array_state, get_field_count, get_field_name, get_field_size, get_field_type, TypeIDs, get_field_alignment};

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
pub enum RSZValue {
    Object(RSZData),
    UserData(RSZUserDataInfo),
    Bool(bool),
    Float(f32),
    Double(f64),
    PlaneXZ(PlaneXZ),
    Float2(Float2),
    Float3(Float3),
    Float4(Float4),
    Fixed(f32),
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
    Unk(Vec<u8>),
    List(Vec<RSZValue>)
}

#[derive(Serialize, Deserialize)]
pub struct RSZField {
    pub name: String,
    pub value_type: TypeIDs,
    pub value: RSZValue,
}

#[derive(Serialize, Deserialize)]
pub struct RSZData {
    pub name: String,
    pub fields: Vec<RSZField>,
}

fn get_value(input: &[u8], offset: usize, field_type: TypeIDs, hash: u32, n: usize, alignment: usize) -> IResult<&[u8], RSZValue>
{
    let field_size = get_field_size(&hash, &n);
    let mut remainder: &[u8] = &input[offset.clone()..];
    let alignment_remainder = (16 - (input.len() - remainder.len()) % 16) % alignment;
    if alignment_remainder != 0 {
        remainder = &remainder[alignment_remainder..];
    }
    let base_remainder = remainder;
    let value = match field_type
    {
        TypeIDs::Resource => {
            /*let alignment_remainder = (16 -(input.len() - remainder.len()) % 16) % 4;
            if alignment_remainder != 0 {
                remainder = &remainder[alignment_remainder..];
            }*/
            let mut data: &[u8] = &[];
            (remainder, data) = take::<usize, &[u8], nom::error::Error<&[u8]>>(field_size)(remainder).unwrap();
            RSZValue::Unk(data.to_vec())
        }
        TypeIDs::UserData => {
            let (remaining_new, data) = parse_userdata_info(remainder).unwrap();
            remainder = remaining_new;
            RSZValue::UserData(data)
        }
        TypeIDs::Bool => {
            let mut bool = 0u8;
            (remainder, bool) = le_u8::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Bool(bool > 0)
        }
        TypeIDs::S8 => {
            let mut byte = 0i8;
            (remainder, byte) = le_i8::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Int8(byte.clone())
        }
        TypeIDs::U8 => {
            let mut ubyte = 0u8;
            (remainder, ubyte) = le_u8::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::UInt8(ubyte.clone())
        }
        TypeIDs::S16 => {
            let mut short = 0i16;
            (remainder, short) = le_i16::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Int16(short.clone())
        }
        TypeIDs::U16 => {
            let mut ushort = 0u16;
            (remainder, ushort) = le_u16::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::UInt16(ushort.clone())
        }
        TypeIDs::S32 => {
            let mut int = 0i32;
            (remainder, int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Int32(int.clone())
        }
        TypeIDs::U32 => {
            let mut uint = 0u32;
            (remainder, uint) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::UInt32(uint.clone())
        }
        TypeIDs::S64 => {
            let mut long = 0i64;
            (remainder, long) = le_i64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Int64(long.clone())
        }
        TypeIDs::U64 => {
            let mut ulong = 0u64;
            (remainder, ulong) = le_u64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::UInt64(ulong.clone())
        }
        TypeIDs::F32 => {
            let mut float = 0f32;
            (remainder, float) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Float(float.clone())
        }
        TypeIDs::F64 => {
            let mut double = 0f64;
            (remainder, double) = le_f64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Double(double.clone())
        }
        TypeIDs::String => {
            let mut size = 0u32;
            (remainder, size) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut data: &[u8] = &[];
            (remainder, data) = take::<usize, &[u8], nom::error::Error<&[u8]>>((size * 2) as usize)(remainder).unwrap();
            RSZValue::String(std::str::from_utf8(data).unwrap().to_owned())
        }
        TypeIDs::MBString => {
            /*let alignment_remainder = (16 -(input.len() - remainder.len()) % 16) % 4;
            if alignment_remainder != 0 {
                remainder = &remainder[alignment_remainder..];
            }*/
            let mut data: &[u8] = &[];
            (remainder, data) = take::<usize, &[u8], nom::error::Error<&[u8]>>(field_size)(remainder).unwrap();
            RSZValue::Unk(data.to_vec())
        }
        TypeIDs::Enum => {
            let mut int = 0i32;
            (remainder, int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Int32(int.clone())
        }
        TypeIDs::Uint2 => {
            let mut x = 0u32;
            (remainder, x) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0u32;
            (remainder, y) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::UInt2(UInt2{
                x: x.clone(),
                y: y.clone(),
            })
        }
        TypeIDs::Uint3 => {
            let mut x = 0u32;
            (remainder, x) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0u32;
            (remainder, y) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut z = 0u32;
            (remainder, z) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::UInt3(UInt3{
                x: x.clone(),
                y: y.clone(),
                z: z.clone(),
            })
        }
        TypeIDs::Uint4 => {
            let mut x = 0u32;
            (remainder, x) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0u32;
            (remainder, y) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut z = 0u32;
            (remainder, z) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut w = 0u32;
            (remainder, w) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::UInt4(UInt4{
                x: x.clone(),
                y: y.clone(),
                z: z.clone(),
                w: w.clone(),
            })
        }
        TypeIDs::Int2 => {
            let mut x = 0i32;
            (remainder, x) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0i32;
            (remainder, y) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Int2(Int2{
                x: x.clone(),
                y: y.clone(),
            })
        }
        TypeIDs::Int3 => {
            let mut x = 0i32;
            (remainder, x) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0i32;
            (remainder, y) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut z = 0i32;
            (remainder, z) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Int3(Int3{
                x: x.clone(),
                y: y.clone(),
                z: z.clone(),
            })
        }
        TypeIDs::Int4 => {
            let mut x = 0i32;
            (remainder, x) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0i32;
            (remainder, y) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut z = 0i32;
            (remainder, z) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut w = 0i32;
            (remainder, w) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Int4(Int4{
                x: x.clone(),
                y: y.clone(),
                z: z.clone(),
                w: w.clone(),
            })
        }
        TypeIDs::Float2 => {0f32;
            let mut x = 0f32;
            (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0f32;
            (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Float2(Float2{
                x: x.clone(),
                y: y.clone(),
            })
        }
        TypeIDs::Float3 => {
            let mut x = 0f32;
            (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0f32;
            (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut z = 0f32;
            (remainder, z) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Float3(Float3{
                x: x.clone(),
                y: y.clone(),
                z: z.clone(),
            })
        }
        TypeIDs::Float4 => {
            let mut x = 0f32;
            (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0f32;
            (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut z = 0f32;
            (remainder, z) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut w = 0f32;
            (remainder, w) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Float4(Float4{
                x: x.clone(),
                y: y.clone(),
                z: z.clone(),
                w: w.clone(),
            })
        }
        TypeIDs::Vec2 => {
            let mut x = 0f32;
            (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0f32;
            (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Float2(Float2{
                x: x.clone(),
                y: y.clone(),
            })
        }
        TypeIDs::Vec3 => {
            let mut x = 0f32;
            (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0f32;
            (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut z = 0f32;
            (remainder, z) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Float3(Float3{
                x: x.clone(),
                y: y.clone(),
                z: z.clone(),
            })
        }
        TypeIDs::Vec4 => {
            let mut x = 0f32;
            (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0f32;
            (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut z = 0f32;
            (remainder, z) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut w = 0f32;
            (remainder, w) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Float4(Float4{
                x: x.clone(),
                y: y.clone(),
                z: z.clone(),
                w: w.clone(),
            })
        }
        TypeIDs::Quaternion => {
            let mut x = 0f32;
            (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0f32;
            (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut z = 0f32;
            (remainder, z) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut w = 0f32;
            (remainder, w) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Float4(Float4{
                x: x.clone(),
                y: y.clone(),
                z: z.clone(),
                w: w.clone(),
            })
        }
        TypeIDs::Guid => {
            let mut data: &[u8] = &[];
            (remainder, data) = take::<usize, &[u8], nom::error::Error<&[u8]>>(field_size)(remainder).unwrap();
            RSZValue::GUID(GUID {
                uuid: data.try_into().unwrap()
            })
        }
        TypeIDs::Color => {
            let mut uint = 0u32;
            (remainder, uint) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::UInt32(uint.clone())
        }
        TypeIDs::DateTime => {
            let mut long = 0i64;
            (remainder, long) = le_i64::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Int64(long.clone())
        }
        TypeIDs::PlaneXZ => {
            let mut x = 0f32;
            (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut z = 0f32;
            (remainder, z) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::PlaneXZ(PlaneXZ{
                x: x.clone(),
                z: z.clone(),
            })
        }
        TypeIDs::Point => {
            let mut x = 0f32;
            (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0f32;
            (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Float2(Float2{
                x: x.clone(),
                y: y.clone(),
            })
        }
        TypeIDs::Range => {
            let mut x = 0f32;
            (remainder, x) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0f32;
            (remainder, y) = le_f32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::Float2(Float2{
                x: x.clone(),
                y: y.clone(),
            })
        }
        TypeIDs::RangeI => {
            let mut x = 0u32;
            (remainder, x) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let mut y = 0u32;
            (remainder, y) = le_u32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            RSZValue::UInt2(UInt2{
                x: x.clone(),
                y: y.clone(),
            })
        }
        TypeIDs::Uri => {
            let mut data: &[u8] = &[];
            (remainder, data) = take::<usize, &[u8], nom::error::Error<&[u8]>>(field_size)(remainder).unwrap();
            RSZValue::GUID(GUID {
                uuid: data.try_into().unwrap()
            })
        }
        TypeIDs::GameObjectRef => {
            let mut data: &[u8] = &[];
            (remainder, data) = take::<usize, &[u8], nom::error::Error<&[u8]>>(field_size)(remainder).unwrap();
            RSZValue::GUID(GUID {
                uuid: data.try_into().unwrap()
            })
        }
        TypeIDs::Sfix => {
            let mut fix = 0i32;
            (remainder, fix) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let float = fix.clone() as f32 / 65536.0f32;
            RSZValue::Float(float)
        }
        TypeIDs::Sfix2 => {
            let mut x_int = 0i32;
            (remainder, x_int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let x = x_int.clone() as f32 / 65536.0f32;
            let mut y_int = 0i32;
            (remainder, y_int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let y = y_int.clone() as f32 / 65536.0f32;
            RSZValue::Float2(Float2{
                x,
                y,
            })
        }
        TypeIDs::Sfix3 => {
            let mut x_int = 0i32;
            (remainder, x_int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let x = x_int.clone() as f32 / 65536.0f32;
            let mut y_int = 0i32;
            (remainder, y_int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let y = y_int.clone() as f32 / 65536.0f32;
            let mut z_int = 0i32;
            (remainder, z_int) = le_i32::<&[u8], nom::error::Error<&[u8]>>(remainder).unwrap();
            let z = z_int.clone() as f32 / 65536.0f32;
            RSZValue::Float3(Float3{
                x,
                y,
                z,
            })
        }
        TypeIDs::Sfix4 => {
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
            RSZValue::Float4(Float4{
                x,
                y,
                z,
                w,
            })
        }
        _ => {
            let mut data: &[u8] = &[];
            (remainder, data) = take::<usize, &[u8], nom::error::Error<&[u8]>>(field_size)(remainder).unwrap();
            RSZValue::Unk(data.to_vec())
        }
    };
    if field_type != TypeIDs::String {
        Ok((&base_remainder[field_size..], value))
    }
    else {
        Ok((remainder, value))
    }
}

fn parse_rsz_data(input: &[u8], offset: usize, hash: u32) -> IResult<&[u8], RSZData> {
    let name = json_parser::get_rsz_class_name(&hash).unwrap();
    let mut fields: Vec<RSZField> = vec![];
    let mut remainder: &[u8] = &input[offset.clone()..];
    for n in 0..get_field_count(&hash)
    {
        let field_type = get_field_type(&hash, &n);
        let is_list = get_field_array_state(&hash, &n).unwrap();
        let field_alignment = get_field_alignment(&hash, &n);
        if is_list
        {
            let mut new_remainder = remainder;
            let alignment_remainder = (16 -(input.len() - new_remainder.len()) % 16) % 4;
            if alignment_remainder != 0 {
                new_remainder = &new_remainder[alignment_remainder..];
            }
            let mut count: u32 = 0;
            (new_remainder, count) = le_u32::<&[u8], nom::error::Error<&[u8]>>(new_remainder).unwrap();
            let mut values: Vec<RSZValue> = vec![];
            if count > 1000
            {
                println!("Testing for bugs");
            }
            for _ in 0..count {
                let offset = input.len() - new_remainder.len();
                let (value_remainder, value) = get_value(input, offset, field_type, hash.clone(), n.clone(), field_alignment.clone()).unwrap();
                values.push(value);
                new_remainder = value_remainder;
            }
            let value = RSZValue::List(values);
            fields.push(RSZField{
                name: get_field_name(&hash, &n),
                value_type: field_type,
                value
            });
            remainder = new_remainder;
        }
        else {
            let offset = input.len() - remainder.len();
            let (new_remainder, value) = get_value(input, offset, field_type, hash.clone(), n, field_alignment).unwrap();
            fields.push(
                RSZField{
                    name: get_field_name(&hash, &n),
                    value_type: field_type,
                    value
                }
            );
            remainder = new_remainder;
        }
    };
    Ok((remainder, RSZData{
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
    #[serde(skip)]
    pub magic: u32,
    pub version: u32,
    #[serde(skip)]
    pub object_count: i32,
    #[serde(skip)]
    pub instance_count: i32,
    #[serde(skip)]
    pub userdata_count: i32,
    #[serde(skip)]
    pub reserved: i32,
    #[serde(skip)]
    pub instance_offsets: i64,
    #[serde(skip)]
    pub data_offset: i64,
    #[serde(skip)]
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
    #[serde(skip)]
    pub object_table: Vec<i32>,
    #[serde(skip)]
    pub instance_infos: Vec<InstanceInfo>,
    pub userdata_infos: Vec<RSZUserDataInfo>,
    pub data: Vec<RSZData>,
}

pub fn parse_rsz(input: &[u8], offset: usize) -> IResult<&[u8], RSZ> {
    let orig_remainder = &input[offset..];
    let (orig_remainder, header) = parse_rsz_header(orig_remainder).unwrap();
    let (orig_remainder, object_table) = count(le_i32::<&[u8], nom::error::Error<&[u8]>>, header.object_count as usize)(orig_remainder).unwrap();
    let (mut remainder, instance_infos) = count(parse_instance_info, header.instance_count as usize)(orig_remainder).unwrap();
    let alignment_remainder = (16 -(input.len() - remainder.len()) % 16) % 16;
    if alignment_remainder != 0 {
        remainder = &remainder[alignment_remainder..];
    }
    let (mut remainder, userdata_infos) = count(parse_userdata_info, header.userdata_count as usize)(remainder).unwrap();
    let mut data: Vec<RSZData> = vec![];
    for n in 1..header.instance_count {
        let new_offset = input.len() - remainder.len();
        let (remainder_new, cur_data) = parse_rsz_data(input, new_offset, instance_infos[n as usize].hash.clone()).unwrap();
        data.push(cur_data);
        remainder = remainder_new;
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