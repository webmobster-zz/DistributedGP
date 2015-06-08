use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::{RecvError,TryRecvError,SendError};

use super::StateIO;

pub struct IndividualComm
{
	input: Sender<StateIO>,
	output: Receiver<StateIO>

}

impl IndividualComm
{

	pub fn new(input: Sender<StateIO>, output: Receiver<StateIO>)->IndividualComm
	{
		IndividualComm{input:input, output: output}

	}

	pub fn send_byte(&mut self, byte: StateIO) -> Result<(), SendError<StateIO>>
	{
		self.input.send(byte)
	}
	pub fn receive_byte(&mut self) -> Result<StateIO, RecvError>
	{
		self.output.recv()
	}
	pub fn try_receive_byte(&mut self) -> Result<StateIO, TryRecvError>
	{
		self.output.try_recv()

	}


}


