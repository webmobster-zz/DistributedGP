#![crate_type = "bin"]

#![crate_name = "distrgp_testimpl"]



//#![deny(missing_docs)]
//#![deny(warnings)]

extern crate rand;
extern crate distrgp_evaluator;
extern crate distrgp_providedoperators;
extern crate distrgp_generator;
extern crate distrgp_util;
extern crate env_logger;
#[macro_use]
extern crate log;

use log::{LogRecord, LogLevel, LogMetadata};
use distrgp_generator::GeneticOperator;
use distrgp_generator::Compiler;
use distrgp_generator::Generator;
use distrgp_generator::BiChannel;
use distrgp_generator::StateIO;
use distrgp_generator::LocalState;
use distrgp_generator::GlobalState;


use distrgp_providedoperators::geneticoperators::TreeCross;
use distrgp_providedoperators::geneticoperators::FlatCross;
use distrgp_providedoperators::geneticoperators::PointMutate;
use distrgp_providedoperators::geneticoperators::StandardGrow;
use distrgp_providedoperators::geneticoperators::Rewire;
use distrgp_providedoperators::geneticoperators::Clean;
use distrgp_providedoperators::geneticoperators::InsertNode;
use distrgp_evaluator::FitnessMessage;

use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc::TryRecvError;
use std::sync::mpsc::channel;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};


use rand::distributions::{IndependentSample, Range};




fn main() {

    env_logger::init().unwrap();

    info!("init");
    //Basic commandline arguments, expand later


    //read problem description file



    let selector = ::distrgp_providedoperators::selectors::Tournament::new(2);
    //names
    let (fit_end_one, fit_end_two) = BiChannel::new();
    let (util_end_one, util_end_two) = BiChannel::new();
    let file = "sddssd".to_string();

    let tree_cross = TreeCross::new(0.05);
    let flat_cross = FlatCross::new(0.1);
    let point_mutate = PointMutate::new(0.25);
    let rewire = Rewire::new(0.25);
    let clean = Clean::new(0.05);
    let insert_node = InsertNode::new(0.3);
    let grow =  StandardGrow::new(1.0, 200);
    let dummy_thing = dummy_compiler;


    thread::spawn(move || {

                           let generator = Generator::init(5000,
                                                           file,
                                                           &selector,
                                                           vec!(    &tree_cross as &GeneticOperator,
                                                                    &flat_cross as &GeneticOperator,
                                                                    &point_mutate as &GeneticOperator,
                                                                    &rewire as &GeneticOperator,
                                                                    &clean as &GeneticOperator,
                                                                    &insert_node as &GeneticOperator,
                                                            ),
                                                            &grow,
                                                           1000000,
                                                            &dummy_thing);
                           distrgp_evaluator::init(generator, 12, fit_end_two, util_end_two);
                       });
    thread::spawn(move || {
                           distrgp_util::util_placeholder_runner(util_end_one);
                       });
    fitness(fit_end_one);

    loop {}


}

struct dummy_compiler;

impl Compiler for dummy_compiler
{
    fn compile(&self, code : String) -> fn(&mut GlobalState, &mut LocalState) -> bool
    {
        return dummy;
    }
}

fn dummy(one: &mut GlobalState, two: &mut LocalState) -> bool
{
    return true;
}

fn fitness(comm: BiChannel<FitnessMessage>) {

    let mut average_size: f64 = 300.0;
    loop {
        info!("Average Graph Size ={}", average_size);
        let mut ind_comms = match comm.recv() {
            Ok(y) => match y {
                FitnessMessage::PopVec(x) => x,
                _ => panic!("Invalid Message"),
            },
            _ => panic!("Dropped sender"),
        };
        info!("Received Communication Channels");
        comm.send(FitnessMessage::Ready);
        let fit_state_vec = initialize_tests(ind_comms.len() as u64);
        ind_fitness(ind_comms, fit_state_vec, &mut average_size);
        comm.send(FitnessMessage::PopFin);
        info!("Finished Calculating Fitnesses");
        comm.recv();

    }

}

#[derive(Clone)]
struct fitness_state {
    pub problem_vec: Vec<(u64, u64)>,
    pub receive_vec: Vec<u64>,
    pub size: Option<u64>,
    pub life: Option<u64>,
}

