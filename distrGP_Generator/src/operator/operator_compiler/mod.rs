pub use super::UUID;
use std::str::FromStr;

//TEMP
pub use self::xml_parser::read_xml;
pub use self::compiler::compile_operators;

mod compiler;
mod xml_parser;


#[derive(Debug)]
pub struct UncompiledOperator
{
	pub code: String,
	pub description: String,
	pub parts: Option<Vec<UUID>>,
	pub sucessors: u8,
	pub cost: u64,
	pub uuid: UUID,
	pub special: Option<String>

}

pub struct TempVecUUID {x: Vec<UUID>}
impl FromStr for TempVecUUID {

	type Err = ();

            #[inline]
            #[allow(deprecated)]
            fn from_str(src: &str) -> Result<Self, ()> {
		let split = src.split(";");
		let mut vec = Vec::new();
		for e in split
		{
			vec.push(e.parse::<UUID>().unwrap());

		}
		Ok(TempVecUUID{x: vec})
		
            }

}
