#[derive(Debug)]
struct Xorshift {
	state: u32,
}

impl Xorshift {
	fn new(seed: u32) -> Xorshift {
		assert!(seed != 0);

		Xorshift{state: seed}
	}
}

impl Iterator for Xorshift {
	type Item = u32; 

	fn next(&mut self) -> Option<Self::Item> {
		let mut new_state = self.state;

		new_state ^= new_state << 13;
		new_state ^= new_state >> 17;
		new_state ^= new_state << 5;

		self.state = new_state;

		Some(new_state)
	}
}

#[cfg(test)]
mod tests {
	use Xorshift;
    #[test]
    fn it_works() {
        let mut xs = Xorshift::new(10);

        assert_eq!(
        	[2703690, 671267850, 2415639211, 3984226684],
        	xs.take(4).collect::<Vec<u32>>().as_slice()
        );
    }
}
