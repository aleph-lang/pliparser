use lalrpop_util::lalrpop_mod;
use aleph_syntax_tree::syntax::AlephTree as at;

lalrpop_mod!(pub grammar); // Generated parser from grammar.lalrpop

/// Parses a PL/I program into an AlephTree.
///
/// # Arguments
/// * `source` - String containing the PL/I code to parse.
///
/// # Returns
/// An `AlephTree` representing the parsed PL/I program.
/// In case of error, returns `at::Unit` and prints an error message.
pub fn parse(source: String) -> at {
    let ast = grammar::ProgramParser::new().parse(&source);
    match ast {
        Ok(res) => res,
        Err(e) => {
            eprintln!("Parse error: {:?}", e);
            at::Unit
        }
    }
}


