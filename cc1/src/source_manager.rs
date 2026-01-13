use std::collections::HashSet;

use crate::{is_sep, is_quote};

pub struct SourceManager<'src> {
	/// The raw source
	src: &'src str,
	files: HashSet<FileRef<'src>>,
}

impl<'src> SourceManager<'src> {
	#[inline]
	pub fn from(src: &'src str, top_file_name: &'src str) -> SourceManager<'src> {
		SourceManager {
			src,
			files: HashSet::from([FileRef(top_file_name)])
		}
	}

	#[inline]
	pub fn get_source_iterator(&self) -> SourceIterator<'src> {
		SourceIterator {
			src_bytes: self.src.as_bytes(),
			idx: 0
		}
	}

	#[inline]
	pub fn src(&self) -> &'src str {
		self.src
	}
}

pub struct SourceIterator<'src> {
	src_bytes: &'src [u8],
	idx: usize,
}

impl<'src> std::iter::Iterator for SourceIterator<'src> {
	type Item = &'src str;

	#[inline]
	fn next(&mut self) -> Option<Self::Item> {
		let start = self.idx;
		let mut i = 0usize;
		while let Some(&c) = self.src_bytes.get(start + i) {
			if is_sep(c) {
				let end = if i == 0 {
					if !is_quote(c) {
						// fetch only the separator
						start + 1
					}
					else {
						let mut j = i + 1;
						let mut is_escape = false;
						loop {
							if let Some(&c_in_quote) = self.src_bytes.get(start + j) {
								if c_in_quote == c && !is_escape {
									// fetch the whole sequence in quotes, including the quotes.
									// note: since this functions returns a slice into an already existing
									// string, the escape character(s) will still be present
									break start + j + 1;
								}
								is_escape = c_in_quote == b'\\' && !is_escape;
								j += 1;
							}
							else {
								todo!("Unterminated quote");
							}
						}
					}
				}
				else {
					// fetch the whole sequence preceeding the separator
					start + i
				};
				self.idx = end;
				// TODO: we should be able to do an unchecked fetch here
				// let result = unsafe { self.src_bytes.get_unchecked(start..end) };
				let result = &self.src_bytes[start..end];
				// pretty sure this could be unchecked because the initial string is valid
				// as long as we don't cut in the middle of a unicode scalar
				return Some(str::from_utf8(result).unwrap());
			}
			i += 1;
		}
		None
	}
}

impl<'src> SourceIterator<'src> {
	#[inline]
	pub fn get_source_location(&self) -> SourceLocation {
		SourceLocation(self.idx)
	}
}

#[derive(Hash, PartialEq, Eq)]
struct FileRef<'src>(&'src str);

/// Opaque structure representing a location in the original source.
/// (TODO) SourceManager is able to transcribe it into its (possibly included)
/// source file, line number and column.
#[derive(Debug, Clone, Copy)]
pub struct SourceLocation(usize);
