#![crate_type = "bin"]

#![crate_name = "distrGP"]



//#![deny(missing_docs)]
//#![deny(warnings)]

extern crate rand;
extern crate distr_gp_evaluator;
extern crate distrGP_ProvidedOperators;
extern crate distrGP_Generator;
extern crate env_logger;
#[macro_use]
extern crate log;

use log::{LogRecord, LogLevel, LogMetadata};
use distrGP_Generator::GeneticOperator;
use distrGP_Generator::Generator;
use distrGP_Generator::IndividualComm;
use distrGP_Generator::StateIO;

use distrGP_ProvidedOperators::geneticoperators::TreeCross;
use distrGP_ProvidedOperators::geneticoperators::FlatCross;
use distrGP_ProvidedOperators::geneticoperators::PointMutate;
use distrGP_ProvidedOperators::geneticoperators::StandardGrow;
use distr_gp_evaluator::FitnessMessage;


use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::channel;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};

use rand::distributions::{IndependentSample, Range};

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
				
				5000,
				problem_description.get_operators(),

				problem_description.get_selector(),
				vec!(	Box::new(TreeCross::new(0.55)) as Box<GeneticOperator>,
					Box::new(FlatCross::new(0.05)) as Box<GeneticOperator>,
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
		ind_fitness(thing);
		send.send(FitnessMessage::PopFin);
		recv.recv();
		info!("fitness calc done");
	}

}


fn ind_fitness(mut comms: Vec<IndividualComm>)
{

	let between = Range::new(5u64,10);
   	let mut rng = rand::thread_rng();
	let test= (between.ind_sample(&mut rng),between.ind_sample(&mut rng));
	let mut fit_vec: Vec<u64> = std::iter::repeat(100000).take(comms.len()).collect::<Vec<_>>();;
	let mut problem_vec: Vec<(u64,u64)> = std::iter::repeat(test).take(comms.len()).collect::<Vec<_>>();;


	let mut x=0;
	for i in comms.iter_mut()
	{
		let (a,b)= problem_vec[x];
		i.send_byte(StateIO::Data(a));
		i.send_byte(StateIO::Data(b));
		x+=1;

	}

	let mut z = 0;
	loop
	{


		if comms.len() == z
		{
			fit_vec.sort();
			println!("best={}",fit_vec[0]);
			break
		}
		let mut x=0;
		for mut i in comms.iter_mut()
		{




			match i.try_receive_byte()
			{
				Ok(p) => match p
				{
					//fix
					StateIO::Done => {i.send_byte(StateIO::Fitness(fit_vec[x])); z+=1;},
					StateIO::Data(y)=> {
								let (a,b)= problem_vec[x];
								fit_vec[x] = (a+b).wrapping_sub(y);
								//println!("a={0}, b={1}, data={2}",a,b,y)
								},
					_=>(),
				},
				Err(e) => match e
				{
					TryRecvError::Empty=> (),
					TryRecvError::Disconnected =>{println!("z={}",z); panic!("Dropped Comms");}

				}
			};
			x+=1;

		

			
		}
		thread::sleep_ms(50);

	}



}


