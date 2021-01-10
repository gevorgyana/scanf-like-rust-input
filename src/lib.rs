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
