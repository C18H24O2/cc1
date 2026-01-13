mod token;
pub use token::{Token, TokenKind, IntLitSuffix, FloatLitSuffix};

use crate::source_manager::{SourceManager, SourceReader};

pub struct Lexer<'src> {
	src: &'src SourceManager<'src>,
	src_reader: SourceReader<'src>,
	returned_eof_already: bool
}

macro_rules! c_ident_start_pat {
	() => {
		b'a' | b'b' | b'c' | b'd' | b'e' | b'f' | b'g' | b'h' | b'i' | b'j' | b'k' | b'l' |
		b'm' | b'n' | b'o' | b'p' | b'q' | b'r' | b's' | b't' | b'u' | b'v' | b'w' | b'x' |
		b'y' | b'z' | b'A' | b'B' | b'C' | b'D' | b'E' | b'F' | b'G' | b'H' | b'I' | b'J' |
		b'K' | b'L' | b'M' | b'N' | b'O' | b'P' | b'Q' | b'R' | b'S' | b'T' | b'U' | b'V' |
		b'W' | b'X' | b'Y' | b'Z' | b'_'
	}
}

macro_rules! c_integer_pat {
	() => {
		b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9'
	};
}

macro_rules! c_ident_pat {
	() => {
		c_ident_start_pat!() | c_integer_pat!()
	};
}

macro_rules! c_whitespace_pat {
	() => {
		//						 \v		   \f
		b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c'
	};
}

impl<'src> Iterator for Lexer<'src> {
	type Item = Token<'src>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		// the loop statement is essentially a hack to mimic a goto.
		// if we just want to skip some characters then lex again, call `continue`
		// instead of making a recursive call
		loop {
			let r = &mut self.src_reader;

			let location = r.get_source_location();
			let chr = if let Some(c) = r.get_char() {
				c
			}
			else if !self.returned_eof_already {
				self.returned_eof_already = true;
				return Some(Token::new(TokenKind::EOF, location));
			}
			else {
				return None;
			};

			let kind = match chr {
				c_whitespace_pat!() => { self.skip_whitespace(); continue; },

				b'#' => { r.advance(); TokenKind::Hash },

				b',' => { r.advance(); TokenKind::Comma },
				b'.' => match r.advance_and_get_char() {
					Some(b'.') => match r.advance_and_get_char() {
						Some(b'.') => TokenKind::DotDotDot,
						_ => todo!("`..` is not a valid token")
					},
					_ => TokenKind::Dot
				},
				b';' => { r.advance(); TokenKind::Semicolon },
				b':' => { r.advance(); TokenKind::Colon },

				b'(' => { r.advance(); TokenKind::LParens },
				b')' => { r.advance(); TokenKind::RParens },
				b'[' => { r.advance(); TokenKind::LBracket },
				b']' => { r.advance(); TokenKind::RBracket },
				b'{' => { r.advance(); TokenKind::LCurly },
				b'}' => { r.advance(); TokenKind::RCurly },

				b'+' => match r.advance_and_get_char() {
					Some(b'+') => { r.advance(); TokenKind::PlusPlus },
					Some(b'=') => { r.advance(); TokenKind::PlusEq },
					_ => TokenKind::Plus
				},
				
				b'-' => match r.advance_and_get_char() {
					Some(b'>') => { r.advance(); TokenKind::Arrow },
					Some(b'-') => { r.advance(); TokenKind::MinusMinus },
					Some(b'=') => { r.advance(); TokenKind::MinusEq },
					_ => TokenKind::Minus
				},

				b'*' => match r.advance_and_get_char() {
					Some(b'=') => { r.advance(); TokenKind::AsteriskEq },
					_ => TokenKind::Asterisk
				},

				b'/' => match r.advance_and_get_char() {
					Some(b'*') => { r.advance(); self.skip_comment(); continue; },
					Some(b'=') => { r.advance(); TokenKind::SlashEq },
					_ => TokenKind::Slash
				},

				b'=' => match r.advance_and_get_char() {
					Some(b'=') => { r.advance(); TokenKind::EqEq },
					_ => TokenKind::Eq
				},

				b'!' => match r.advance_and_get_char() {
					Some(b'=') => { r.advance(); TokenKind::NotEq },
					_ => TokenKind::Not
				},

				b'>' => match r.advance_and_get_char() {
					Some(b'=') => { r.advance(); TokenKind::GtEq },
					_ => TokenKind::Gt
				},

				b'<' => match r.advance_and_get_char() {
					Some(b'=') => { r.advance(); TokenKind::LtEq },
					_ => TokenKind::Lt
				},
				
				c_ident_start_pat!() => self.lex_identifier(),
				c_integer_pat!() => self.lex_number(),
				b'\'' | b'"' => self.lex_string(),
				_ => panic!("Unhandled character {:?}", chr as char)
			};
			return Some(Token::new(kind, location))
		}
	}
}

