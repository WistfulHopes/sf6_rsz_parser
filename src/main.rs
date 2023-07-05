use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use serde::{Deserialize, Serialize};
use crate::rsz::json_parser::parse_json;
use include_bytes_zstd::include_bytes_zstd;

mod fchar;
mod rsz;
mod prefab;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("\nArguments not provided! The argument should be the file to parse.")
    }

    //let json_bytes = include_bytes_zstd!("rszsf6.json", 9);
    let json_bytes = match args[1].ends_with(".17"){
        true=>include_bytes_zstd!("rszsf6.json", 9),
        false=>include_bytes_zstd!("rszdmc5.json", 9),
    };
    parse_json(json_bytes)?;
    
    let mut reader = BufReader::with_capacity(0x7fffff,File::open(&args[1]).unwrap());
    let mut buffer: Vec<u8> = vec![];
    reader.read_to_end(&mut buffer).unwrap();
    
    if args[1].ends_with("fchar.17")
    {
        let fchar_file = fchar::parse_fchar(&buffer).unwrap().1;
        let serialized_fchar = serde_json::to_string_pretty(&fchar_file).unwrap();
        println!("Writing fchar to json...");

        let mut json_name = args[1].clone();
        json_name.push_str(".json");

        std::fs::write(json_name, serialized_fchar)?;
        println!("Complete!");
    }
    
    else if args[1].ends_with("pfb.17")
    {
        let pfb_file = prefab::parse_prefab(&buffer,false).unwrap().1;
        let serialized_prefab = serde_json::to_string_pretty(&pfb_file).unwrap();
        
        println!("Writing prefab to json...");

        let mut json_name = args[1].clone();
        json_name.push_str(".json");

        std::fs::write(json_name, serialized_prefab)?;
        println!("Complete!");
    }
    else if args[1].ends_with("pfb.16")
    {
        let pfb_file = prefab::parse_prefab(&buffer,true).unwrap().1;
        let serialized_prefab = serde_json::to_string_pretty(&pfb_file).unwrap();
        
        println!("Writing prefab to json...");

        let mut json_name = args[1].clone();
        json_name.push_str(".json");

        std::fs::write(json_name, serialized_prefab)?;
        println!("Complete!");
    }

    Ok(())
}