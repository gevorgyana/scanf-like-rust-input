use std::io::{self, Read, Write};

/// Reads a sample input into a buffer, separating the the lines from each other.
/// On Unix, one needs to enter C-D to mark the end of input.
///
/// # Examples
///
/// ```
/// use concise_scanf_like_input::read_input_lines;
/// use std::io::{self, Read, Write};
/// // you need to make the buffer mutable to allow the macro to fill it
/// let mut buf = String::new();
/// // the lines must be mutable to rewind themselves on every new line read
/// let mut lines: &[&str] = read_input_lines!(buf);
/// ```
#[macro_export]
macro_rules! read_input_lines {
    ($buf:ident) => {
	{
	    let mut stdin = io::stdin();
	    stdin.read_to_string(&mut $buf);
	    &$buf.lines().collect::<Vec<_>>()
	}
    }
}


/// Reads a sample line from the previously fetched vector of lines, then parses a
/// heterogeneous sequence of variables and stores them in previously allocated
/// memory.
///
/// # Examples
///
/// ```
/// use concise_scanf_like_input::read_line_of_vars;
/// let mut lines: &[&str] = &vec!["1", "2 3"];
/// // you don't need to make a variable mutable
/// let t: i32;
/// read_line_of_vars!(i32, lines, t);
/// for _ in 0..t {
///	let (n, x): (i32, i32);
///	read_line_of_vars!(i32, lines, n, x);
/// }
/// ```
#[macro_export]
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
