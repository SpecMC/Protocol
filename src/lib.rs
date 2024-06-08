pub mod base;
pub mod enums;
pub mod packets;
pub mod types;

use specmc_base::parse::{Parse, ParseError};

use enums::Enum;
use packets::Packet;
use types::CustomType;

#[derive(Debug)]
pub struct Protocol {
    pub enums: Vec<Enum>,
    pub types: Vec<CustomType>,
    pub packets: Vec<Packet>,
}
impl Parse for Protocol {
    fn parse(tokens: &mut Vec<String>) -> Result<Self, specmc_base::parse::ParseError> {
        let mut enums: Vec<Enum> = vec![];
        let mut types: Vec<CustomType> = vec![];
        let mut packets: Vec<Packet> = vec![];
        while !tokens.is_empty() {
            match tokens.last().unwrap().as_str() {
                "enum" => {
                    enums.push(Enum::parse(tokens)?);
                }
                "type" => {
                    types.push(CustomType::parse(tokens)?);
                }
                "packet" => {
                    packets.push(Packet::parse(tokens)?);
                }
                token => {
                    return Err(ParseError::InvalidToken {
                        token: token.to_string(),
                        error: "Expected \"enum\", \"type\" or \"packet\"".to_string(),
                    });
                }
            }
        }

        Ok(Protocol {
            enums,
            types,
            packets,
        })
    }
}
