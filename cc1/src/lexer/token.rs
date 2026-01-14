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

impl<'a> AsRef<SourceLocation> for Token<'a> {
	#[inline]
	fn as_ref(&self) -> &SourceLocation {
		&self.location
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(non_camel_case_types)]
pub enum TokenKind<'a> {
	Invalid,
	/// The end of the source stream
	EOF,
	/// Emitted when finding a new-line while in a directive
	EOD,

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
	/// The literal `?`
	Question,

	Literal(&'a str),
	/// A string literal wrapped in double quotes (`"`)
	StringLit(&'a str),
	/// A character literal wrapped in single quotes (`'`)
	CharLit(&'a str),

	/// A literal representing either a signed or unsigned integer with variable size
	IntLit { value: u64, suffix: IntLitSuffix },
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
	RBracket,

	// All the C89 keywords (3.1.1 Keywords)
	K_auto, K_double, K_int, K_struct,
	K_break, K_else, K_long, K_switch,
	K_case, K_enum, K_register, K_typedef,
	K_char, K_extern, K_return, K_union,
	K_const, K_float, K_short, K_unsigned,
	K_continue, K_for, K_signed, K_void,
	K_default, K_goto, K_sizeof, K_volatile,
	K_do, K_if, K_static, K_while,
}

impl<'a> TokenKind<'a> {
	pub fn map_keyword_or_return_literal(ident: &'a str) -> TokenKind<'a> {
		match ident {
			"auto" => TokenKind::K_auto,
			"double" => TokenKind::K_double,
			"int" => TokenKind::K_int,
			"struct" => TokenKind::K_struct,
			"break" => TokenKind::K_break,
			"else" => TokenKind::K_else,
			"long" => TokenKind::K_long,
			"switch" => TokenKind::K_switch,
			"case" => TokenKind::K_case,
			"enum" => TokenKind::K_enum,
			"register" => TokenKind::K_register,
			"typedef" => TokenKind::K_typedef,
			"char" => TokenKind::K_char,
			"extern" => TokenKind::K_extern,
			"return" => TokenKind::K_return,
			"union" => TokenKind::K_union,
			"const" => TokenKind::K_const,
			"float" => TokenKind::K_float,
			"short" => TokenKind::K_short,
			"unsigned" => TokenKind::K_unsigned,
			"continue" => TokenKind::K_continue,
			"for" => TokenKind::K_for,
			"signed" => TokenKind::K_signed,
			"void" => TokenKind::K_void,
			"default" => TokenKind::K_default,
			"goto" => TokenKind::K_goto,
			"sizeof" => TokenKind::K_sizeof,
			"volatile" => TokenKind::K_volatile,
			"do" => TokenKind::K_do,
			"if" => TokenKind::K_if,
			"static" => TokenKind::K_static,
			"while" => TokenKind::K_while,
			_ => TokenKind::Literal(ident)
		}
	}
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
