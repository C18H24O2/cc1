mod token;
pub use token::{Token, TokenKind, IntLitSuffix, FloatLitSuffix};

use crate::source_manager::{SourceManager, SourceReader};

pub struct Lexer<'src> {
	src: &'src SourceManager<'src>,
	src_reader: SourceReader<'src>,
	at_line_start: bool,
	in_directive: bool,
	returned_eof_already: bool,
}

macro_rules! c_ident_start_pat {
	() => {
		b'a'..=b'z' | b'A'..=b'Z' | b'_'
	}
}

macro_rules! c_integer_pat {
	() => {
		b'0'..=b'9'
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
				c_whitespace_pat!() => {
					self.skip_whitespace();
					if !self.at_line_start || !self.in_directive {
						continue;
					}
					self.in_directive = false;
					TokenKind::EOD
				},

				b'#' => {
					r.advance();
					self.parse_directive();
					continue;
				},

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
				b'?' => { r.advance(); TokenKind::Question },

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
			if kind != TokenKind::EOD {
				self.at_line_start = false;
			}
			return Some(Token::new(kind, location))
		}
	}
}

impl<'src> Lexer<'src> {
	pub fn from(src: &'src SourceManager) -> Lexer<'src> {
		Lexer {
			src,
			src_reader: src.get_source_reader(),
			in_directive: false,
			at_line_start: true,
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

	/// Skips whitespaces in the stream and toggles `at_line_start` if a
	/// new-line was skipped
	fn skip_whitespace(&mut self) {
		let r = &mut self.src_reader;
		while let Some(c) = r.get_char() {
			match c {
				b'\n' => self.at_line_start = true,
				b' ' | b'\t' | b'\x0b' | b'\x0c' => (),
				_ => break
			}
			r.advance();
		}
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

	fn skip_to_next_line(&mut self) {
		let r = &mut self.src_reader;

		while let Some(c) = r.get_char() {
			if c == b'\n' {
				// might as well skip all the remaining whitespaces while we're at it
				while let Some(c) = r.advance_and_get_char() && matches!(c, c_whitespace_pat!()) { }
				self.at_line_start = true;
				return;
			}
			r.advance();
		}
	}

	/// Parses a directive starting with #
	/// note: this is NOT a preprocessor logic.
	/// Therefore, it will only parse things like linemarkers or pragmas,
	/// and error on the rest.
	fn parse_directive(&mut self) {
		if !self.at_line_start {
			todo!("# Directive not on line start");
		}
		self.at_line_start = false;
		self.in_directive = true;

		// SAFETY: there should at least be the EOD token
		let next_tok = unsafe { self.next().unwrap_unchecked() };
		match next_tok.kind() {
			TokenKind::EOD => (), // empty directive is not an error
			TokenKind::Literal(lit) => self.parse_ident_directive(lit),
			TokenKind::IntLit { value, suffix: IntLitSuffix::None } => {
				let line_number: u32 = if let Ok(x) = value.try_into() { x } else {
					todo!("Line constant too big");
				};
				self.parse_linemarker(line_number);
			}
			_ => todo!("Unexpected token while parsing directive: {:?}", next_tok.kind())
		}
	}

	fn parse_ident_directive(&mut self, ident: &str) {
		match ident {
			"pragma" => self.skip_to_next_line(),
			_ => todo!("Unknown PP directive `{}`", ident)
		}
	}

	/// https://gcc.gnu.org/onlinedocs/gcc-11.1.0/cpp/Preprocessor-Output.html
	fn parse_linemarker(&mut self, line_number: u32) {
		// SAFETY: there should at least be the EOD token
		let next_tok = unsafe { self.next().unwrap_unchecked() };
		let file_name = match next_tok.kind() {
			TokenKind::StringLit(f) => f,
			TokenKind::EOD => { _mark_current_file(line_number); return; },
			_ => todo!("Unexpeced token {:?}", next_tok.kind())
		};

		let mut try_parse_flag = || {
			// SAFETY: there should at least be the EOD token
			let next_tok = unsafe { self.next().unwrap_unchecked() };
			match next_tok.kind() {
				TokenKind::EOD => None,
				TokenKind::IntLit { value, suffix: IntLitSuffix::None } => {
					if value > 4 {
						todo!("Linemarker flag too big");
					}
					Some(value as u8)
				},
				_ => todo!("Expected linemarker flag, got {:?}", next_tok.kind())
			}
		};

		match try_parse_flag() {
			Some(flag @ (1 | 2)) => {
				let flag = if flag == 1 { MarkLineKind::StartNewFile } else { MarkLineKind::ReturningToFile };
				_mark_line(line_number, file_name, flag);
				self.skip_to_next_line();
			},
			// we don't care about 3 and 4
			Some(_) => {
				_mark_line(line_number, file_name, MarkLineKind::None);
				self.skip_to_next_line();
			},
			None => _mark_line(line_number, file_name, MarkLineKind::None)
		}
	}
}

/// TODO: MOVE IN SOURCE_MANAGER LATER. THIS IS JUST A SKELETON.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MarkLineKind {
	None,
	StartNewFile,
	ReturningToFile,
}

/// TODO: MOVE IN SOURCE_MANAGER LATER. THIS IS JUST A SKELETON.
fn _mark_current_file(_line_number: u32) {
	println!("CURRENT_FILE:{} MARKED", _line_number);
}

/// TODO: MOVE IN SOURCE_MANAGER LATER. THIS IS JUST A SKELETON.
fn _mark_line(_line_number: u32, _file_name: &str, _kind: MarkLineKind) {
	println!("{}:{} MARKED, {:?}", _file_name, _line_number, _kind);
}
