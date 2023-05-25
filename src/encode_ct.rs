use crate::{TABLE, encode_sz::encoder_output_size_usize_panic as comp_size};

pub const fn array_from<const N: usize>(input: &[u8]) -> [u8; N] {
	if comp_size(input.len()) != N {
		panic!("output length is not encoder_output_size_usize_panic(input)");
	}

	let mut input_idx = 0;
	let mut output: [u8; N] = unsafe {
		std::mem::MaybeUninit::uninit().assume_init()
	};
	let mut output_idx = 0;

	macro_rules! push {
		($ch: expr) => {
			output[output_idx] = $ch;
			output_idx += 1;
		};
	}

	let mut prev = 0u8;
	let mut remainder = 0u8;
	let mut six_bits;
	macro_rules! read_six_bits {
		() => {
			macro_rules! set_remainder {
				($val: expr) => {
					remainder = $val;
					prev = prev.wrapping_add(1);
				};
			}

			let prev_bits = (prev & 0b11) * 2;

			let req = 6 - prev_bits;
			if req == 0 {
				six_bits = Some(remainder);
				set_remainder!(0);
			} else if input_idx == input.len() {
				six_bits = None;
			} else {
				let curr_byte = input[input_idx];
				input_idx += 1;

				let consumed = curr_byte >> (8 - req);
				six_bits = Some((remainder << (6 - prev_bits)) | consumed);
				set_remainder!(curr_byte & (0xff >> req));
			}
		};
	}

	loop {
		read_six_bits!();
		let Some(idx) = six_bits else {
			break;
		};
		push!(TABLE[idx as usize]);
	}
	let prev = (prev & 0b11) * 2;
	if prev != 0 {
		let req = 6 - prev;
		let idx = remainder << req;
		push!(TABLE[idx as usize]);
	}

	while output_idx < N {
		output[output_idx] = b'=';
		output_idx += 1;
	}

	return output;
}