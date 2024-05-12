mod compiler;
pub mod interning;
pub mod lexer;
mod operators;
pub mod parser;
mod register_allocator;

pub use self::{
    compiler::{compile_chunk, CompileError, CompileErrorKind, CompiledPrototype, FunctionRef},
    interning::StringInterner,
    lexer::LineNumber,
    parser::parse_chunk,
    parser::{ParseError, ParseErrorKind},
};
