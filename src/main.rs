#![crate_type = "bin"]

#![crate_name = "distrGP"]



//#![deny(missing_docs)]
//#![deny(warnings)]


extern crate distr_gp_evaluator;
extern crate distrGP_ProvidedOperators;
extern crate distrGP_Generator;
extern crate env_logger;
#[macro_use]
extern crate log;

use log::{LogRecord, LogLevel, LogMetadata};
use distrGP_Generator::GeneticOperator;
use distrGP_Generator::Generator;
use distrGP_Generator::StateIO;
use std::sync::mpsc::TryRecvError;
use distrGP_ProvidedOperators::geneticoperators::TreeCross;
use distrGP_ProvidedOperators::geneticoperators::FlatCross;
use distrGP_ProvidedOperators::geneticoperators::PointMutate;
use distrGP_ProvidedOperators::geneticoperators::StandardGrow;
use distr_gp_evaluator::FitnessMessage;
use std::sync::mpsc::channel;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};

mod reader;



fn main()
{

	env_logger::init().unwrap();

	info!("init");
	//Basic commandline arguments, expand later


	//read problem description file




	
	//names
	let (tx, rx) = channel();
	let (txt, rxt) = channel();
	thread::spawn(move || {
		let problem_description = reader::readfile();

		let generator = Generator::init(
				
				500,
				problem_description.get_operators(),

				problem_description.get_selector(),
				vec!(	Box::new(TreeCross::new(0.3)) as Box<GeneticOperator>,
					Box::new(FlatCross::new(0.3)) as Box<GeneticOperator>,
					Box::new(PointMutate::new(0.4)) as Box<GeneticOperator>,
					),
				Box::new(StandardGrow::new(1.0,300)),
				10000
				);
		distr_gp_evaluator::init(generator,12,tx,rxt);
	});
	fitness(txt,rx);

}

fn fitness(send: Sender<FitnessMessage>, recv: Receiver<FitnessMessage>)
{
	loop{
		let mut thing =match recv.recv()
		{
			Ok(y) => match y
				{
					FitnessMessage::PopVec(x) => x,
					_=> panic!("Invalid Message")
				},
			_=> panic!("Dropped sender")

		};
		send.send(FitnessMessage::Ready);
		let mut fit_vec: Vec<u64> = std::iter::repeat(10000).take(thing.len()).collect::<Vec<_>>();;
		let mut z = 0;
		loop
		{


			if thing.len() == z
			{
				break
			}
			for mut i in thing.iter_mut()
			{




				match i.try_receive_byte()
				{
					Ok(x) => match x
					{
						//fix
						StateIO::Done => {i.send_byte(StateIO::Fitness(fit_vec[z])); z+=1;},
						StateIO::Data(y)=> {fit_vec[z] = 10000u64.wrapping_sub(y); println!("{}",y);},
						_=>(),
					},
					Err(e) => match e
					{
						TryRecvError::Empty=> (),
						TryRecvError::Disconnected => panic!("Dropped Comms")

					}
				};
				

				
			}

		}
		recv.recv();
		info!("fitness calc done");
	}

}



