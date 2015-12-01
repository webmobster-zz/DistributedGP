use super::UncompiledOperator;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;
use std::io;

pub fn compile_operators(op_list: Vec<UncompiledOperator>, dynlib_path: String, generation_num: i32) -> Result<(),io::Error>
{
	let path_root = Path::new(&*dynlib_path);
	let mut path_buf = PathBuf::new();
	path_buf.push(path_root);
	path_buf.set_file_name(generation_num.to_string());
	path_buf.set_extension("rs");
	let mut file = try!(File::create(path_buf));

	//Write header
	try!(write!(file, 	"extern crate distrGP_Generator;

				use self::distrGP_Generator::GlobalState;
				use self::distrGP_Generator::LocalState;
				use self::distrGP_Generator::StateIO;

				use std::sync::mpsc::TryRecvError;
				use std::sync::{{Arc, Mutex}};"));

	for op in op_list.iter(){
		try!(write!(file, "\n #[no_mangle] \n fn {:?}(global: &mut GlobalState, local: &mut LocalState) -> bool \n {{ \n", op.uuid));
		try!(file.write(op.code.as_bytes()));
		try!(write!(file,"\n}}\n"));
		


	}
	return Ok(());

}

/*
fn load_operators(map: &mut OperatorMap)
{


}


fn combine_operators()
{


}
fn parse_operators()
{


}

fn open_operators_dylib()
{


}
*/
