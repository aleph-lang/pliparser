# pliparser

Parses PL/I source code into an [`AlephTree`](https://github.com/aleph-lang/aleph-syntax-tree).
Built with [LALRPOP](https://github.com/lalrpop/lalrpop).

## Installation

```toml
[dependencies]
pliparser = "0.1"
```

## Usage

```rust
let ast = pliparser::parse(source_code);
```

## Example

Input:

```pli
DCL X FIXED BIN(31) INIT(42);
```

Produces an `AlephTree::Let` binding with an `Int` value.

## Related

- [`aleph-syntax-tree`](https://github.com/aleph-lang/aleph-syntax-tree) — AST definition
- [`alephc`](https://github.com/aleph-lang/aleph) — full compiler
