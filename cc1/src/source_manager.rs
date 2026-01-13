use std::collections::HashSet;

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
	pub fn get_source_reader(&self) -> SourceReader<'src> {
		SourceReader::from(self.src.as_bytes())
	}

	#[inline]
	pub fn src(&self) -> &'src str {
		self.src
	}
}

pub struct SourceReader<'src> {
	cursor: *const u8,
	remaining: usize,
	origin: *const u8,

	// this phantom asserts the `cursor` and `origin` pointers will only remain valid
	// as long as 'src is valid
	phantom: std::marker::PhantomData<&'src [u8]>,
}

impl<'src> SourceReader<'src> {
	#[inline]
	fn from(src_bytes: &'src [u8]) -> SourceReader<'src> {
		SourceReader {
			cursor: src_bytes.as_ptr(),
			phantom: std::marker::PhantomData,
			remaining: src_bytes.len(),
			origin: src_bytes.as_ptr()
		}
	}

	#[inline]
	pub fn remaining(&self) -> usize {
		self.remaining
	}

	#[inline]
	pub fn get_source_location(&self) -> SourceLocation {
		// SAFETY: cursor will always be >= origin
		let loc = unsafe { self.cursor.sub(self.origin as usize) } as usize;
		SourceLocation(loc)
	}

	#[inline]
	pub fn cursor_ptr(&self) -> *const u8 {
		self.cursor
	}

	#[inline]
	fn get_char_at_offset(&self, offs: usize) -> Option<u8> {
		if self.remaining > offs {
			// SAFETY: dereference is ensured to be inbounds because of the above check
			unsafe { Some(*(self.cursor.add(offs))) }
		}
		else {
			None
		}
	}

	#[inline]
	pub fn advance(&mut self) {
		if self.remaining > 0 {
			// SAFETY: we are guaranteed to remain inbounds because of the above check
			unsafe {
				self.cursor = self.cursor.add(1);
				self.remaining = self.remaining.unchecked_sub(1);
			}
		}
	}

	#[inline]
	pub unsafe fn advance_unchecked(&mut self) {
		// SAFETY: upheld by caller
		unsafe {
			self.cursor = self.cursor.add(1);
			self.remaining = self.remaining.unchecked_sub(1);
		}
	}

	#[inline]
	pub fn get_char(&self) -> Option<u8> {
		self.get_char_at_offset(0)
	}

	#[inline]
	pub unsafe fn get_char_unchecked(&self) -> u8 {
		// SAFETY: upheld by caller
		unsafe { *self.cursor }
	}

	#[inline]
	pub fn peek_next_char(&self) -> Option<u8> {
		self.get_char_at_offset(1)
	}

	#[inline]
	pub fn get_char_and_advance(&mut self) -> Option<u8> {
		let chr = self.get_char();
		self.advance();
		chr
	}

	#[inline]
	pub fn advance_and_get_char(&mut self) -> Option<u8> {
		self.advance();
		self.get_char()
	}
}

#[derive(Hash, PartialEq, Eq)]
struct FileRef<'src>(&'src str);

/// Opaque structure representing a location in the original source.
/// (TODO) SourceManager is able to transcribe it into its (possibly included)
/// source file, line number and column.
#[derive(Debug, Clone, Copy)]
pub struct SourceLocation(usize);
