# mylint-rs

Tiny homemade linter for Rust based on [tree-sitter](https://github.com/tree-sitter).

The current main goal of this linter is to find out any usage that might crash.

For example:
```rust
func_call().unwrap()
func_call().expect()

unchecked_array[large_index]
unchecked_array[0..large_index]

unsafe { do_something_unsafe }
// unsafe_pinned for example
call_function_with_unsafe_in_its_name()
```

Just run `mylint` and it will tell you how to fix them.

**NOTICE: This linter will ignore all the test modules.**

Usage:

```
$ make

# This will copy the `mylint` binary to `/usr/local/bin/mylint`
$ make install

# Lint your codes. The default directory is `./src`
$ mylint -p <src directory>

# List all rules
# mytlint -l

# Suppress some rules listed as above in the first column (case-insensitive).
# mylint -s expect -s unwrap
```

## Why
I believe linter can save the time for every team member in a project,
even though some rules might seem stupid (even to myself !!!).

Rules will be followed if they are easy to do so.

## Disclaimer
This tool is not to persuade the whole rust community to remove `unsafe` in their codes,
which is also impossible and useless.
I appreciate those smart experts who devote their time to writing and reviewing unsafe codes
so that I don't have to.
It's just a very small tool for linting Rust codes for better team collaboration
only for the projects which don't need to use `unsafe` and see reliability as their top priority.
*Please don't use this to attack other projects*

**The codebase of this linter is very small and feel free to create `yourlinter` :)**.

## Development
The Rust grammar sits in:
- https://github.com/tree-sitter/tree-sitter-rust/blob/master/grammar.js

And there's an online playground to view the grammar structure:
- https://tree-sitter.github.io/tree-sitter/playground

Just write out some Rust codes you want to lint and it will give you the grammar structure.

For further details, the source code should be self-explained.
