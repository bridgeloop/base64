use crate::{TABLE, encode_sz::encoder_output_size_usize as comp_size};

fn encode<T: B64OutputTraitInternal>(input: &[u8], output: &mut T) -> Result<(), ()> {
	if !output.size(
		comp_size(input.len())
			.and_then(|sz| usize::try_from(sz).ok())
			.ok_or(())?
	) {
		Err(())?;
	}

	let mut iter = input.iter();

	let mut prev = 0u8;
	let mut remainder = 0u8;
	let mut read_six_bits = || {
		macro_rules! set_remainder {
			($val: expr) => {
				remainder = $val;
				prev = prev.wrapping_add(1);
			};
		}

		let prev_bits = (prev & 0b11) * 2;

		let req = 6 - prev_bits;
		if req == 0 {
			let six_bits = Some(remainder);
			set_remainder!(0);
			return six_bits;
		}

		let curr_byte = iter.next()?;

		let consumed = curr_byte >> (8 - req);
		let six_bits = Some((remainder << (6 - prev_bits)) | consumed);
		set_remainder!(curr_byte & (0xff >> req));

		return six_bits;
	};
	while let Some(idx) = read_six_bits() {
		output.push(TABLE[idx as usize]);
	}
	let prev = (prev & 0b11) * 2;
	if prev != 0 {
		let req = 6 - prev;
		let idx = remainder << req;
		output.push(TABLE[idx as usize]);
	}
	output.pad();

	return Ok(());
}


pub struct B64Output<T>(Option<T>, usize);
pub trait B64OutputTrait<O> {
	fn encode<T: AsRef<[u8]>>(self, input: T) -> Result<O, ()>;
}
trait B64OutputTraitInternal {
	fn push(&mut self, byte: u8);
	fn pad(&mut self);
	fn size(&mut self, size: usize) -> bool;
}


impl B64Output<Vec<u8>> {
	pub fn to_vec() -> Self {
		return Self(None, 0);
	}
}
impl B64Output<String> {
	pub fn to_string() -> Self {
		return Self(None, 0);
	}
}
impl<'a> B64Output<&'a mut [u8]> {
	pub fn slice(slice: &'a mut [u8]) -> Self {
		return Self(Some(slice), 0);
	}
}

impl B64OutputTraitInternal for B64Output<Vec<u8>> {
	#[inline]
	fn push(&mut self, byte: u8) {
		let vec = self.0.as_mut().unwrap();
		vec.push(byte);
		return;
	}
	#[inline]
	fn pad(&mut self) {
		let vec = self.0.as_mut().unwrap();
		vec.resize(self.1, b'=');
		return;
	}
	#[inline]
	fn size(&mut self, size: usize) -> bool {
		assert_eq!(self.0, None);
		self.0 = Some(Vec::with_capacity(size));
		self.1 = size;
		return true;
	}
}
impl B64OutputTraitInternal for B64Output<String> {
	#[inline]
	fn push(&mut self, byte: u8) {
		let string = self.0.as_mut().unwrap();
		string.push(byte as char);
		return;
	}
	#[inline]
	fn pad(&mut self) {
		let string = self.0.as_mut().unwrap();
		for _ in string.len()..self.1 {
			string.push('=');
		}
		return;
	}
	#[inline]
	fn size(&mut self, size: usize) -> bool {
		assert_eq!(self.0, None);
		self.0 = Some(String::with_capacity(size));
		self.1 = size;
		return true;
	}
}
impl B64OutputTraitInternal for B64Output<&mut [u8]> {
	#[inline]
	fn push(&mut self, byte: u8) {
		let sl = self.0.as_mut().unwrap();
		sl[self.1] = byte;
		self.1 += 1;
		return;
	}
	#[inline]
	fn pad(&mut self) {
		let sl = self.0.as_mut().unwrap();
		let cap = sl.len();
		sl[self.1..cap].fill(b'=');
		return;
	}
	#[inline]
	fn size(&mut self, size: usize) -> bool {
		let sl = self.0.as_ref().unwrap();
		return sl.len() == size;
	}
}

impl B64OutputTrait<Vec<u8>> for B64Output<Vec<u8>> {
	fn encode<I: AsRef<[u8]>>(mut self, input: I) -> Result<Vec<u8>, ()> {
		encode(input.as_ref(), &mut self)?;
		return self.0.ok_or(());
	}
}
impl B64OutputTrait<String> for B64Output<String> {
	fn encode<I: AsRef<[u8]>>(mut self, input: I) -> Result<String, ()> {
		encode(input.as_ref(), &mut self)?;
		return self.0.ok_or(());
	}
}
impl B64OutputTrait<()> for B64Output<&mut [u8]> {
	fn encode<I: AsRef<[u8]>>(mut self, input: I) -> Result<(), ()> {
		return encode(input.as_ref(), &mut self);
	}
}