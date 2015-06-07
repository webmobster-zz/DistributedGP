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
use distrGP_Generator::StateIO;
use distrGP_ProvidedOperators::geneticoperators::TreeCross;
use distrGP_ProvidedOperators::geneticoperators::StandardGrow;
use distrGP_Evaluator::FitnessMessage;
use std::sync::mpsc::channel;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};

mod reader;



fn main()
{

	println!("init");
	//Basic commandline arguments, expand later


	//read problem description file




	
	//names
	let (tx, rx) = channel();
	let (txt, rxt) = channel();
	thread::spawn(move || {
		let problem_description = reader::readfile();

		let generator = Generator::init(
				
				problem_description.get_popcount(),
				problem_description.get_operators(),

				problem_description.get_selector(),
				vec!(Box::new(TreeCross::new(1.0)) as Box<GeneticOperator>),
				Box::new(StandardGrow::new(1.0,300)),
				10000
				);
		server::init(generator,4,tx,rxt);
	});
	fitness(txt,rx);

}

fn fitness(send: Sender<FitnessMessage>, recv: Receiver<FitnessMessage>)
{
	loop{
		let thing =match recv.recv()
		{
			Ok(y) => match y
				{
					FitnessMessage::PopVec(x) => x,
					_=> panic!("Invalid Message")
				},
			_=> panic!("Dropped sender")

		};
		send.send(FitnessMessage::Ready);

		for mut i in thing
		{
			i.send_byte(StateIO::Fitness(8));
		}
		recv.recv();
	}

}