impl<'src> Lexer<'src> {
	pub fn from(src: &'src SourceManager) -> Lexer<'src> {
		Lexer {
			src,
			src_reader: src.get_source_reader(),
			returned_eof_already: false
		}
	}

	/// Advances the reader's cursor until `f` returns `false` for the current character
	/// or the end of the file.
	/// Returns a slice representing all the characters that were skipped,
	/// as well as whether or not the end of file was reached
	#[inline]
	fn fetch_source_while<F>(&mut self, mut f: F) -> (&'src str, bool)
	where
		F: FnMut(u8) -> bool
	{
		let r = &mut self.src_reader;
		let start = r.cursor_ptr();

		let end_of_file = loop {
			if let Some(c) = r.advance_and_get_char() {
				if !f(c) {
					break false;
				}
			}
			else {
				break true;
			}
		};

		let end = r.cursor_ptr();

		let slice = unsafe {
			// SAFETY: end > star
			let len = end.sub(start as usize) as usize;
			// SAFETY: guaranteed by the constraints of SourceReader
			std::slice::from_raw_parts(start, len)
		};
		// TODO: slice should always be valid utf-8, use unchecked version
		(std::str::from_utf8(slice).unwrap(), end_of_file)
	}

	fn lex_identifier(&mut self) -> TokenKind<'src> {
		let (slice, _) = self.fetch_source_while(|c| matches!(c, c_ident_pat!()));
		// TODO: handle UTF-8
		TokenKind::Literal(slice)
	}

	fn lex_number(&mut self) -> TokenKind<'src> {
		let r = &mut self.src_reader;

		let mut repr: u64 = 0;

		while let Some(c) = r.get_char() && matches!(c, c_integer_pat!()) {
			let digit = c - b'0';
			// TODO: handle overflow
			repr = repr * 10 + digit as u64;
			r.advance();
		}

		if let Some(c) = r.get_char() {
			match c {
				b'.' => todo!("Floating point repr"),
				c_ident_start_pat!() => todo!("Integer Suffix"),
				_ => (),
			}
		}
		TokenKind::IntLit { value: repr, suffix: IntLitSuffix::None }
	}

	fn lex_string(&mut self) -> TokenKind<'src> {
		let r = &mut self.src_reader;
		// SAFETY: this function is private and only called when a string start has already been matched
		let string_type = unsafe { r.get_char_unchecked() };
		r.advance();

		let mut is_escaping = false;

		// TODO: handle trigraphs
		let (slice, end_of_file) = self.fetch_source_while(|c| {
			if c == string_type {
				if !is_escaping {
					return false;
				}
				is_escaping = false;
			}
			else {
				is_escaping = c == b'\\' && !is_escaping;
			}
			true
		});
		if end_of_file {
			todo!("Unterminated string");
		}
		// consume the end quote character
		// SAFETY: there is at least a quote character
		unsafe { self.src_reader.advance_unchecked(); }

		if string_type == b'"' {
			TokenKind::StringLit(slice)
		}
		else {
			TokenKind::CharLit(slice)
		}
	}

	#[inline]
	fn skip_whitespace(&mut self) {
		let r = &mut self.src_reader;
		while let Some(c) = r.advance_and_get_char() && matches!(c, c_whitespace_pat!()) { }
	}

	fn skip_comment(&mut self) {
		let r = &mut self.src_reader;
		let mut prev_is_star = false;
		while let Some(c) = r.get_char_and_advance() {
			if c == b'/' {
				if prev_is_star {
					return;
				}
				prev_is_star = false;
			}
			else {
				prev_is_star = c == b'*';
			}
		}
		todo!("Unterminated C89 comment");
	}
}
