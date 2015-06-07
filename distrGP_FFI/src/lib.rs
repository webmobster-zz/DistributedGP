

#[no_mangle]
pub extern fn defaultgen(popcount: usize) -> Box<(txt,rx)>)
{
	let generator = Generator::init(
				
				problem_description.get_popcount(),
				problem_description.get_operators(),

				problem_description.get_selector(),
				vec!(Box::new(TreeCross::new(1.0)) as Box<GeneticOperator>),
				Box::new(StandardGrow::new(1.0,300)),
				problem_description.get_life()
				);
	//names
	let (tx, rx) = channel();
	let (txt, rxt) = channel();
	thread::spawn(move || {
		server::init(generator,threads,tx,rxt);
	});
	Box::new((txt,rx))

	
}



#[no_mangle]
pub extern fn load_initial_values(comms: &mut Box<(txt,rx)>), individual: usize, data: *[u64]) -> Vec<IndividualComm>
{


	comms.send(data);
	
}


#[no_mangle]
//0 is nothing, 1 is byte ready, 2 is done
pub extern fn check_byte(comm: IndividualComm, individual: usize, data: u64) -> u8
{


	comm.check_byte(individual)
	
}

#[no_mangle]
pub extern fn read_byte(comm: IndividualComm, individual: usize, data: u64) -> u64
{


	comm.read_byte(individual)
	
}

#[no_mangle]
pub extern fn send_byte(comms: IndividualComm, individual: usize, data: u64)
{


	comm.send_byte(individual);
	
}
