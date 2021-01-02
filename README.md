# indirect-once

One-time resolvable runtime indirection in the style of glibc's `ifunc`.

[![Crates.io](https://img.shields.io/crates/v/indirect-once)](https://crates.io/crates/indirect-once)
[![Docs.rs](https://docs.rs/indirect-once/badge.svg)](https://docs.rs/indirect-once)
[![GitHub top language](https://img.shields.io/github/languages/top/toshokan/indirect-once)](https://github.com/toshokan/indirect-once)


## Usage
The `indirect-once` crate provides some macros to help with runtime indirection like glibc's `ifunc` (but without special compiler support).
The exposed API aims to be similar in spirit to the `ifunc` API, but with a Rust flavour. 

### Example
Two different styles of macro are supported by `indirect-once`. 

#### Procedural Macro

The default style is an attribute procedural macro. It looks a bit cleaner and closer to the original `ifunc` (which is also exposed as an attribute).
A downside of this style is that it pulls in dependencies on some proc-macro tooling crates which may increase compile time (`syn`, `proc_macro2`, `quote`, etc). If keeping your dependency graph small or your compile time as low as possible is important to you, you can opt out of this by disabling the `proc-macro` feature. 

```rust
use indirect_once::*;

// Two implementations of foo. One should be picked at runtime based on hardware features.
fn foo_with_avx(x: i32, y: u8, p: bool) -> u16 { unimplemented!() };
fn foo_with_sse(x: i32, y: u8, p: bool) -> u16 { unimplemented!() };

// A resolver function to decide which to pick. Gets called at most once, the first time foo is called.
fn resolve_foo() -> &'static fn(i32, u8, bool) -> u16 {
	if(cpu_has_avx!()) {
	  &(foo_with_avx as _)
	} else {
	  &(foo_with_sse as _)
	}
}

// Now define the real foo.

#[indirect(resolver = "resolve_foo")]
pub fn foo(x: i32, y: u8, p: bool) -> u16 {}
```

#### `macro_rules!` Macro

The other style of macro is a simple `macro_rules!` macro. The generated code is very similar, but it does not pull in any proc-macro crates.

```rust
use indirect_once::*;

// Two implementations of foo. One should be picked at runtime based on hardware features.
fn foo_with_avx(x: i32, y: u8, p: bool) -> u16 { unimplemented!() };
fn foo_with_sse(x: i32, y: u8, p: bool) -> u16 { unimplemented!() };

// A resolver function to decide which to pick. Gets called at most once, the first time foo is called.
fn resolve_foo() -> &'static fn(i32, u8, bool) -> u16 {
	if(cpu_has_avx!()) {
	  &(foo_with_avx as _)
	} else {
	  &(foo_with_sse as _)
	}
}

// Now define the real foo.

pub indirect_fn! {
  resolver = resolve_foo; fn foo(x: i32, y: u8, p: bool) -> u16 {}
}
```

### Behaviour

In either style, the function specified in the `resolver` argument (of the macro or attribute) will be called at most once the first time the newly-declared function is called.

By default, the implementation uses `std::sync::Once`, but the `parking-lot` feature replaces this with a `parking_lot::Once`-based backend instead.
In either case, once the resolver has run once, this should be a simple atomic load.

## License
`indirect-once` is dual licensed under the MIT license and the Apache 2.0 license. You may choose whichever one you prefer.
