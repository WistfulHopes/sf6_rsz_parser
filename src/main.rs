use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use serde::{Deserialize, Serialize};
use crate::rsz::json_parser::parse_json;
use include_bytes_zstd::include_bytes_zstd;

mod fchar;
mod rsz;

#[derive(Serialize, Deserialize)]
struct Prefab {
    magic: u32,
    info_count: i32,
    resource_count: i32,
    gameobject_ref_info_count: i32,
    userdata_count: i32,
    #[serde(skip)]
    reserved: i32,
    gameobject_ref_info_tbl: u64,
    resource_info_tbl: u64,
    userdata_info_tbl: u64,
    data_offset: u64,
}

#[derive(Serialize, Deserialize)]
struct UserData {
    magic: u32,
    resource_count: i32,
    userdata_count: i32,
    info_count: i32,
    resource_info_tbl: u64,
    userdata_info_tbl: u64,
    data_offset: u64,
}

#[derive(Serialize, Deserialize)]
enum HeaderType {
    Prefab(Prefab),
    UserData(UserData),
}

#[derive(Serialize, Deserialize)]
struct PrefabGameObjectInfo {
    id: i32,
    parent_id: i32,
    component_count: i32,
}

#[derive(Serialize, Deserialize)]
struct StandardGameObjectInfo {
    guid: rsz::GUID,
    id: i32,
    parent_id: i32,
    component_count: u16,
    unknown: i16,
    prefab_id: i32,
}

#[derive(Serialize, Deserialize)]
struct UserDataInfo {
    crc: u32,
}

#[derive(Serialize, Deserialize)]
struct RSZFile
{
    header: HeaderType,
    prefab_gameobject_info: Vec<PrefabGameObjectInfo>,
    standard_gameobject_info: Vec<StandardGameObjectInfo>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("\nArguments not provided! The argument should be the file to parse.")
    }

    let json_bytes = include_bytes_zstd!("rszsf6.json", 19);
    
    parse_json(json_bytes)?;
    
    let mut reader = BufReader::with_capacity(0x3fffff,File::open(&args[1]).unwrap());
    let mut buffer: Vec<u8> = vec![];
    reader.read_to_end(&mut buffer).unwrap();
    
    let fchar_file = fchar::parse_fchar(&buffer).unwrap().1;
    let serialized_fchar = serde_json::to_string_pretty(&fchar_file).unwrap();
    println!("Writing fchar to json...");

    std::fs::write(&args[1].push_str(".json"), serialized_fchar)?;
    println!("Complete!");

    Ok(())
}