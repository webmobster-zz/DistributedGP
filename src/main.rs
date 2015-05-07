#![crate_type = "bin"]

#![crate_name = "gp_project"]



#![feature(box_syntax)]
#![feature(os)]
#![feature(collections)]
#![feature(into_cow)]
#![feature(convert)]
#![feature(rustc_private)]
//#![deny(warnings)]

extern crate graphviz;

mod servermessage;
mod server;
mod client;



fn main()
{

	println!("init");
	//Basic commandline arguments, expand later
	

	println!("server launching");
	server::init();

	



}





