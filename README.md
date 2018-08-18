# Sample Nom parser: An EGL parser

This is a parser written in Nom for the toy language EGL - it is a sample parser written in order to get a feel for Nom.

**Uses:** Nom 4.0.x

## Build instructions

### Building

```
$ cargo build
```

### Running

```
$ cargo run
```

## EGL - the Enthusiastic Greeting Language

Welcome to the Enthusiastic Greeting Language! EGL has three constructs:
- **Expression start:** The word `HI` signifies the start of an expression
- **Expression end:** The symbol `!` signifies the end of an expression
- **Optional expression block:** Between the start and end tags, there may be a block, delimited by `{` and `}`, which may contain up to one expression

Additional rules and clarifications:
- EGL accepts an arbitrary amount of whitespace between each word it accepts, as well as preceding and following whitespace
- Any expression started must be ended
- Any block started with a `{` symbol must be closed with a `}` symbol
- Blocks may be empty (or contain whitespace)

### Sample programs written in EGL

```
HI!
```

```

HI    !

```

```
HI {   }!
```

```
HI {
  HI{
HI{  }!}  !
}


```

## Data structures

```rs
struct ParseError {
    description: String,
}
```

```rs
enum EGLAst {
    Blank,                  // Can only be passed to ExprBlock
    ExprEnd,                // Can only be passed to and explicitly ends ExprStart
    ExprStart(Box<EGLAst>), // This is the outermost part of every expression
    ExprBlock(Box<EGLAst>), // Implicitly ends ExprStart
}
```

## Helper functions

```rs
fn is_whitespace(c : char) -> bool {
    return c.is_whitespace();
}
```

## Combinators

- `egl_end_tag` - consumes a `!` character surrounded by any amount of whitespace. Returns ExprEnd.
- `egl_start_tag` - consumes a `HI` tag surrounded by any amount of whitespace.
- `blank_block` - consumes any amount of whitespace. Returns Blank.
- `egl_block` - consumes a block delimited by `{` and `}` surrounded by any amount of whitespace. Returns ExprBlock(...).
- `egl` - consumes a complete EGL expression. Returns ExprStart(...).

## Findings

I've found that it's relatively simple to work with Nom, after a little bit of practice. It was helpful to create simple named combinators which could be used as building blocks in order to define the grammar of Nom, and I felt like it resulted in somewhat readable and easy to understand code.

The errors are bullshit.

Also note that in order to properly use the whitespace wrapper combinator, you need to use the `CompleteStr` or `CompleteByteSlice` wrappers. Also, `ws!()` doesn't seem to work with an empty body. Also, `do_parse!()` does not seem to work without a return value.

A complete list of combinators to choose from can be found [here](https://docs.rs/nom/4.0.0/nom/).

## Disclaimer

I don't actually know how to program in Rust. What I've done here is the result of reading a couple of pages of the Rust Book, googling a lot of stuff, and a bunch of trying and failing. If I've done something stupid in this code... Do not be alarmed. It is merely what should be expected.
