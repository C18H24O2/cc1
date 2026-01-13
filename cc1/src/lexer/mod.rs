mod token;
pub use token::{Token, TokenKind, IntLitSuffix, FloatLitSuffix};

use crate::source_manager::{SourceManager, SourceIterator};

pub struct Lexer<'src> {
	src: &'src SourceManager<'src>,
	src_iter: SourceIterator<'src>,
	returned_eof_already: bool
}

impl<'src> Lexer<'src> {
	pub fn from(src: &'src SourceManager) -> Lexer<'src> {
		Lexer {
			src,
			src_iter: src.get_source_iterator(),
			returned_eof_already: false
		}
	}
}

impl<'src> Iterator for Lexer<'src> {
	type Item = Token<'src>;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		let location = self.src_iter.get_source_location();
		let word = if let Some(s) = self.src_iter.next() {
			s
		}
		else {
			if !self.returned_eof_already {
				self.returned_eof_already = true;
				return Some(Token::new(TokenKind::EOF, location));
			}
			return None;
		};
		println!("{:?}", word);
		Some(Token::new(TokenKind::Literal(word), location))
	}
}
