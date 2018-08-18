// Let's create a toy language: The Enthusiastic Greeting Language (EGL)
//
// The language has three words:
// - "HI": This is a word that signifies the start of an expression.
// - "{" and "}": MAY contain a single expression. When opened, must be closed.
// - "!": This concludes an expression. Every expression started must be concluded.
// 
// Any number of whitespace can be inserted anywhere between words.
//
// Sample programs:
//
// ```
// HI!
// ```
//
// ```
// HI    !
// ```
//
// ```
// HI
//
//     !
// ```
//
// ```
// HI
// {
//   HI   {   HI{} !   }!
// }
// !
// ```

#[macro_use]
extern crate nom;

use nom::types::CompleteStr; // wrapper which helps whitespace stuff to work

// this is probably not necessary, but I included it bcz why not
#[derive(Debug)]
struct ParseError {
    description: String,
}

#[derive(Debug)]
enum EGLAst {
    Blank,                  // Can only be passed to ExprBlock
    ExprEnd,                // Can only be passed to and explicitly ends ExprStart
    ExprStart(Box<EGLAst>), // This is the outermost part of every expression
    ExprBlock(Box<EGLAst>), // Implicitly ends ExprStart
}

// helper function
fn is_whitespace(c : char) -> bool {
    return c.is_whitespace();
}

// consumes a ! character surrounded by any amount of whitespace
// RESULT: ExprEnd
named!(egl_end_tag<CompleteStr, EGLAst>,
       value!(EGLAst::ExprEnd, ws!(char!('!'))));

// consumes a HI tag surrounded by any amount of whitespace
named!(egl_start_tag<CompleteStr, CompleteStr>,
       ws!(tag!("HI")));

// consumes any amount of whitespace
// RESULT: Blank
named!(blank_block<CompleteStr, EGLAst>,
       value!(EGLAst::Blank, take_while!(is_whitespace))); // can't use ws! with empty body, for some reason

// consumes a block delimited by { and } surrounded by any amount of whitespace
// RESULT: ExprBlock with a complete AST as an argument
named!(egl_block<CompleteStr, EGLAst>,
       ws!(do_parse!(block_content: delimited!(char!('{'), alt!(egl | blank_block), char!('}'))
                     >> ( EGLAst::ExprBlock(Box::new(block_content)) ) ))); 

// consumes an EGL expression
// RESULT: ExprStart with a complete AST as an argument
named!(egl<CompleteStr, EGLAst>,
       do_parse!(egl_start_tag 
                 >> ast: alt!(egl_end_tag | do_parse!(val: egl_block >> egl_end_tag >> (val)))
                 >> ( EGLAst::ExprStart(Box::new(ast)) )));

// convenience function for parsing a program written in EGL
// tries to give good errors on fail, unless nom::Err, in which case, fuck all
fn parse_egl(input: &str) -> Result<EGLAst, ParseError> {
    let res = egl(CompleteStr(input));
    match res {
        Ok((rem, ast)) => {
            if rem != CompleteStr("") {
                return Err(ParseError { description: format!("Expected end of input, but found '{}'", rem) });
            }
            return Ok(ast);
        },
        Err(err) => {
            return Err(ParseError { description: format!("{:?}", err) });
        },
    }
}

// parsing an expression
fn main() {
    let expr = "HI { HI{  }! } !  ";
    println!("Trying to parse the expression '{}'...", expr);
    match parse_egl(expr) {
        Ok(x) => println!("Result: {:?}", x),
        Err(x) => println!("Parsing error: {}", x.description),
    }
}

// TODO: TESTS