fn initialize_tests(pop_count: u64) -> Vec<fitness_state> {
    let mut rng = rand::thread_rng();
    let mut fit_state_vec = Vec::new();
    let reps = Range::new(500u64, 1000);
    let repititions = reps.ind_sample(&mut rng);
    let between = Range::new(200u64, 1000);


    for i in 0..pop_count {

        let mut problem_vec: Vec<(u64, u64)> = Vec::new();
        for i in 0..repititions {
            let test = (between.ind_sample(&mut rng), between.ind_sample(&mut rng));
            problem_vec.push(test);
        }

        let mut receive_vec: Vec<u64> = Vec::new();
        fit_state_vec.push(fitness_state{ problem_vec: problem_vec, receive_vec: receive_vec, size: None, life: None});
    }
    fit_state_vec




}

fn ind_fitness(mut comms: Vec<BiChannel<StateIO>>,
               mut fit_state_vec: Vec<fitness_state>,
               average_size: &mut f64) {




    let pop_count = comms.len();

    let mut x = 0;
    for i in comms.iter_mut() {

        for (a,b) in fit_state_vec[x].problem_vec.clone() {
            i.send(StateIO::Data(a));
            i.send(StateIO::Data(b));
        }
        x += 1;

    }

    let mut z = 0;
    loop {


        if comms.len() == z {

            break
        }
        let mut x = 0;
        let mut progress_made = false;
        for mut i in comms.iter_mut() {


            match i.try_recv() {
                Ok(p) => {
                    progress_made = true;
                    match p {
                        StateIO::Done => {

                            fit_state_vec[x].size = Some(match i.recv() {
                                    Ok(p) => match p {
                                        StateIO::SizeGraph(x) => x,
                                        _ => panic!("invalid message"),
                                    },
                                    Err(_) => panic!("Dropped Comms"),
                                });
                            fit_state_vec[x].life = Some(match i.recv() {
                                    Ok(p) => match p {
                                        StateIO::Life(x) => x,
                                        _ => panic!("invalid message"),
                                    },
                                    Err(_) => panic!("Dropped Comms"),
                                });

                            let fitness = fitness_calc(fit_state_vec[x].clone(),
                                                       average_size,
                                                       pop_count as u64);
                            i.send(StateIO::Fitness(fitness));
                            z += 1;


                        }
                        StateIO::Data(y) => {
                            fit_state_vec[x].receive_vec.push(y);
                        }
                        _ => (),
                    }
                }
                Err(e) => match e {
                    TryRecvError::Empty => (),
                    TryRecvError::Disconnected => {
                        println!("z={}",z);
                        panic!("Dropped Comms");
                    }
                },
            };
            x += 1;




        }
        debug!("waiting for {} to finish",comms.len() -z);
        if !progress_made {
            debug!("No progress made");
            thread::sleep_ms(5);
        }

    }



}

fn fitness_calc(fit_state: fitness_state, average_size: &mut f64, pop_count: u64) -> u64 {
    let size = fit_state.size.unwrap();
    let life = fit_state.life.unwrap();
    *average_size = (*average_size * (pop_count -1) as f64 + size as f64) / pop_count as f64;

    let mut cumm_fit: f64 = 0.0;
    if life == 0 {
        return 900000
    }



    for i in 0..fit_state.problem_vec.len() {

        let (a,b) = fit_state.problem_vec[i];

        if i < fit_state.receive_vec.len() {
            let result = fit_state.receive_vec[i];
            let mut fit_percent = 100.0 *
                                  (( a + b ) as i64 -fit_state.receive_vec[i] as i64).abs() as f64 /
                                  (a+b) as f64;
            fit_percent = fit_percent.powi(2);

            cumm_fit += fit_percent;
        } else {
            cumm_fit += 10000.;

        }
    }

    if fit_state.receive_vec.len() > fit_state.problem_vec.len() {
        cumm_fit += 10000. * (fit_state.receive_vec.len() - fit_state.problem_vec.len()) as f64;

    }


    cumm_fit = cumm_fit / fit_state.problem_vec.len() as f64;

    //difference is a percent so 30 is 30% bigger, 100 is 100% bigger
    let difference = 100.0 * (size as f64 - *average_size) / *average_size;

    //0.943989 b increased slightly from correct/perfect value for a bit of safety in rounding

    let a: f64 = 5.;
    let b: f64 = 0.0;
    let k: f64 = 0.0230259;

    //a*e^(k*x)+b
    //plug three points that you want on the curve into an equation solver.
    //this example uses 0,1 30,2 100,1000 to get a shallow penalty till 30, afterwards it swiftly grows

    //0,2 30,5 100,1000
    let penalty: f64 = a * (k*difference).exp() + b;

    //println!("penalty={}, size={1}, average = {2}",penalty,size,*average_size);

    let mut final_fit = (cumm_fit*10.0) as u64 + penalty as u64 + size as u64;


    final_fit
}
