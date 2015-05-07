
use super::envmessage::EnvMessage;
use super::envmessage::EnvMessage::{PopClient, EndPopClient, Startclient,OperatorTraitEnv, RepetitionsEnv};

use super::super::server::generator::graph::Graph;
use std::collections::HashSet;

use super::super::server::generator::operator::OperatorTrait;

use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;




pub fn init(send: SyncSender<EnvMessage>, receive: Receiver<EnvMessage>)
{ 


	
	loop
	{

		match receive.recv()
		{
			Ok(msg) => {match msg
			{
				Startclient => {},
				_=> {panic!("Invalid Message");}
			};},
			Err(_)=> panic!("receive error")
				
		}


		let op_trait;

		match receive.recv()
		{
			Ok(msg) => {match msg
			{
				OperatorTraitEnv(x) => {op_trait = x},
				_=> {panic!("Invalid Message");}
			};},
			Err(_)=> panic!("receive error")
				
		}



		let repetitions;

		match receive.recv()
		{
			Ok(msg) => {match msg
			{
				RepetitionsEnv(x) => {repetitions = x},
				_=> {panic!("Invalid Message");}
			};},
			Err(_)=> panic!("receive error")
				
		}






		// receive population

		let mut vec_pops;
		match receive.recv()
		{
			Ok(msg) => {match msg
			{
				PopClient(x) => vec_pops=x,
				_=> {panic!("Invalid Message");}
			};},
			Err(_)=> panic!("receive error")
					

		};	

		match receive.recv()
		{
			Ok(msg) => {match msg
			{
				EndPopClient => (),
				_=> {panic!("Invalid Message");}
			};},
			Err(_)=> panic!("receive error")

			
		};	

		println!("env got {} graphs",vec_pops.len());

		// run all populations and send fitnesses 
		run(&mut vec_pops,op_trait, repetitions);

		assert!(send.send(PopClient(vec_pops)).is_ok());


	

		//loop back to beginning




	}
	
}
#[allow(unused_mut)]
fn run(pop: &mut Box<Vec<Graph>>, mut op_trait: Vec<Box<OperatorTrait + Send>>,repetitions: u32)
{


	for i in 0 .. pop.len()
	{

		let working_graph = (&mut **pop).get_mut(i).unwrap();

		let mut op_trait_clone = Vec::new();
		for z in 0..op_trait.len()
		{
			op_trait_clone.push(op_trait[z].clone());

		}
		let (result,perfect) = iterate_over_entity(working_graph, op_trait_clone,repetitions);

		working_graph.set_fitness(result);
		working_graph.set_perfect(perfect);

			

	}

}
//result, life
fn iterate_over_entity(entity: &Graph,  mut op_list: Vec<Box<OperatorTrait + Send>>, repetitions: u32) -> (u32,bool)
{
	let mut set = HashSet::new();
	let mut dups =0;
	let mut fitness_life_list: Vec<Option<(u32,u32)>> = Vec::new();
	
	for i in 0 .. repetitions as usize
	{

		let mut op_trait = op_list[i].clone();



		if !op_trait.init_state()
		{
			panic!("unitialized data");

		}

		if !set.insert(op_trait.get_random())
		{
			dups +=1;

		}

		let mut life = entity.life;


		

		let mut index: isize = 0;
		


	
		loop
		{

			let (suc1,suc2,_) = entity.get_sucessor_index(index as usize);



	
			if life == 0
			{
				fitness_life_list.push(None);
				//println!("outof life");
				break 

			}


			let trait_index = entity.get_operator(index as usize).call();

			let sucessor_bool = op_trait.op(trait_index);

		
			if sucessor_bool
			{

				index = suc1;
			}
			else
			{
				index = suc2;

			}


			if suc1 == -1
			{

				let fitness = op_trait.fitness();
				fitness_life_list.push(Some((fitness,life)));
				break;

			}
			

			life = life -1;



		}




	}
	//println!("{} dups 2",dups);
	op_list[0].secondary(entity.list.len(),&mut fitness_life_list)



}




