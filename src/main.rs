#![crate_type = "bin"]

#![crate_name = "distrGP"]



//#![deny(missing_docs)]
//#![deny(warnings)]


extern crate distrGP_Evaluator;
extern crate distrGP_ProvidedOperators;
extern crate distrGP_Generator;

use distrGP_Evaluator::server;
use distrGP_Generator::GeneticOperator;
use distrGP_Generator::Generator;
use distrGP_ProvidedOperators::geneticoperators::TreeCross;
use distrGP_ProvidedOperators::geneticoperators::StandardGrow;

mod reader;



fn main()
{

	println!("init");
	//Basic commandline arguments, expand later


	//read problem description file

	let problem_description = reader::readfile();

	//server: Generate the initial population
	//this needs fixing badly	
	let generator = Generator::init(
				
				problem_description.get_popcount(),
				problem_description.get_operators(),

				problem_description.get_selector(),
				vec!(Box::new(TreeCross::new(1.0)) as Box<GeneticOperator>),
				Box::new(StandardGrow::new(1.0,300)),
				problem_description.get_life()
				);
	
	println!("created generator");
	println!("server launching");
	server::init(generator,4);

	
	

	



}





