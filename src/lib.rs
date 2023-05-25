// aiden@cmp.bz
// currently only implements encoding

const TABLE: [u8; 64] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[cfg(feature = "encode-sz")]
pub mod encode_sz;
#[cfg(feature = "encode-rt")]
pub mod encode_rt;
#[cfg(feature = "encode-ct")]
pub mod encode_ct;

#[cfg(test)]
mod tests {
	use {super::*, std::assert_eq};

	#[cfg(feature = "encode-sz")]
	#[test]
	fn encode_sz() {
		use encode_sz::*;

		assert_eq!(encoder_output_size(5), 8);
		assert_eq!(encoder_output_size_usize(8).unwrap(), 12);
		assert_eq!(encoder_output_size_usize_panic(12), 16);
	}

	#[cfg(feature = "encode-rt")]
	#[test]
	fn encode_rt() {
		use {encode_sz::*, encode_rt::*};

		const INP: &[u8] = &*b"encode_rt test input string";
		let mut out = [0u8; encoder_output_size_usize_panic(INP.len())];
		B64Output::slice(&mut(out)).encode(INP).unwrap();
		assert_eq!(out, *b"ZW5jb2RlX3J0IHRlc3QgaW5wdXQgc3RyaW5n");

		let vec = B64Output::to_vec().encode(INP).unwrap();
		assert_eq!(vec.as_slice(), out);

		let string = B64Output::to_string().encode(INP).unwrap();
		assert_eq!(string.as_bytes(), out);
	}

	#[cfg(feature = "encode-ct")]
	#[test]
	fn encode_ct() {
		use {encode_sz::*, encode_ct::*};

		const INP: &[u8] = &*b"hello";
		const OUT: [u8; encoder_output_size_usize_panic(INP.len())] = array_from(INP);
		
		assert_eq!(OUT, *b"aGVsbG8=");
	}
}