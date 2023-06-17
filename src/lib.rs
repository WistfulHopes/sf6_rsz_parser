use nom::IResult;
use crate::fchar::CharacterAsset;

pub mod rsz;
pub mod fchar;

pub fn parse_fchar(input: &[u8]) -> IResult<&[u8], CharacterAsset> {
    fchar::parse_fchar(input)
}