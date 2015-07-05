use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc::{RecvError,TryRecvError,SendError};
use std::sync::mpsc::channel;


pub struct BiChannel<T>
{
	input: Sender<T>,
	output: Receiver<T>

}

impl<T> BiChannel<T>
{

	pub fn new()->(BiChannel<T>, BiChannel<T>)
	{

		let (input_one,output_one) = channel();
		let (input_two,output_two) = channel();
		(BiChannel{input:input_one, output: output_two},BiChannel{input:input_two, output: output_one})

	}

	pub fn send(&self, byte: T) -> Result<(), SendError<T>>
	{
		self.input.send(byte)
	}
	pub fn recv(&self) -> Result<T, RecvError>
	{
		self.output.recv()
	}
	pub fn try_recv(&self) -> Result<T, TryRecvError>
	{
		self.output.try_recv()

	}


}


