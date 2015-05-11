#![crate_type = "bin"]

#![crate_name = "distrGP"]



#![deny(missing_docs)]
#![deny(warnings)]


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





