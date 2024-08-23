impl Solution {
	pub fn fraction_addition(expr: String) -> String {
	  let (mut ans, mut expr) = Self::frac(expr.as_bytes());
		while let [c, rest @ ..] = expr {
			let opr;
			(opr, expr) = Self::frac(rest);
			ans = match c {
				b'+' => Self::add(ans, opr),
				_ => Self::sub(ans, opr),
			};
		}
		format!("{}/{}", ans[0], ans[1])
	}
	fn frac(expr: &[u8]) -> ([i32; 2], &[u8]) {
		match expr {
			[b'-', rest @ ..] => match Self::frac(rest) {
				([a, b], r) => ([-a, b], r),
			},
			_ => {
				let (a, rest) = Self::num(expr);
				let (b, rest) = Self::num(&rest[1..]);
				([a, b], rest)
			}
		}
	}
	fn num(expr: &[u8]) -> (i32, &[u8]) {
		match expr {
			[b'1', b'0', rest @ ..] => (10, rest),
			[c, rest @ ..] => ((c - b'0') as i32, rest),
			_ => unreachable!(),
		}
	}
    
	fn reduce([a, b]: [i32; 2]) -> [i32; 2] {
		let gcd = Self::gcd(a.abs(), b);
		[a / gcd, b / gcd]
	}
	fn add(a: [i32; 2], b: [i32; 2]) -> [i32; 2] {
		Self::reduce([a[0] * b[1] + b[0] * a[1], a[1] * b[1]])
	}
	fn sub(a: [i32; 2], b: [i32; 2]) -> [i32; 2] {
		Self::reduce([a[0] * b[1] - b[0] * a[1], a[1] * b[1]])
	}
	fn gcd(a: i32, b: i32) -> i32 {
		match a % b {
			0 => b,
			c => Self::gcd(b, c),
		}
	}
}
