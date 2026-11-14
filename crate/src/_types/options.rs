use oxc::allocator::Allocator;
use oxc::ast::ast::SourceType;

#[derive(Clone, Copy)]
pub struct JsonToProgramOptions<'a> {
    /// Arena allocator the resulting AST nodes are allocated in.
    pub allocator: &'a Allocator,
    /// Source type of the program; resolves unambiguous values from the JSON.
    pub source_type: SourceType,
    /// Original source text, used to build string atoms from the program.
    pub source_text: &'a str,
}
