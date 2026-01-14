pub enum DiagKind {
	Message,
	Warning,
	Error
}

macro_rules! define_diags {
	($(
		($name:ident $( { $($arg:ident : $arg_type:ty),+ } )? , $kind:ident, $msg_fmt:literal)
	),* $(,)?) => {
		#[derive(Debug, Clone, Copy)]
		#[allow(non_camel_case_types)]
		pub enum Diag<'src> {
			$( $name $( { $($arg: $arg_type, )* } )? , )*
		}

		impl<'src> std::fmt::Display for Diag<'src> {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				match &self {
					$(self::Diag::$name $( { $($arg, )* } )? => write!(f, $msg_fmt $( $(, $arg)* )? ),)*
				}
			}
		}

		impl<'src> Diag<'src> {
			pub fn get_human_readable_string(&self) -> String {
				match &self {
					$(self::Diag::$name $( { $($arg, )* } )? => format!($msg_fmt $( $(, $arg)* )? ),)*
				}
			}

			pub fn get_kind(&self) -> DiagKind {
				match &self {
					$(self::Diag::$name $( { $( $arg: _, )* } )? => self::DiagKind::$kind,)*
				}
			}
		}
	};
}

use crate::lexer::TokenKind;

define_diags!(
	(err_too_many_errors, Error, "Too many errors emmited, stopping now."),

	// Tokenizer errors
	(err_unhandled_character, Error, "Unhandled character"),
	(err_dotdot_not_valid, Error, "`..` is not a valid token"),
	(err_unterminated_string, Error, "Unterminated string"),
	(err_unterminated_comment, Error, "Unterminated comment"),
	(err_directive_not_on_line_start, Error, "`#` not at start of line"),
	(err_line_marker_too_big, Error, "Line constant too big"),
	(err_unexpected_token_in_directive{tok: TokenKind<'src>}, Error, "Unexpected token while parsing directive: {:?}"),
	(err_unknown_directive{ident: &'src str}, Error, "Unknown directive `{}`"),
	(err_unexpected_token_in_linemarker{tok: TokenKind<'src>}, Error, "Unexpected token while parsing linemarker: {:?}"),
	(err_linemarker_flag_too_big, Error, "Linemarker flag too big"),

	// ...
);

