use std::{fmt, fs::File, io::Read};
use serde_json::{Value, json};
use std::sync::Mutex;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref JSON: Mutex<Value> = Mutex::new(json!(null));
}

type RSZResult<T> = Result<T, RSZError>;

#[derive(Debug, Clone)]
pub struct RSZError;

impl fmt::Display for RSZError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RSZ json error")
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum TypeIDs {
    UknError = 0,
	UknType,
	NotInit,
	ClassNotFound,
	OutOfRange,
	Undefined,
	Object,
	Action,
	Struct,
	NativeObject,
	Resource,
	UserData,
	Bool,
	C8,
	C16,
	S8,
	U8,
	S16,
	U16,
	S32,
	U32,
	S64,
	U64,
	F32,
	F64,
	String,
	MBString,
	Enum,
	Uint2,
	Uint3,
	Uint4,
	Int2,
	Int3,
	Int4,
	Float2,
	Float3,
	Float4,
	Float3x3,
	Float3x4,
	Float4x3,
	Float4x4,
	Half2,
	Half4,
	Mat3,
	Mat4,
	Vec2,
	Vec3,
	Vec4,
	VecU4,
	Quaternion,
	Guid,
	Color,
	DateTime,
	AABB,
	Capsule,
	TaperedCapsule,
	Cone,
	Line,
	LineSegment,
	OBB,
	Plane,
	PlaneXZ,
	Point,
	Range,
	RangeI,
	Ray,
	RayY,
	Segment,
	Size,
	Sphere,
	Triangle,
	Cylinder,
	Ellipsoid,
	Area,
	Torus,
	Rect,
	Rect3D,
	Frustum,
	KeyFrame,
	Uri,
	GameObjectRef,
	RuntimeType,
	Sfix,
	Sfix2,
	Sfix3,
	Sfix4,
	Position,
	F16,
	End,
	Data
}

pub fn get_field_type(class_hash: &u32, field_index: &usize) -> TypeIDs {
    let class_key = format!("{:x}", class_hash);
    let json = JSON.lock().unwrap();
    let type_name = json.get(class_key).unwrap()
        .get("fields").unwrap()
        .get(field_index).unwrap()
        .get("type").unwrap().as_str().unwrap();
    match type_name.to_lowercase().as_str() {
        "undefined" => TypeIDs::Undefined,
        "object" => TypeIDs::Object,
        "action" => TypeIDs::Action,
        "struct" => TypeIDs::Struct,
        "nativeobject" => TypeIDs::NativeObject,
        "resource" => TypeIDs::Resource,
        "userdata" => TypeIDs::UserData,
        "bool" => TypeIDs::Bool,
        "c8" => TypeIDs::C8,
        "c16" => TypeIDs::C16,
        "s8" => TypeIDs::S8,
        "u8" => TypeIDs::U8,
        "s16" => TypeIDs::S16,
        "u16" => TypeIDs::U16,
        "s32" => TypeIDs::S32,
        "u32" => TypeIDs::U32,
        "s64" => TypeIDs::S64,
        "u64" => TypeIDs::U64,
        "f32" => TypeIDs::F32,
        "f64" => TypeIDs::F64,
        "string" => TypeIDs::String,
        "mbstring" => TypeIDs::MBString,
        "enum" => TypeIDs::Enum,
        "uint2" => TypeIDs::Uint2,
        "uint3" => TypeIDs::Uint3,
        "uint4" => TypeIDs::Uint4,
        "int2" => TypeIDs::Int2,
        "int3" => TypeIDs::Int3,
        "int4" => TypeIDs::Int4,
        "float2" => TypeIDs::Float2,
        "float3" => TypeIDs::Float3,
        "float4" => TypeIDs::Float4,
        "float3x3" => TypeIDs::Float3x3,
        "float3x4" => TypeIDs::Float3x4,
        "float4x3" => TypeIDs::Float4x3,
        "float4x4" => TypeIDs::Float4x4,
        "half2" => TypeIDs::Half2,
        "half4" => TypeIDs::Half4,
        "mat3" => TypeIDs::Mat3,
        "mat4" => TypeIDs::Mat4,
        "vec2" => TypeIDs::Vec2,
        "vec3" => TypeIDs::Vec3,
        "vec4" => TypeIDs::Vec4,
        "vecu4" => TypeIDs::VecU4,
        "quaternion" => TypeIDs::Quaternion,
        "guid" => TypeIDs::Guid,
        "color" => TypeIDs::Color,
        "datetime" => TypeIDs::DateTime,
        "aabb" => TypeIDs::AABB,
        "capsule" => TypeIDs::Capsule,
        "taperedcapsule" => TypeIDs::TaperedCapsule,
        "cone" => TypeIDs::Cone,
        "line" => TypeIDs::Line,
        "linesegment" => TypeIDs::LineSegment,
        "obb" => TypeIDs::OBB,
        "plane" => TypeIDs::Plane,
        "planexz" => TypeIDs::PlaneXZ,
        "range" => TypeIDs::Range,
        "rangei" => TypeIDs::RangeI,
        "ray" => TypeIDs::Ray,
        "rayy" => TypeIDs::RayY,
        "segment" => TypeIDs::Segment,
        "size" => TypeIDs::Size,
        "sphere" => TypeIDs::Sphere,
        "triangle" => TypeIDs::Triangle,
        "cylinder" => TypeIDs::Cylinder,
        "ellipsoid" => TypeIDs::Ellipsoid,
        "area" => TypeIDs::Area,
        "torus" => TypeIDs::Torus,
        "rect" => TypeIDs::Rect,
        "rect3d" => TypeIDs::Rect3D,
        "frustum" => TypeIDs::Frustum,
        "keyframe" => TypeIDs::KeyFrame,
        "uri" => TypeIDs::Uri,
        "gameobjectref" => TypeIDs::GameObjectRef,
        "runtimetype" => TypeIDs::RuntimeType,
        "sfix" => TypeIDs::Sfix,
        "sfix2" => TypeIDs::Sfix2,
        "sfix3" => TypeIDs::Sfix3,
        "sfix4" => TypeIDs::Sfix4,
        "position" => TypeIDs::Position,
        "f16" => TypeIDs::F16,
        "end" => TypeIDs::End,
        "data" => TypeIDs::Data,
        _ => TypeIDs::UknType,
    }
}

