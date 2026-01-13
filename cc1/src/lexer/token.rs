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

	/// The literal `,`
	Comma,
	/// The literal `.`
	Dot,
	/// The literal `...`
	DotDotDot,
	/// The literal `;`
	Semicolon,
	/// The literal `:`
	Colon,

	Literal(&'a str),
	/// A string literal wrapped in double quotes (`"`)
	StringLit(&'a str),
	/// A character literal wrapped in single quotes (`'`)
	CharLit(&'a str),

	/// One of multiple whitespace characters. The representation
	/// is kept because we may have to output it (e.g the -E flag)
	Whitespace(&'a str),

	/// A literal representing either a signed or unsigned integer with variable size
	IntLit { value: u64, signed: bool, suffix: IntLitSuffix },
	/// A literal representing either a float or double or long double
	FloatLit { value: f64, suffix: FloatLitSuffix },

	/// The literal `#`
	Hash,

	/// The literal `=`
	Eq,
	/// The literal `==`
	EqEq,
	/// The literal `!`
	Not,
	/// The literal `!=`
	NotEq,
	/// The literal `>`
	Gt,
	/// The literal `>=`
	GtEq,
	/// The literal `<`
	Lt,
	/// The literal `<=`
	LtEq,

	/// The literal `+`
	Plus,
	/// The literal `+=`
	PlusEq,
	/// The literal `++`
	PlusPlus,
	/// The literal `-`
	Minus,
	/// The literal `-=`
	MinusEq,
	/// The literal `--`
	MinusMinus,
	/// The literal `*`
	Asterisk,
	/// The literal `*=`
	AsteriskEq,
	/// The literal `/`
	Slash,
	/// The literal `/=`
	SlashEq,

	/// The literal `->`
	Arrow,

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
