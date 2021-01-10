use std::io::{self, Read, Write};

macro_rules! read_input_lines {
    ($buf:ident) => {
	{
	    let mut stdin = io::stdin();
	    stdin.read_to_string(&mut $buf);
	    &$buf.lines().collect::<Vec<_>>()
	}
    }
}

macro_rules! read_line_of_vars {
    ($type:ty, $lines:ident, $($var:ident),+) => {
	{
	    let (line, rest) = $lines.split_first().unwrap();
	    $lines = rest;
	    let parsed = line.trim().split_whitespace().map(
		|x| {
		    x.parse::<$type>().unwrap()
		}
	    )
		.collect::<Vec<$type>>();
	    let parsed = &parsed[..];
	    $(
		match parsed.split_first() {
		    None => {
			$var = parsed[0];
		    },
		    Some((value, parsed)) => {
			$var = *value;
		    }
		}
	    );+;
	    {(
		$(
		    $var
		),+
	    )}
	}
    }
}

// call this crate scanf-like rust macros

fn main() {
    // you need to make the buffer mutable to allow the macro to fill it
    let mut buf = String::new();
    // the lines must be mutable to rewind themselves on every new line read
    let mut lines: &[&str] = read_input_lines!(buf);

    // you don't need to make a variable mutable
    let t: i32;
    read_line_of_vars!(i32, lines, t);

    for _ in 0..t {
	let (n, x): (i32, i32);
	read_line_of_vars!(i32, lines, n, x);
    }
}
