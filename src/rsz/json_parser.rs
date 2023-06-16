use std::{fmt, fs::File, io::Read};
use serde_json::Value;

type RSZResult<T> = Result<T, RSZError>;

#[derive(Debug, Clone)]
pub struct RSZError;

impl fmt::Display for RSZError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RSZ json error")
    }
}

pub enum TypeIDs {
    UknError = 0,
	UknType,
	NotInit,
	ClassNotFound,
	OutOfRange,
	UndefinedTid,
	ObjectTid,
	ActionTid,
	StructTid,
	NativeObjectTid,
	ResourceTid,
	UserDataTid,
	BoolTid,
	C8Tid,
	C16Tid,
	S8Tid,
	U8Tid,
	S16Tid,
	U16Tid,
	S32Tid,
	U32Tid,
	S64Tid,
	U64Tid,
	F32Tid,
	F64Tid,
	StringTid,
	MBStringTid,
	EnumTid,
	Uint2Tid,
	Uint3Tid,
	Uint4Tid,
	Int2Tid,
	Int3Tid,
	Int4Tid,
	Float2Tid,
	Float3Tid,
	Float4Tid,
	Float3x3Tid,
	Float3x4Tid,
	Float4x3Tid,
	Float4x4Tid,
	Half2Tid,
	Half4Tid,
	Mat3Tid,
	Mat4Tid,
	Vec2Tid,
	Vec3Tid,
	Vec4Tid,
	VecU4Tid,
	QuaternionTid,
	GuidTid,
	ColorTid,
	DateTimeTid,
	AABBTid,
	CapsuleTid,
	TaperedCapsuleTid,
	ConeTid,
	LineTid,
	LineSegmentTid,
	OBBTid,
	PlaneTid,
	PlaneXZTid,
	PointTid,
	RangeTid,
	RangeITid,
	RayTid,
	RayYTid,
	SegmentTid,
	SizeTid,
	SphereTid,
	TriangleTid,
	CylinderTid,
	EllipsoidTid,
	AreaTid,
	TorusTid,
	RectTid,
	Rect3DTid,
	FrustumTid,
	KeyFrameTid,
	UriTid,
	GameObjectRefTid,
	RuntimeTypeTid,
	SfixTid,
	Sfix2Tid,
	Sfix3Tid,
	Sfix4Tid,
	PositionTid,
	F16Tid,
	EndTid,
	DataTid
}

