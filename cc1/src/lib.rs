pub mod lexer;
pub mod source_manager;

#[inline]
fn is_sep(chr: u8) -> bool {
	matches!(chr,
		b'\t' | b'\n' | b' ' |
		b'+' | b'-' | b'*' | b'/' |
		b'<' | b'>' | b'=' |
		b'\'' | b'"' |
		b'(' | b')' | b'{' | b'}' | b'[' | b']' |
		b'#'
	)
}

#[inline]
fn is_whitespace(chr: u8) -> bool {
	matches!(chr, b'\t' | b'\n' | b' ')
}

#[inline]
fn is_quote(chr: u8) -> bool {
	matches!(chr, b'\'' | b'"')
}
