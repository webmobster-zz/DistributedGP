#![crate_type = "bin"]

#![crate_name = "distrGP"]



//#![deny(missing_docs)]
//#![deny(warnings)]


extern crate distrGP_Evaluator;

use distrGP_Evaluator::server;

mod reader;



fn main()
{

	println!("init");
	//Basic commandline arguments, expand later


	//read problem description file

	let problem_description = reader::readfile();

	server::init(
				problem_description.get_popcount(),
				problem_description.get_operators(),
				problem_description.get_end_operators(),

				problem_description.get_operator_trait(),


				problem_description.get_repetitions(),

				problem_description.get_selector(),
				Vec::new(),
				problem_description.get_life(),
				4
	);

	println!("server launching");
	

	



}





