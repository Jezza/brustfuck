
#[macro_export]
macro_rules! brainfuck {
	(
		$($tt:tt)*
	) => {
		$crate::brainfuck_inner! {
			#function
			{
				_evaluate;
				_r;
				_w;
				_data;
				_ptr;
				raw = false;
			}

			#body
			{}

			#token
			[{ $($tt)* }]
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! brainfuck_raw {
	(
		$($tt:tt)*
	) => {
		$crate::brainfuck_inner! {
			#function
			{
				_evaluate;
				_r;
				_w;
				_data;
				_ptr;
				raw = true;
			}

			#body
			{}

			#token
			[{ $($tt)* }]
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! brainfuck_inner {
	// Right Shift
	(
		#function
		{
			$inner_name:ident;
			$read:ident;
			$write:ident;
			$data:ident;
			$ptr:ident;
			raw = $raw_value:ident;
		}

		#body
		{$($body:tt)*}

		#token
		[{ >> $($rest:tt)* }]
	) => {
		$crate::brainfuck_inner! {
			#function
			{
				$inner_name;
				$read;
				$write;
				$data;
				$ptr;
				raw = $raw_value;
			}

			#body
			{
				$($body)*
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::Right, &mut $data, $ptr);
				$ptr = $ptr.wrapping_add(1);
				#[cfg(feature = "debug")]
				$crate::on_op_post($crate::Op::Right, &mut $data, $ptr);
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::Right, &mut $data, $ptr);
				$ptr = $ptr.wrapping_add(1);
				#[cfg(feature = "debug")]
				$crate::on_op_post($crate::Op::Right, &mut $data, $ptr);
			}

			#token
			[{ $( $rest )* }]
		}
	};
	(
		#function
		{
			$inner_name:ident;
			$read:ident;
			$write:ident;
			$data:ident;
			$ptr:ident;
			raw = $raw_value:ident;
		}

		#body
		{$($body:tt)*}

		#token
		[{ > $($rest:tt)* }]
	) => {
		$crate::brainfuck_inner! {
			#function
			{
				$inner_name;
				$read;
				$write;
				$data;
				$ptr;
				raw = $raw_value;
			}

			#body
			{
				$($body)*
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::Right, &mut $data, $ptr);
				$ptr = $ptr.wrapping_add(1);
				#[cfg(feature = "debug")]
				$crate::on_op_post($crate::Op::Right, &mut $data, $ptr);
			}

			#token
			[{ $( $rest )* }]
		}
	};

	// Left Shift
	(
		#function
		{
			$inner_name:ident;
			$read:ident;
			$write:ident;
			$data:ident;
			$ptr:ident;
			raw = $raw_value:ident;
		}

		#body
		{$($body:tt)*}

		#token
		[{ << $($rest:tt)* }]
	) => {
		$crate::brainfuck_inner! {
			#function
			{
				$inner_name;
				$read;
				$write;
				$data;
				$ptr;
				raw = $raw_value;
			}

			#body
			{
				$($body)*
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::Left, &mut $data, $ptr);
				$ptr = $ptr.wrapping_sub(1);
				#[cfg(feature = "debug")]
				$crate::on_op_post($crate::Op::Left, &mut $data, $ptr);
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::Left, &mut $data, $ptr);
				$ptr = $ptr.wrapping_sub(1);
				#[cfg(feature = "debug")]
				$crate::on_op_post($crate::Op::Left, &mut $data, $ptr);
			}

			#token
			[{ $( $rest )* }]
		}
	};
	(
		#function
		{
			$inner_name:ident;
			$read:ident;
			$write:ident;
			$data:ident;
			$ptr:ident;
			raw = $raw_value:ident;
		}

		#body
		{$($body:tt)*}

		#token
		[{ < $($rest:tt)* }]
	) => {
		$crate::brainfuck_inner! {
			#function
			{
				$inner_name;
				$read;
				$write;
				$data;
				$ptr;
				raw = $raw_value;
			}

			#body
			{
				$($body)*
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::Left, &mut $data, $ptr);
				$ptr = $ptr.wrapping_sub(1);
				#[cfg(feature = "debug")]
				$crate::on_op_post($crate::Op::Left, &mut $data, $ptr);
			}

			#token
			[{ $( $rest )* }]
		}
	};

	// Plus
	(
		#function
		{
			$inner_name:ident;
			$read:ident;
			$write:ident;
			$data:ident;
			$ptr:ident;
			raw = $raw_value:ident;
		}

		#body
		{$($body:tt)*}

		#token
		[{ + $($rest:tt)* }]
	) => {
		$crate::brainfuck_inner! {
			#function
			{
				$inner_name;
				$read;
				$write;
				$data;
				$ptr;
				raw = $raw_value;
			}

			#body
			{
				$($body)*
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::Plus, &mut $data, $ptr);
				$data[$ptr] = $data[$ptr].wrapping_add(1);
				#[cfg(feature = "debug")]
				$crate::on_op_post($crate::Op::Plus, &mut $data, $ptr);
			}

			#token
			[{ $( $rest )* }]
		}
	};

	// Minus
	(
		#function
		{
			$inner_name:ident;
			$read:ident;
			$write:ident;
			$data:ident;
			$ptr:ident;
			raw = $raw_value:ident;
		}

		#body
		{$($body:tt)*}

		#token
		[{ - $($rest:tt)* }]
	) => {
		$crate::brainfuck_inner! {
			#function
			{
				$inner_name;
				$read;
				$write;
				$data;
				$ptr;
				raw = $raw_value;
			}

			#body
			{
				$($body)*
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::Minus, &mut $data, $ptr);
				$data[$ptr] = $data[$ptr].wrapping_sub(1);
				#[cfg(feature = "debug")]
				$crate::on_op_post($crate::Op::Minus, &mut $data, $ptr);
			}

			#token
			[{ $( $rest )* }]
		}
	};

	// Dot
	(
		#function
		{
			$inner_name:ident;
			$read:ident;
			$write:ident;
			$data:ident;
			$ptr:ident;
			raw = $raw_value:ident;
		}

		#body
		{$($body:tt)*}

		#token
		[{ .. $($rest:tt)* }]
	) => {
		$crate::brainfuck_inner! {
			#function
			{
				$inner_name;
				$read;
				$write;
				$data;
				$ptr;
				raw = $raw_value;
			}

			#body
			{
				$($body)*
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::Print, &mut $data, $ptr);
				$write.write(&$data[$ptr..=$ptr])?;
				#[cfg(feature = "debug")]
				$crate::on_op_post($crate::Op::Print, &mut $data, $ptr);
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::Print, &mut $data, $ptr);
				$write.write(&$data[$ptr..=$ptr])?;
				#[cfg(feature = "debug")]
				$crate::on_op_post($crate::Op::Print, &mut $data, $ptr);
			}

			#token
			[{ $( $rest )* }]
		}
	};
	(
		#function
		{
			$inner_name:ident;
			$read:ident;
			$write:ident;
			$data:ident;
			$ptr:ident;
			raw = $raw_value:ident;
		}

		#body
		{$($body:tt)*}

		#token
		[{ . $($rest:tt)* }]
	) => {
		$crate::brainfuck_inner! {
			#function
			{
				$inner_name;
				$read;
				$write;
				$data;
				$ptr;
				raw = $raw_value;
			}

			#body
			{
				$($body)*
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::Print, &mut $data, $ptr);
				$write.write(&$data[$ptr..=$ptr])?;
				#[cfg(feature = "debug")]
				$crate::on_op_post($crate::Op::Print, &mut $data, $ptr);
			}

			#token
			[{ $( $rest )* }]
		}
	};

	// Comma
	(
		#function
		{
			$inner_name:ident;
			$read:ident;
			$write:ident;
			$data:ident;
			$ptr:ident;
			raw = $raw_value:ident;
		}

		#body
		{$($body:tt)*}

		#token
		[{ , $($rest:tt)* }]
	) => {
		$crate::brainfuck_inner! {
			#function
			{
				$inner_name;
				$read;
				$write;
				$data;
				$ptr;
				raw = $raw_value;
			}

			#body
			{
				$($body)*
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::Read, &mut $data, $ptr);
				$read.read(&mut $data[$ptr..=$ptr])?;
				#[cfg(feature = "debug")]
				$crate::on_op_post($crate::Op::Read, &mut $data, $ptr);
			}

			#token
			[{ $( $rest )* }]
		}
	};

	// While
	(
		#function
		{
			$inner_name:ident;
			$read:ident;
			$write:ident;
			$data:ident;
			$ptr:ident;
			raw = $raw_value:ident;
		}

		#body
		{$($body:tt)*}

		#token
		[{ [ $($inner:tt)* ] $($rest:tt)* }]
	) => {
		$crate::brainfuck_inner! {
			#function
			{
				$inner_name;
				$read;
				$write;
				$data;
				$ptr;
				raw = $raw_value;
			}

			#body
			{
				$($body)*

				let inner = $crate::brainfuck_raw! {
					$($inner)*
				};

				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::LoopStart, &mut $data, $ptr);
				while $data[$ptr] != 0 {
					#[cfg(feature = "debug")]
					$crate::on_op_pre($crate::Op::LoopIt, &mut $data, $ptr);
					$ptr = inner($ptr, &mut $data, $read, $write)?;
					#[cfg(feature = "debug")]
					$crate::on_op_post($crate::Op::LoopIt, &mut $data, $ptr);
				}
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::LoopEnd, &mut $data, $ptr);
			}

			#token
			[{ $( $rest )* }]
		}
	};

	// Special Tokens
	(
		#function
		{
			$inner_name:ident;
			$read:ident;
			$write:ident;
			$data:ident;
			$ptr:ident;
			raw = $raw_value:ident;
		}

		#body
		{$($body:tt)*}

		#token
		[{ <- $($rest:tt)* }]
	) => {
		$crate::brainfuck_inner! {
			#function
			{
				$inner_name;
				$read;
				$write;
				$data;
				$ptr;
				raw = $raw_value;
			}

			#body
			{
				$($body)*
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::Left, &mut $data, $ptr);
				$ptr = $ptr.wrapping_sub(1);
				#[cfg(feature = "debug")]
				$crate::on_op_post($crate::Op::Left, &mut $data, $ptr);
				#[cfg(feature = "debug")]
				$crate::on_op_pre($crate::Op::Minus, &mut $data, $ptr);
				$data[$ptr] = $data[$ptr].wrapping_sub(1);
				#[cfg(feature = "debug")]
				$crate::on_op_post($crate::Op::Minus, &mut $data, $ptr);
			}

			#token
			[{ $( $rest )* }]
		}
	};

	// Finish
	(
		#function
		{
			$inner_name:ident;
			$read:ident;
			$write:ident;
			$data:ident;
			$ptr:ident;
			raw = true;
		}

		#body
		{$($body:tt)*}

		#token
		[{}]
	) => {{
		const BUF_SIZE: usize = 30_000;
		fn $inner_name(mut $ptr: usize, mut $data: &mut [u8; BUF_SIZE], $read: &mut dyn ::std::io::Read, $write: &mut dyn ::std::io::Write) -> ::std::io::Result<usize> {
			$(
				$body
			)*

			Ok($ptr)
		}
		$inner_name
	}};
	(
		#function
		{
			$inner_name:ident;
			$read:ident;
			$write:ident;
			$data:ident;
			$ptr:ident;
			raw = false;
		}

		#body
		{$($body:tt)*}

		#token
		[{}]
	) => {{
		const BUF_SIZE: usize = 30_000;
		fn $inner_name(mut $ptr: usize, mut $data: [u8; BUF_SIZE], $read: &mut dyn ::std::io::Read, $write: &mut dyn ::std::io::Write) -> ::std::io::Result<[u8; BUF_SIZE]> {

			$(
				$body
			)*

			Ok($data)
		}
		|$read, $write| {
			let $data = [0u8; BUF_SIZE];
			$inner_name(0 ,$data, $read, $write)
		}
	}};
}

#[doc(hidden)]
#[cfg(feature = "debug")]
pub enum Op {
	Left,
	Right,
	Plus,
	Minus,
	Print,
	Read,
	LoopStart,
	LoopIt,
	LoopEnd,
}

#[doc(hidden)]
#[cfg(feature = "debug")]
impl Op {
	fn display(&self) -> &'static str {
		use Op::*;
		match self {
			Left => "Left",
			Right => "Right",
			Plus => "Plus",
			Minus => "Minus",
			Print => "Print",
			Read => "Read",
			LoopStart => "LoopStart",
			LoopIt => "LoopIt",
			LoopEnd => "LoopEnd",
		}
	}
}

#[doc(hidden)]
#[cfg(feature = "debug")]
pub fn on_op_pre(_op: Op, _data: &mut [u8; 30_000], _ptr: usize) {
}

#[doc(hidden)]
#[cfg(feature = "debug")]
pub fn on_op_post(op: Op, data: &mut [u8; 30_000], ptr: usize) {
	match op {
		Op::LoopStart | Op::LoopEnd => return,
		_ => (),
	}
	println!("{:<10}: {:?}[{}] => {}", op.display(), &data[0..10], ptr, data[ptr]);
}
