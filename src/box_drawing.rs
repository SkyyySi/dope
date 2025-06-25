// SPDX-License-Identifier: MIT
#![allow(unused)]

use std::num::NonZeroU8;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum LineStyle {
	None,
	Normal,
	Heavy,
	Rounded,
}

impl LineStyle {
	pub fn is_none(&self)    -> bool { matches!(self, Self::None) }
	pub fn is_normal(&self)  -> bool { matches!(self, Self::Normal) }
	pub fn is_heavy(&self)   -> bool { matches!(self, Self::Heavy) }
	pub fn is_rounded(&self) -> bool { matches!(self, Self::Rounded) }

	pub fn not_none(&self)    -> bool { !self.is_none() }
	pub fn not_normal(&self)  -> bool { !self.is_normal() }
	pub fn not_heavy(&self)   -> bool { !self.is_heavy() }
	pub fn not_rounded(&self) -> bool { !self.is_rounded() }
}

/*pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}*/

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Symbol {
	up: LineStyle,
	down: LineStyle,
	left: LineStyle,
	right: LineStyle,
	segments_count: NonZeroU8,
}

use std::collections::HashMap;
fn lookup_symbol(symbol: &Symbol) -> Option<char> {
	let mut chars: HashMap<Symbol, char> = HashMap::new();

	use LineStyle::{None as No, Normal as N, Heavy as H, Rounded as R};

	macro_rules! insert {
		( $( ( $up:expr, $down:expr, $left:expr, $right:expr, $segments:literal ) => $c:literal ),* $(,)? ) => {
			$(chars.insert(
				Symbol::new($left, $right, $up, $down, NonZeroU8::try_from($segments).unwrap()),
				$c,
			);)*
		};
	}

	insert! {
		(No, No, No, No, 1) => ' ',
		(No, No, N, N, 1) => '─',
	};

	chars.get(symbol).map(|c| c.to_owned())
}

impl Symbol {
	pub fn new(
			up: LineStyle,
			down: LineStyle,
			left: LineStyle,
			right: LineStyle,
			segments_count: NonZeroU8,
			//segments_count: u8,
	) -> Self {
		assert!(segments_count.get() <= 4);
		//assert!(segments_count <= 4);
		Self {
			up,
			down,
			left,
			right,
			segments_count,
			//segments_count: NonZeroU8::try_from(segments_count).unwrap(),
		}
	}

	fn has_up(&self)    -> bool { self.up.not_none() }
	fn has_down(&self)  -> bool { self.down.not_none() }
	fn has_left(&self)  -> bool { self.left.not_none() }
	fn has_right(&self) -> bool { self.right.not_none() }

	fn directions_as_bits(&self) -> u32 {
		let mut result: u32 = 0;

		if (self.has_up())    { result |= 0b0001; }
		if (self.has_down())  { result |= 0b0010; }
		if (self.has_left())  { result |= 0b0100; }
		if (self.has_right()) { result |= 0b1000; }

		debug_assert!(result <= 0b1111);

		result
	}

	pub fn get_char(&self) -> char {
		let bits: u32 = self.directions_as_bits();

		let mut result: u32 = '─' as u32; // '\u{2500}'

		if bits == 0b1100 && self.left == self.right { // left + right
			result += match self.segments_count.get() {
				1 => 0x00,
				2 => 0x4c,//(16 * 4) + 13,
				3 => 0x04,
				4 => 0x08,
				_ => unreachable!(),
			};

			result += self.left.is_heavy() as u32;
		}

		result.try_into().unwrap()
	}
}
