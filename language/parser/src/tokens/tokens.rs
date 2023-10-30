use std::convert::Infallible;
use std::ops::{ControlFlow, FromResidual, Try};
use syntax::ParsingError;

/// A token is a single string of characters in the file.
/// For example, keywords, variables, etc... are a single token.
#[derive(Clone, Debug)]
pub struct Token {
    // The type of the token
    pub token_type: TokenTypes,
    // The starting line and index in that line of the token.
    pub start: (u32, u32),
    // The offset to the start of the token
    pub start_offset: usize,
    // The ending line and index in that line of the token.
    pub end: (u32, u32),
    // The offset to the end of the token
    pub end_offset: usize,
    // Data about the code block around this token
    pub code_data: Option<TokenCodeData>
}

impl Token {
    pub fn new(token_type: TokenTypes, code_data: Option<TokenCodeData>, start: (u32, u32), start_offset: usize, end: (u32, u32), end_offset: usize) -> Self {
        return Self {
            token_type,
            start,
            start_offset,
            end,
            end_offset,
            code_data
        }
    }

    /// Creates an error for this part of the file.
    pub fn make_error(&self, file: String, error: String) -> ParsingError {
        return ParsingError::new(file, self.start, self.start_offset, self.end, self.end_offset, error);
    }

    /// Turns the token into the string it points to.
    pub fn to_string(&self, buffer: &[u8]) -> String {
        let mut start = self.start_offset;
        let mut end = self.end_offset-1;
        while buffer[start] == b' ' || buffer[start] == b'\t' || buffer[start] == b'\r' || buffer[start] == b'\n' &&
            start < end {
            start += 1;
        }
        while buffer[end] == b' ' || buffer[end] == b'\t' || buffer[end] == b'\r' || buffer[end] == b'\n' &&
            start < end {
            end -= 1;
        }
        return String::from_utf8_lossy(&buffer[start..end+1]).to_string();
    }
}

/// This allows for Tokens to be used in the Result type.
///
impl Try for Token {
    type Output = Token;
    type Residual = Token;

    fn from_output(output: Self::Output) -> Self {
        return output;
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        return ControlFlow::Continue(self);
    }
}

/// Required for Try
impl FromResidual<Token> for Token {
    fn from_residual(residual: Token) -> Self {
        return residual;
    }
}

/// Required for Try
impl FromResidual<Result<Infallible, Token>> for Token {
    fn from_residual(residual: Result<Infallible, Token>) -> Token {
        return residual.err().unwrap();
    }
}

/// Data about the current code block
#[derive(Clone, Debug)]
pub struct TokenCodeData {
    pub start_line: u32,
    pub end_line: u32
}

/// The different types of tokens.
/// The numerical value assigned is arbitrary and used
/// for deriving functions like equals
/// and some IDEs require a numerical id for each token.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenTypes {
    Start,
    EOF,
    InvalidCharacters,
    StringStart,
    StringEscape,
    StringEnd,
    ImportStart,
    Identifier,
    AttributesStart,
    Attribute,
    ModifiersStart,
    Modifier,
    GenericsStart,
    Generic,
    GenericBound,
    GenericEnd,
    ArgumentsStart,
    ArgumentName,
    ArgumentType,
    ArgumentEnd,
    ArgumentsEnd,
    ReturnType,
    CodeStart,
    StructStart,
    TraitStart,
    ImplStart,
    FunctionStart,
    StructTopElement,
    StructEnd,
    FieldName,
    FieldType,
    FieldValue,
    FieldEnd,
    LineEnd,
    Operator,
    CodeEnd,
    Variable,
    Integer,
    Float,
    CallingType,
    Return,
    Break,
    Switch,
    For,
    While,
    Else,
    If,
    ParenOpen,
    ParenClose,
    BlockStart,
    BlockEnd,
    New,
    Colon,
    In,
    ImportEnd,
    ReturnTypeArrow,
    ArgumentTypeSeparator,
    ArgumentSeparator,
    Let,
    Equals,
    AttributeEnd,
    FieldSeparator,
    Period,
    Comment,
    True,
    False,
    AttributeStart,
    GenericBoundEnd,
    GenericsEnd,
    Do,
    Char
}