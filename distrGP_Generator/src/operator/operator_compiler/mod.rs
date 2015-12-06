use super::UUID;
use super::super::GlobalState;
use super::super::LocalState;
use super::SpecialOperator;
use super::MinifiedOperator;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use self::core::str::FromStr;

extern crate serde_json;
extern crate core;


#[derive(Serialize, Deserialize, Debug)]
struct ParsedOperator
{
	pub path: String,
	pub description: String,
	pub sucessors: u8,
	pub cost: u64,
	pub uuid: String,
	pub special: String

}

pub struct CompiledOperator
{
	pub function: fn(&mut GlobalState, &mut LocalState) -> bool,
	pub code: Option<String>,
	pub description: Option<String>,
	pub parts: Option<Vec<UUID>>,
	pub sucessors: u8,
	pub cost: u64,
	pub uuid: UUID,
	pub special: SpecialOperator

}


pub trait Compiler
{
	fn compile(&self, code : String) -> fn(&mut GlobalState, &mut LocalState) -> bool;
}

pub fn load_base_operators(path: String, compiler: &Compiler)-> (Vec<CompiledOperator>,Vec<(UUID,MinifiedOperator)>)
{
	let path = Path::new(&*path);
	let mut file = File::open(path).unwrap();
	let mut s = String::new();
	assert!(file.read_to_string(&mut s).is_ok());

	let mut compiled_operators = Vec::new();
	let mut minified_operators = Vec::new();


	let json: Vec<ParsedOperator> = serde_json::from_str(&s).unwrap();

	for operator in json
	{
		let path = Path::new(&*operator.path);
		let mut file = File::open(path).unwrap();
		let mut code = String::new();
		assert!(file.read_to_string(&mut code).is_ok());
		let compiled = CompiledOperator{
			function: compiler.compile(	code.clone()),
			code: Some(code),
			description: Some(operator.description),
			parts: None,
			sucessors: operator.sucessors,
			cost: operator.cost,
			uuid: parse_uuid(operator.uuid),
			special: parse_special(operator.special)
		};
		let minified = MinifiedOperator{
			op :compiled.function,
			sucessors: compiled.sucessors,
			cost: compiled.cost,
			special: compiled.special

		};
		minified_operators.push((compiled.uuid.clone(),minified));
		compiled_operators.push(compiled);
	}
	(compiled_operators,minified_operators)
}

fn parse_uuid(uuid: String) -> UUID
{
	UUID::from_str(&*uuid).unwrap()
}

fn parse_special(special: String) -> SpecialOperator
{
	match &*special
	{
		"None" => SpecialOperator::None,
		"NewThread" => SpecialOperator::NewThread,
		_ => panic!("Unexpected token in special operator")
	}
}
