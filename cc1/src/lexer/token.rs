use crate::source_manager::SourceLocation;

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
	kind: TokenKind<'a>,
	location: SourceLocation,
}

impl<'a> Token<'a> {
	#[inline]
	pub fn new(kind: TokenKind<'a>, location: SourceLocation) -> Token<'a> {
		Token {
			kind,
			location
		}
	}

	#[inline]
	pub fn kind(&self) -> TokenKind<'a> {
		self.kind
	}

	#[inline]
	pub fn location(&self) -> SourceLocation {
		self.location
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind<'a> {
	Invalid,
	/// The end of the source stream
	EOF,

	Literal(&'a str),
	/// A literal wrapped in double quotes (`"`)
	StringLit(&'a str),
	/// A single-character literal wrapped in single quotes (`'`)
	CharLit(char),

	/// A literal representing either a signed or unsigned integer with variable size
	IntLit { value: u64, signed: bool, suffix: IntLitSuffix },
	/// A literal representing either a float or double or long double
	FloatLit { value: f64, suffix: FloatLitSuffix },

	/// The literal `#`
	Hash,

	/// The literal `=`
	Eq,
	/// The literal `+`
	Plus,
	/// The literal `-`
	Minus,
	/// The literal `*`
	Asterisk,
	/// The literal `/`
	Slash,

	/// The literal `(`
	LParens,
	/// The literal `)`
	RParens,
	/// The literal `{`
	LCurly,
	/// The literal `}`
	RCurly,
	/// The literal `[`
	LBracket,
	/// The literal `]`
	RBracket
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntLitSuffix {
	None,
	Long,
	Unsigned,
	UnsignedLong
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatLitSuffix {
	None,
	Float,
	Long
}