pub fn get_field_count(class_hash: &u32) -> usize
{
    let class_key = format!("{:x}", class_hash);
    JSON.lock().unwrap().get(class_key).unwrap().get("fields")
        .unwrap().as_array().unwrap().len()
}

pub fn get_field_name(class_hash: &u32, field_index: &usize) -> String {
    let class_key = format!("{:x}", class_hash);
    JSON.lock().unwrap().get(class_key).unwrap().get("fields").unwrap()
        .get(field_index).unwrap().get("name")
        .unwrap().as_str().unwrap().to_string()
}

pub fn get_field_size(class_hash: &u32, field_index: &usize) -> usize
{
    let class_key = format!("{:x}", class_hash);
    JSON.lock().unwrap().get(class_key).unwrap().get("fields").unwrap()
        .get(field_index).unwrap().get("size")
        .unwrap().as_u64().unwrap() as usize
}

pub fn get_field_alignment(class_hash: &u32, field_index: &usize) -> usize
{
    let class_key = format!("{:x}", class_hash);
    JSON.lock().unwrap().get(class_key).unwrap().get("fields").unwrap()
        .get(field_index).unwrap().get("align")
        .unwrap().as_u64().unwrap() as usize
}

pub fn get_field_array_state(class_hash: &u32, field_index: &usize) -> RSZResult<bool> {
    let class_key = format!("{:x}", class_hash);
    match JSON.lock().unwrap().get(class_key) {
        Some(class) => {
            match class.get("fields") {
                Some(fields) => {
                    match fields.get(field_index)
                    {
                        Some(field) => {
                            match field.get("array") {
                                Some(array) => {
                                    match array.as_bool() {
                                        Some(array) => Ok(array),
                                        None => Ok(false)
                                    }
                                }
                                None => Ok(false)
                            }
                        }
                        None => Ok(false)
                    }
                }
                None => Ok(false)
            }
        }
        None => Ok(false)
    }
}

pub fn get_rsz_class_name(class_hash: &u32) -> RSZResult<String>
{
    let class_key = format!("{:x}", class_hash);
    match JSON.lock().unwrap().get(class_key) {
        Some(class) => {
            match class.get("name") {
                Some(name) => {
                    match name.as_str() {
                        Some(name) => Ok(name.to_string()),
                        None => Err(RSZError)
                    }
                }
                None => Err(RSZError)
            }
        }
        None => Err(RSZError)
    }
}

pub fn parse_json(path: String) -> std::io::Result<()> {
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut json = JSON.lock().unwrap();
    *json = serde_json::from_str(&contents)?;
    
    Ok(())
}