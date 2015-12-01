extern crate distrgp_generator;
extern crate threadpool;
use self::distrgp_generator::StateIO;
use self::distrgp_generator::SpecialOperator;

use std::sync::{Arc, Mutex};

use super::GreenThreadData;
use threadpool::ThreadPool;

pub fn step(mut state: GreenThreadData, pool: Arc<Mutex<ThreadPool>>) {

    loop {

        let (suc1, suc2);
        let operator;
        let local_life: u64;
        {
            let lock = state.global_state.graph.lock().unwrap();
            let (x, y) = lock.get_sucessor_index(state.local_state.node.unwrap());
            operator = lock.get_operator(state.local_state.node.unwrap());
            suc1 = x;
            suc2 = y;



            let life = state.global_state.life.clone().unwrap();
            let mut lifelock = life.lock().unwrap();


            if *lifelock > state.map.get(&operator).unwrap().get_base_cost() {
                *lifelock = *lifelock - state.map.get(&operator).unwrap().get_base_cost();
                local_life = *lifelock;
            } else {

                *lifelock = 0;
                local_life = 0;
            }


        }
        let sucessor_bool = state.map
                                 .get(&operator)
                                 .unwrap()
                                 .call(&mut state.global_state, &mut state.local_state);



        if suc1 == None || local_life == 0 {

            if local_life == 0 {
                debug!("Killed overunning individual");

            }
            let thread = state.global_state.thread_count.unwrap();
            let mut threadlock = thread.lock().unwrap();
            assert!(*threadlock >= 1);
            if *threadlock == 1 {



                let comm = state.global_state.comm.clone().unwrap();
                let commlock = comm.lock().unwrap();

                let fitness = state.global_state.fitness.clone().unwrap();
                let mut fitlock = fitness.lock().unwrap();

                let life = state.global_state.life.clone().unwrap();
                let lifelock = life.lock().unwrap();

                let graphlock = state.global_state.graph.lock().unwrap();



                match commlock.send(StateIO::Done) {
                    Ok(_) => (),
                    _ => panic!("Dropped Comms"),

                }
                match commlock.send(StateIO::SizeGraph(graphlock.get_size() as u64)) {
                    Ok(_) => (),
                    _ => panic!("Dropped Comms"),

                }
                match commlock.send(StateIO::Life(*lifelock)) {
                    Ok(_) => (),
                    _ => panic!("Dropped Comms"),

                }

                // clear input
                loop {
                    match commlock.recv() {
                        Ok(x) => {
                            match x {
                                StateIO::Data(_) => (),
                                StateIO::Fitness(y) => {
                                    *fitlock = y;
                                    break;
                                }
                                _ => panic!("Invalid Data"),
                            }
                        }
                        Err(_) => panic!("Dropped Comms"),

                    }
                }

            } else {
                *threadlock = *threadlock - 1;

            }
            return;
        }

        match state.map.get(&operator).unwrap().get_special() {
            SpecialOperator::NewThread => {



                let mut state2 = state.clone();

                state.local_state.node = Some(suc1.unwrap());

                state2.local_state.node = Some(suc2.unwrap());
                let thread = state.global_state.thread_count.clone().unwrap();
                let mut threadlock = thread.lock().unwrap();
                *threadlock = *threadlock + 1;
                // This has to happen otherwise the strong refs to pool drops to
                // 1 and the parent thread continues into an unknown state
                {

                    let lock = pool.lock().unwrap();
                    let pool_clone = pool.clone();
                    lock.execute(move || {
                        step(state2, pool_clone);
                    });
                }




            }
            _ => {

                let index;
                if sucessor_bool {

                    index = suc1.unwrap();
                } else {
                    index = suc2.unwrap();

                }

                state.local_state.node = Some(index);


            }

        }

    }



}
