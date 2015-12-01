use xml::reader::EventReader;
use xml::reader::XmlEvent;
use xml::reader::ParserConfig;
use std::io::Read;
use super::UUID;
use super::UncompiledOperator;
use super::TempVecUUID;





//TODO: Remove all unwraps
pub fn read_xml<T: Read>(file: T) -> Vec<UncompiledOperator>
{
	let config = ParserConfig::new()
   	 .trim_whitespace(true)
   	 .ignore_comments(true);
	let mut parser = EventReader::new_with_config(file,config);
	let mut op_vec = Vec::new();
	match parser.next().unwrap()
	{
			XmlEvent::StartDocument{..} =>
			{
			}
			element => panic!("invalid xml expected start document got: {:?}", element)
	}

	match parser.next().unwrap()
	{
			XmlEvent::StartElement { ref name, .. } if name.local_name == "operatorlist" =>
			{
			}
			element => return panic!("invalid xml expected operator list start element got: {:?}", element)
	}

	while match parser.next().unwrap()
		{
				XmlEvent::StartElement { ref name, .. } if name.local_name == "operator" =>
				{
					let op = parse_operator(&mut parser);
					if op.is_ok()
					{
						op_vec.push(op.unwrap());
					}
					else
					{
						panic!("xml parsing error: {}, got to {:?}",op.err().unwrap(), op_vec);
					}
					true
				}
				XmlEvent::EndElement { ref name, .. } if name.local_name == "operatorlist" =>
				{
					false
				}
				element => return panic!("invalid xml, expected start or end tag got: {:?}", element)
		}
	{}
	op_vec
	/*
	for e in parser.events()
	{
		match e
		{
			XmlEvent::StartElement { name, .. } =>
			{
				println!("{}+{}", indent(depth), name);
				depth += 1;
			}
			XmlEvent::EndElement { name } =>
			{
				depth -= 1;
				println!("{}-{}", indent(depth), name);

			}
			XmlEvent::Characters(chars) =>
			{
				println!("{}={}", indent(depth), chars);
			}
			XmlEvent::CData(chars) =>
			{
				println!("{}#{}", indent(depth), chars);
			}

			XmlEvent::Error(e) =>
			{
				println!("Error: {}", e);
				break;
			}
		   	_ => {}
		}
	}
	parser
	*/

}

fn parse_operator<T: Read>(mut parser: &mut EventReader<T>) -> Result<UncompiledOperator,String>
{
	let mut code = None; let mut description = None; let mut parts = None;let mut uuid: Option<String> = None;
	let mut sucessors: Option<String> = None; let mut cost: Option<String> = None; let mut special= None;
	while match parser.next().unwrap()
		{
				XmlEvent::StartElement { name, .. } =>
				{
					if name.local_name == "sucessors"
					{

						let result =  parse_chars(&mut parser);
						if result.is_ok()
						{
							sucessors =Some(result.unwrap());
						}
						else
						{
							return Err(result.err().unwrap());
						}
						
					}
					else if name.local_name == "code"
					{
						let result =  parse_cdata(&mut parser);
						if result.is_ok()
						{
							code =Some(result.unwrap());
						}
						else
						{
							return Err(result.err().unwrap());
						}
					}
					else if name.local_name == "description"
					{
						let result =  parse_chars(&mut parser);
						if result.is_ok()
						{
							description =Some(result.unwrap());
						}
						else
						{
							return Err(result.err().unwrap());
						}
					}
					else if name.local_name == "parts"
					{
						let result =  parse_chars(&mut parser);
						if result.is_ok()
						{
							parts =Some(result.unwrap());
						}
						else
						{
							return Err(result.err().unwrap());
						}
					}
					else if name.local_name == "cost"
					{
						let result =  parse_chars(&mut parser);
						if result.is_ok()
						{
							cost =Some(result.unwrap());
						}
						else
						{
							return Err(result.err().unwrap());
						}
					}
					else if name.local_name == "special"
					{
						let result =  parse_chars(&mut parser);
						if result.is_ok()
						{
							special =Some(result.unwrap());
						}
						else
						{
							return Err(result.err().unwrap());
						}
					}
					else if name.local_name == "UUID"
					{
						let result =  parse_chars(&mut parser);
						if result.is_ok()
						{
							uuid =Some(result.unwrap());
						}
						else
						{
							return Err(result.err().unwrap());
						}
					}
					else
					{
						return Err(format!("unrecongized xml tag: {:?}", name.local_name))

					}
					
					true
				}
				XmlEvent::EndElement { ref name, .. } if name.local_name == "operator" =>
				{
					false
				}
				element => return Err(format!("invalid xml, expected start or end tag got: {:?}", element))
		}
	{}
	if code.is_some() && description.is_some() && sucessors.is_some() && cost.is_some() && uuid.is_some()
	{
		let cost =  cost.unwrap().parse::<u64>().unwrap();
		let sucessors = sucessors.unwrap().parse::<u8>().unwrap();
		let uuid = uuid.unwrap().parse::<UUID>().unwrap();
		let parsedparts= if parts.is_some()
		{
			Some(parts.unwrap().parse::<TempVecUUID>().unwrap().x)
		}
		else
		{
			None

		};
		let op =UncompiledOperator{ code: code.unwrap(), description: description.unwrap(), parts: parsedparts,
					 sucessors: sucessors, cost: cost, special: special, uuid: uuid};
		Ok(op)

	}
	else
	{
		Err("Missing Required Fields".to_string())

	}

}


fn parse_chars<T: Read>(parser: &mut EventReader<T>) -> Result<String,String>
{
	
 		match parser.next().unwrap()
		{
				XmlEvent::Characters(chars) => 
				{

					match parser.next().unwrap()
					{

						XmlEvent::EndElement{..}=> (),
						element => return Err(format!("invalid xml, expected end tag got: {:?}", element))


					}
					return Ok(chars)
				},
				element => return Err(format!("invalid xml, expected characters got:{:?}", element))
				
		}

}

fn parse_cdata<T: Read>(parser: &mut EventReader<T>) -> Result<String,String>
{
	match parser.next().unwrap()
		{
				XmlEvent::CData(chars) => 
				{

					match parser.next().unwrap()
					{

						XmlEvent::EndElement{..}=> (),
						element => return Err(format!("invalid xml, expected end tag got: {:?}", element))


					}
					return Ok(chars)
				},
				element => return Err(format!("invalid xml, expected CData got:{:?}", element))
				
		}
}



