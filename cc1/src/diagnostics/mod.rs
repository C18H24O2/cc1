mod diag_kind;

pub use diag_kind::{Diag, DiagKind};

use crate::{source_manager::{SourceLocation, SourceLocationInfo, SourceManager}};
use std::cell::{Cell, RefCell, Ref};

pub struct DiagnosticsManager<'src> {
	curr_error_count: Cell<usize>,
	max_error_count: usize,
	diags: Option<RefCell<Vec<Diag<'src>>>>
}

impl<'src> DiagnosticsManager<'src> {
	pub fn new(max_error_count: usize, store_diagnostics: bool) -> DiagnosticsManager<'src> {
		let diags = if store_diagnostics {
			Some(RefCell::new(Vec::with_capacity(max_error_count * 2)))
		}
		else { None };
		DiagnosticsManager {
			curr_error_count: Cell::from(0),
			max_error_count,
			diags,
		}
	}

	#[inline]
	pub fn max_error_count(&self) -> usize {
		self.max_error_count
	}

	#[inline]
	pub fn curr_error_count(&self) -> usize {
		self.curr_error_count.get()
	}

	#[inline]
	pub fn has_errored(&self) -> bool {
		self.curr_error_count() != 0
	}

	#[inline]
	pub fn get_diags(&'src self) -> Option<Ref<'src, Vec<Diag<'src>>>> {
		if let Some(v) = &self.diags {
			Some(v.borrow())
		}
		else {
			None
		}
	}

	fn print_diag(diag: Diag<'src>) {
		let kind_str = match diag.get_kind() {
			DiagKind::Error => "\x1b[31merror: \x1b[39m",
			DiagKind::Warning => "\x1b[35mwarning: \x1b[39m",
			DiagKind::Message => "\x1b[34minfo: \x1b[39m",
		};
		println!("{}{}", kind_str, diag);
	}

	fn print_diag_with_loc(diag: Diag<'src>, loc_info: SourceLocationInfo) {
		const ANSI_BOLD: &str = "\x1b[1m";
		const ANSI_RESET: &str = "\x1b[0m";

		print!("{}{}:{}:{}: ", ANSI_BOLD, loc_info.file, loc_info.line_num, loc_info.column_num);
		Self::print_diag(diag);
		print!("{}", ANSI_RESET);
		print!("\t{}\n\t", loc_info.line);
		for _ in 0..loc_info.column_num { print!(" "); }
		println!("\x1b[32m{}^{}", ANSI_BOLD, ANSI_RESET);
	}

	#[inline]
	fn maybe_store_diag(&self, diag: Diag<'src>) {
		if let Some(v) = &self.diags {
			v.borrow_mut().push(diag);
		}
	}

	pub fn diag(&self, loc: &impl AsRef<SourceLocation>, diag: Diag<'src>, src: &SourceManager<'src>) {
		let mut curr_error_count = self.curr_error_count.get();

		if curr_error_count < self.max_error_count {
			Self::print_diag_with_loc(diag, src.get_location_info(*loc.as_ref()));
			self.maybe_store_diag(diag);

			if matches!(diag.get_kind(), DiagKind::Error) {
				curr_error_count += 1;
				if curr_error_count == self.max_error_count {
					Self::print_diag(Diag::err_too_many_errors);
					self.maybe_store_diag(Diag::err_too_many_errors);
				}
				self.curr_error_count.set(curr_error_count);
			}
		}
	}
}
