pub fn encoder_output_size(count: u32) -> u64 {
	return if count == 0 {
		0
	} else {
		((count as u64 + 2) * 0x55555556) >> 30 & !3
	}
}
pub fn encoder_output_size_usize(count: usize) -> Option<u64> {
	return if usize::BITS < u32::BITS || count <= u32::MAX as usize {
		Some(encoder_output_size(count as u32))
	} else {
		count.try_into().ok()
			.and_then(|count: u64| {
				let mut d = count % 3;
				if d != 0 {
					d = 3 - d;
				}
				return count.checked_add(d);
			})
			.and_then(|count| count.checked_div(3))
			.and_then(|count| count.checked_mul(4))
	};
}
pub const fn encoder_output_size_usize_panic(count: usize) -> usize {
	let mut d = count % 3;
	if d != 0 {
		d = 3 - d;
	}
	return (count + d) / 3 * 4;
}