use nom::IResult;
use crate::fchar::CharacterAsset;
use crate::prefab::Prefab;

pub mod rsz;
pub mod fchar;
pub mod prefab;

pub fn parse_fchar(input: &[u8]) -> IResult<&[u8], CharacterAsset> {
    fchar::parse_fchar(input)
}

pub fn parse_prefab(input: &[u8]) -> IResult<&[u8], Prefab> {
    prefab::parse_prefab(input)
}