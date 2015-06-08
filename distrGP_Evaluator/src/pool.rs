//This Source file contains code copied and modified from the "thread pool" rust library at https://github.com/rust-lang/threadpool/
//The original copyright notice for that code is below 

// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Evaluates an individual using thread pools

extern crate distrGP_Generator;

use self::distrGP_Generator::OperatorMap;
use self::distrGP_Generator::Graph;
use self::distrGP_Generator::GlobalState;
use self::distrGP_Generator::LocalState;
use self::distrGP_Generator::StateIO;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::TryRecvError;
use std::mem;
use std::thread::JoinGuard;



pub struct GreenThreadData<'pool>
{
	pub global_state: GlobalState,
	pub local_state: LocalState,
	map: &'pool OperatorMap
}

impl<'pool> GreenThreadData<'pool>
{
	pub fn new(global_state: GlobalState,local_state: LocalState, map: &'pool OperatorMap ) -> GreenThreadData<'pool>
	{
		GreenThreadData{global_state: global_state, local_state: local_state, map: map}

	}

}


pub struct ThreadPool<'pool>  {
    // How the threadpool communicates with subthreads.
    //
    // This is the only such Sender, so when it is dropped all subthreads will
    // quit.
    jobs: Option<Sender<(GreenThreadData<'pool>,Arc<Mutex<ThreadPool<'pool>>>)>>,
     _guards: Vec<JoinGuard<'pool, ()>>
}

impl<'pool> ThreadPool<'pool> {
        /// Spawns a new thread pool with `threads` threads.
        ///
        /// # Panics
        ///
        /// This function will panic if `threads` is 0.
        pub fn new(threads: usize) -> ThreadPool<'pool>
	{
		 assert!(threads >= 1);
	    
		 let (tx, rx) = channel::<(GreenThreadData,Arc<Mutex<ThreadPool>>)>();
		 let rx = Arc::new(Mutex::new(rx));
	    
		let mut guards = Vec::with_capacity(threads);
		for _ in 0..threads 
		{
			guards.push(spawn_in_pool(rx.clone()));
		}

		ThreadPool { jobs: Some(tx), _guards: guards }
	}
		    

        /// Executes the function `job` on a thread in the pool.
        pub fn execute(&self, job: GreenThreadData<'pool>, pool: Arc<Mutex<ThreadPool<'pool>>>)
        {
            self.jobs.as_ref().unwrap().send((job, pool));
        }
        
}

impl<'a> Drop for ThreadPool<'a> {
    fn drop(&mut self) {
        // We need to ensure that the sender is dropped before the JoinGuards
        // Otherwise the threads will be joined and wait forever in the loop
        mem::replace(&mut self.jobs, None);
    }
}

fn spawn_in_pool<'pool>(jobs: Arc<Mutex<Receiver<(GreenThreadData<'pool>,Arc<Mutex<ThreadPool<'pool>>>)>>>) -> JoinGuard<'pool, ()>
{
        thread::scoped(move || {
           // Will spawn a new thread on panic unless it is cancelled.
            loop
	    {
                let message = {
                    // Only lock jobs for the time it takes
                    // to get a job, not run it.
                    let lock = jobs.lock().unwrap();
                    lock.recv()
                };
                match message {
                    Ok((data,pool)) => step(data,pool),
    
                    // The Threadpool was dropped.
                    Err(..) => break
                }
            };
        })
}



fn step<'pool>(mut state: GreenThreadData<'pool>, pool: Arc<Mutex<ThreadPool<'pool>>>)
{

    	let (mut suc1,mut suc2);
	let mut operator;
	{
		let lock = state.global_state.graph.lock().unwrap();
		let (x,y) = lock.get_sucessor_index(state.local_state.node.unwrap());
		operator= lock.get_operator(state.local_state.node.unwrap());
		suc1 =x; suc2 =y;


	}

	let sucessor_bool =state.map.get(&operator).unwrap().call(&mut state.global_state,&mut state.local_state);

	let index;

	if suc1 == None
	{

		let thread = state.global_state.thread_count.unwrap();
		let mut threadlock = thread.lock().unwrap();
		assert!(*threadlock >= 1);
		if *threadlock == 1
		{
	
			let output = state.global_state.output.clone().unwrap();
			let outlock = output.lock().unwrap();

			let input = state.global_state.input.clone().unwrap();
			let inlock = input.lock().unwrap();

			let fitness = state.global_state.fitness.clone().unwrap();
			let mut fitlock = fitness.lock().unwrap();


			match outlock.send(StateIO::Done)
			{
				Ok(_) => (),
				_=> panic!("Dropped Comms")

			}


			//clear input
			loop
			{
				match inlock.try_recv()
				{
					Ok(x) => match x
					{
						StateIO::Data(_) => (),
						StateIO::Fitness(y) => {*fitlock = y; break},
						_=> panic!("Invalid Data"),
					},
					Err(e) => match e
					{
						TryRecvError::Empty=> (),
						TryRecvError::Disconnected => panic!("Dropped Comms")

					}
				}	
			}

		}
		else
		{
			*threadlock = *threadlock - 1;

		}
		
		return;
	}

	if sucessor_bool
	{

		index = suc1.unwrap();
	}
	else
	{
		index = suc2.unwrap();

	}

	state.local_state.node = Some(index);


	//This has to happen otherwise the strong refs to pool drops to 1 and the parent thread continues into an unknown state
	{
		let lock= pool.lock().unwrap();
		lock.execute(state,pool.clone());
	}
	    
			


} 
