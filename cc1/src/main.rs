use cc1::{diagnostics::DiagnosticsManager, lexer, source_manager::SourceManager};

fn main() {
	const FILE_NAME: &str = "b_pp.c";

	let mut args = std::env::args();
	args.next();
	let arg = args.next();

	let file_name = if let Some(x) = &arg { x.as_str() } else { FILE_NAME };

	let contents = std::fs::read_to_string(file_name).unwrap();
	let diag_manager = DiagnosticsManager::new(20, false);
	let source = SourceManager::from(&*contents, file_name, diag_manager);
	let lexer = lexer::Lexer::from(&source);
	for token in lexer {
		println!("{:?}", token);
	}
}
