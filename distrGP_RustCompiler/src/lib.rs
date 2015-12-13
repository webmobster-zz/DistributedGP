#![deny(warnings)]
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::io::Write;
use std::io;
use std::process::Command;
use distrgp_generator::UUID;
use shared_library::dynamic_library::DynamicLibrary;
use std::mem;

extern crate distrgp_generator;
extern crate shared_library;

#[derive(Debug)]
enum CompileError
{
	LaunchError(std::io::Error),
	ExecutionError(Vec<u8>),
	OpenError(String),
	SymbolError(String)
}


pub struct RustCompiler
{
	compiled_path: String,
	generated_path: String,
	rustc_path: String
}

impl RustCompiler
{
	pub fn new(compiled_path: String, generated_path: String, rustc_path: String) -> RustCompiler
	{
		{
			let path = Path::new(&*compiled_path);
			DynamicLibrary::prepend_search_path(&path);
		}
		RustCompiler
		{
			compiled_path: compiled_path,
			generated_path: generated_path,
			rustc_path: rustc_path
		}
	}
}


impl distrgp_generator::Compiler for RustCompiler
{
	fn compile(&self, code : Vec<String>,uuid: UUID) -> fn(&mut distrgp_generator::GlobalState, &mut distrgp_generator::LocalState) -> bool
	{
			write_code(code,uuid,self.generated_path.clone()).unwrap();
			compile_code(uuid,self.rustc_path.clone(),self.compiled_path.clone()).unwrap();
			//FIXME
			load_function(uuid).unwrap()
	}

	//TODO
	fn drop(&self, _: UUID)
	{

	}


}


pub fn write_code(op_list: Vec<String>,uuid: UUID, path: String) -> Result<(),io::Error>
{
	let path_root = Path::new(&*path);
	let mut path_buf = PathBuf::new();
	path_buf.push(path_root);
	path_buf.set_file_name(uuid.to_string());
	path_buf.set_extension("rs");
	let mut file = try!(File::create(path_buf));

	//Write header
	try!(write!(file, 	"extern crate distrGP_Generator;

				use self::distrGP_Generator::GlobalState;
				use self::distrGP_Generator::LocalState;
				use self::distrGP_Generator::StateIO;

				use std::sync::mpsc::TryRecvError;
				use std::sync::{{Arc, Mutex}};"));

	try!(write!(file, "\n #[no_mangle] \n fn {:?}(global: &mut GlobalState, local: &mut LocalState) -> bool \n {{ \n", uuid));

	for op in op_list.iter(){
		try!(file.write(op.as_bytes()));
	}
	try!(write!(file,"\n}}\n"));

	return Ok(());

}


fn load_function(uuid: UUID) -> Result<fn(&mut distrgp_generator::GlobalState, &mut distrgp_generator::LocalState) -> bool,CompileError>
{
	let path = format!("{}.dll",uuid.to_string());
	let path = Path::new(&*path);
	let dynamic_lib = DynamicLibrary::open(Some(path));
	if dynamic_lib.is_ok()
	{
		let dynamic_lib=dynamic_lib.unwrap();
		unsafe
		{
			let function = dynamic_lib.symbol(&*uuid.to_string());
			if function.is_ok()
			{
				let function = function.unwrap();
				let function = mem::transmute::<*mut u8,fn(&mut distrgp_generator::GlobalState, &mut distrgp_generator::LocalState) -> bool >(function);
				return Ok(function as fn(&mut distrgp_generator::GlobalState, &mut distrgp_generator::LocalState) -> bool);
			}
			return Err(CompileError::SymbolError(function.err().unwrap()))

		}
	}
	Err(CompileError::OpenError(dynamic_lib.err().unwrap()))

}

fn compile_code(uuid: UUID, rustc_path: String, compiled_path: String) -> Result<(),CompileError>
{

		let output = Command::new(rustc_path)
                     .arg("-c")
                     .arg("echo hello")
                     .output();
		if output.is_err()
		{
			return Err(CompileError::LaunchError(output.err().unwrap()));
		}
		let output = output.unwrap();
		if output.status.success()
		{
			return Ok(())
		}
		Err(CompileError::ExecutionError(output.stdout))


}