pub fn get_field_type(json: &Value, class_hash: &u32, field_index: &usize) -> TypeIDs {
    let class_key = format!("{:x}", class_hash);
    let type_name = json.get(class_key).unwrap()
        .get("fields").unwrap()
        .get(field_index).unwrap()
        .get("type").unwrap().as_str().unwrap();
    match type_name.to_lowercase().as_str() {
        "undefined" => TypeIDs::UndefinedTid,
        "object" => TypeIDs::ObjectTid,
        "action" => TypeIDs::ActionTid,
        "struct" => TypeIDs::StructTid,
        "nativeobject" => TypeIDs::NativeObjectTid,
        "resource" => TypeIDs::ResourceTid,
        "userdata" => TypeIDs::UserDataTid,
        "bool" => TypeIDs::BoolTid,
        "c8" => TypeIDs::C8Tid,
        "c16" => TypeIDs::C16Tid,
        "s8" => TypeIDs::S8Tid,
        "u8" => TypeIDs::U8Tid,
        "s16" => TypeIDs::S16Tid,
        "u16" => TypeIDs::U16Tid,
        "s32" => TypeIDs::S32Tid,
        "u32" => TypeIDs::U32Tid,
        "s64" => TypeIDs::S64Tid,
        "u64" => TypeIDs::U64Tid,
        "f32" => TypeIDs::F32Tid,
        "f64" => TypeIDs::F64Tid,
        "string" => TypeIDs::StringTid,
        "mbstring" => TypeIDs::MBStringTid,
        "enum" => TypeIDs::EnumTid,
        "uint2" => TypeIDs::Uint2Tid,
        "uint3" => TypeIDs::Uint3Tid,
        "uint4" => TypeIDs::Uint4Tid,
        "int2" => TypeIDs::Int2Tid,
        "int3" => TypeIDs::Int3Tid,
        "int4" => TypeIDs::Int4Tid,
        "float2" => TypeIDs::Float2Tid,
        "float3" => TypeIDs::Float3Tid,
        "float4" => TypeIDs::Float4Tid,
        "float3x3" => TypeIDs::Float3x3Tid,
        "float3x4" => TypeIDs::Float3x4Tid,
        "float4x3" => TypeIDs::Float4x3Tid,
        "float4x4" => TypeIDs::Float4x4Tid,
        "half2" => TypeIDs::Half2Tid,
        "half4" => TypeIDs::Half4Tid,
        "mat3" => TypeIDs::Mat3Tid,
        "mat4" => TypeIDs::Mat4Tid,
        "vec2" => TypeIDs::Vec2Tid,
        "vec3" => TypeIDs::Vec3Tid,
        "vec4" => TypeIDs::Vec4Tid,
        "vecu4" => TypeIDs::VecU4Tid,
        "quaternion" => TypeIDs::QuaternionTid,
        "guid" => TypeIDs::GuidTid,
        "color" => TypeIDs::ColorTid,
        "datetime" => TypeIDs::DateTimeTid,
        "aabb" => TypeIDs::AABBTid,
        "capsule" => TypeIDs::CapsuleTid,
        "taperedcapsule" => TypeIDs::TaperedCapsuleTid,
        "cone" => TypeIDs::ConeTid,
        "line" => TypeIDs::LineTid,
        "linesegment" => TypeIDs::LineSegmentTid,
        "obb" => TypeIDs::OBBTid,
        "plane" => TypeIDs::PlaneTid,
        "planexz" => TypeIDs::PlaneXZTid,
        "range" => TypeIDs::RangeTid,
        "rangei" => TypeIDs::RangeITid,
        "ray" => TypeIDs::RayTid,
        "rayy" => TypeIDs::RayYTid,
        "segment" => TypeIDs::SegmentTid,
        "size" => TypeIDs::SizeTid,
        "sphere" => TypeIDs::SphereTid,
        "triangle" => TypeIDs::TriangleTid,
        "cylinder" => TypeIDs::CylinderTid,
        "ellipsoid" => TypeIDs::EllipsoidTid,
        "area" => TypeIDs::AreaTid,
        "torus" => TypeIDs::TorusTid,
        "rect" => TypeIDs::RectTid,
        "rect3d" => TypeIDs::Rect3DTid,
        "frustum" => TypeIDs::FrustumTid,
        "keyframe" => TypeIDs::KeyFrameTid,
        "uri" => TypeIDs::UriTid,
        "gameobjectref" => TypeIDs::GameObjectRefTid,
        "runtimetype" => TypeIDs::RuntimeTypeTid,
        "sfix" => TypeIDs::SfixTid,
        "sfix2" => TypeIDs::Sfix2Tid,
        "sfix3" => TypeIDs::Sfix3Tid,
        "sfix4" => TypeIDs::Sfix4Tid,
        "position" => TypeIDs::PositionTid,
        "f16" => TypeIDs::F16Tid,
        "end" => TypeIDs::EndTid,
        "data" => TypeIDs::DataTid,
        _ => TypeIDs::UknType,
    }
}

pub fn get_field_count(json: &Value, class_hash: &u32) -> usize
{
    let class_key = format!("{:x}", class_hash);
    json.get(class_key).unwrap().get("fields")
        .unwrap().as_array().unwrap().len()
}

pub fn get_field_size(json: &Value, class_hash: &u32, field_index: &usize) -> usize
{
    let class_key = format!("{:x}", class_hash);
    json.get(class_key).unwrap().get("fields").unwrap()
        .get(field_index).unwrap().get("size")
        .unwrap().as_u64().unwrap() as usize
}

pub fn get_field_array_state(json: &Value, class_hash: &u32, field_index: &usize) -> RSZResult<bool> {
    let class_key = format!("{:x}", class_hash);
    match json.get(class_key) {
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

pub fn get_rsz_class_name(json: &Value, class_hash: &u32) -> RSZResult<String>
{
    let class_key = format!("{:x}", class_hash);
    match json.get(class_key) {
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

pub fn parse_json() -> std::io::Result<Value> {
    let mut file = File::open("rszsf6.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(serde_json::from_str(&contents)?)
}