#![crate_type = "bin"]

#![crate_name = "gp_project"]



//#![feature(box_syntax)]
//#![feature(os)]
//#![feature(collections)]
//#![feature(into_cow)]
//#![feature(convert)]
//#![feature(rustc_private)]
//#![deny(warnings)]


mod servermessage;
mod server;
mod enviroment;



fn main()
{

	println!("init");
	//Basic commandline arguments, expand later
	

	println!("server launching");
	server::init();

	



}





