/*
	This file will parse from the HTML files
*/

struct Parser {
    pos: usize, // "usize" is an unsigned integer, similar to "size_t" in C
    input: String,
}

impl Parser {
    // Read the current character without consuming it.
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    // Do the next characters start with the given string?
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos ..].starts_with(s)
    }

    // Return true if all input is consumed.
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // Return the current character, and advance self.pos to the next character.
	fn consume_char(&mut self) -> char {
		let mut iter = self.input[self.pos..].char_indices();
		let (_, cur_char) = iter.next().unwrap();
		let (next_pos, _) = iter.next().unwrap_or((1, ' '));
		self.pos += next_pos;
		return cur_char;
	}
}