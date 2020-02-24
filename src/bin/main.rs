//#![recursion_limit="8"]
#![feature(trace_macros)]
#![feature(slice_patterns)]

#[macro_export]
macro_rules! brainfuck {
	(
		$($tt:tt)*
	) => {
		brainfuck_inner! {
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

#[macro_export]
macro_rules! brainfuck_raw {
	(
		$($tt:tt)*
	) => {
		brainfuck_inner! {
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
		brainfuck_inner! {
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
				on_op_pre(Op::Right, &mut $data, $ptr);
				$ptr = $ptr.wrapping_add(1);
				on_op_post(Op::Right, &mut $data, $ptr);
				on_op_pre(Op::Right, &mut $data, $ptr);
				$ptr = $ptr.wrapping_add(1);
				on_op_post(Op::Right, &mut $data, $ptr);
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
		brainfuck_inner! {
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
				on_op_pre(Op::Right, &mut $data, $ptr);
				$ptr = $ptr.wrapping_add(1);
				on_op_post(Op::Right, &mut $data, $ptr);
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
		brainfuck_inner! {
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
				on_op_pre(Op::Left, &mut $data, $ptr);
				$ptr = $ptr.wrapping_sub(1);
				on_op_post(Op::Left, &mut $data, $ptr);
				on_op_pre(Op::Left, &mut $data, $ptr);
				$ptr = $ptr.wrapping_sub(1);
				on_op_post(Op::Left, &mut $data, $ptr);
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
		brainfuck_inner! {
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
				on_op_pre(Op::Left, &mut $data, $ptr);
				$ptr = $ptr.wrapping_sub(1);
				on_op_post(Op::Left, &mut $data, $ptr);
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
		brainfuck_inner! {
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
				on_op_pre(Op::Plus, &mut $data, $ptr);
				$data[$ptr] = $data[$ptr].wrapping_add(1);
				on_op_post(Op::Plus, &mut $data, $ptr);
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
		brainfuck_inner! {
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
				on_op_pre(Op::Minus, &mut $data, $ptr);
				$data[$ptr] = $data[$ptr].wrapping_sub(1);
				on_op_post(Op::Minus, &mut $data, $ptr);
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
		brainfuck_inner! {
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
				on_op_pre(Op::Print, &mut $data, $ptr);
				$write.write(&$data[$ptr..=$ptr])?;
				on_op_post(Op::Print, &mut $data, $ptr);
				on_op_pre(Op::Print, &mut $data, $ptr);
				$write.write(&$data[$ptr..=$ptr])?;
				on_op_post(Op::Print, &mut $data, $ptr);
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
		brainfuck_inner! {
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
				on_op_pre(Op::Print, &mut $data, $ptr);
				$write.write(&$data[$ptr..=$ptr])?;
				on_op_post(Op::Print, &mut $data, $ptr);
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
		brainfuck_inner! {
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
				on_op_pre(Op::Read, &mut $data, $ptr);
				$read.read(&mut $data[$ptr..=$ptr])?;
				on_op_post(Op::Read, &mut $data, $ptr);
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
		brainfuck_inner! {
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

				let inner = brainfuck_raw! {
					$($inner)*
				};

				on_op_pre(Op::LoopStart, &mut $data, $ptr);
				while $data[$ptr] != 0 {
					on_op_pre(Op::LoopIt, &mut $data, $ptr);
					$ptr = inner($ptr, &mut $data, $read, $write)?;
					on_op_post(Op::LoopIt, &mut $data, $ptr);
				}
				on_op_pre(Op::LoopEnd, &mut $data, $ptr);
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
		brainfuck_inner! {
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
				on_op_pre(Op::Left, &mut $data, $ptr);
				$ptr = $ptr.wrapping_sub(1);
				on_op_post(Op::Left, &mut $data, $ptr);
				on_op_pre(Op::Minus, &mut $data, $ptr);
				$data[$ptr] = $data[$ptr].wrapping_sub(1);
				on_op_post(Op::Minus, &mut $data, $ptr);
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


fn on_op_pre(op: Op, data: &mut [u8; 30_000], ptr: usize) {
//	match data {
//		[8, 0, 9, 13, 11, 4, 1, ..] => panic!("End"),
//		_ => (),
//	}
}

fn on_op_post(op: Op, data: &mut [u8; 30_000], ptr: usize) {
//	println!("PST: {} => {:?}", ptr, &data[0..10]);
	match op {
		Op::LoopStart | Op::LoopEnd => return,
		_ => (),
	}
//	println!("{:<10}: {:?}[{}] => {}", op.display(), &data[0..10], ptr, data[ptr]);
}

fn main() {
	let code = brainfuck! {
//		[ This program prints "Hello World!" and a newline to the screen, its
//		  length is 106 active command characters. [It is not the shortest.]
//
//		  This loop is an "initial comment loop", a simple way of adding a comment
//		  to a BF program such that you don't have to worry about any command
//		  characters. Any ".", ",", "+", "-", "<" and ">" characters are simply
//		  ignored, the "[" and "]" characters just have to be balanced. This
//		  loop and the commands it contains are ignored because the current cell
//		  defaults to a value of 0; the 0 value causes this loop to be skipped.
//		]
		++++++++                // Set Cell #0 to 8
		[
			>++++               // Add 4 to Cell #1; this will always set Cell #1 to 4
			[                   // as the cell will be cleared by the loop
				>++             // Add 2 to Cell #2
				>+++            // Add 3 to Cell #3
				>+++            // Add 3 to Cell #4
				>+              // Add 1 to Cell #5
				<<<<-           // Decrement the loop counter in Cell #1
			]                   // Loop till Cell #1 is zero; number of iterations is 4
			>+                  // Add 1 to Cell #2
			>+                  // Add 1 to Cell #3
			>-                  // Subtract 1 from Cell #4
			>>+                 // Add 1 to Cell #6
			[<]                 // Move back to the first zero cell you find; this will
								// be Cell #1 which was cleared by the previous loop
			<-                  // Decrement the loop Counter in Cell #0
		]                       // Loop till Cell #0 is zero; number of iterations is 8

		// The result of this is:
		// Cell No :   0   1   2   3   4   5   6
		// Contents:   0   0  72 104  88  32   8
		// Pointer :   ^

		>>.                     // Cell #2 has value 72 which is 'H'
		>---.                   // Subtract 3 from Cell #3 to get 101 which is 'e'
		+++++++..+++.           // Likewise for 'llo' from Cell #3
		>>.                     // Cell #5 is 32 for the space
		<-.                     // Subtract 1 from Cell #4 for 87 to give a 'W'
		<.                      // Cell #3 was set to 'o' from the end of 'Hello'
		+++.------.--------.    // Cell #3 for 'rl' and 'd'
		>>+.                    // Add 1 to Cell #5 gives us an exclamation point
		>++.                    // And finally a newline from Cell #6
	};

//	use std::io::Write;
//	use std::io::Read;
//	let mut buf = [0];
//	std::io::stdin().lock().read(&mut buf[0..=0]).unwrap();
//	println!("{:?}", buf);
//
////	{
//		let stdout = std::io::stdout();
//		let mut t = stdout.lock();
//		t.write(&buf[0..=0]).unwrap();
////		t.flush().unwrap();
////	}

	code(&mut std::io::stdin().lock(), &mut std::io::stdout().lock()).unwrap();


	// '>' => $ptr += 1;
	// '<' => $ptr -= 1;
	// '+' => $data[$ptr] += 1;
	// '-' => $data[$ptr] -= 1;
	// '.' => $w.write(&$data[$ptr..$ptr])?;
	// ',' => $r.read$exact(&mut $data[$ptr..$ptr])?;
	// '[' => while $data[$ptr] != 0 {
	// ']' => }

//	fn evaluate(mut _r: impl ::std::io::Read, mut _w: impl ::std::io::Write) -> ::std::io::Result<[u8; 30_000]> {
//		let mut _data = [0u8; 30_000];
//		let _ptr = 0;
//
//
//		Ok(_data)
//	}
}


