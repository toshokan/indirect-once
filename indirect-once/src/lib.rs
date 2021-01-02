#![allow(unused_imports)]

#[cfg(feature = "proc-macro")]
pub use indirect_once_derive::*;

#[cfg(not(feature = "parking-lot"))]
use std::sync::Once;
#[cfg(feature = "parking-lot")]
use parking_lot::Once;

#[macro_export]
macro_rules! indirect_fn {
    (resolver = $resolver: ident ; fn $name:ident($($arg: ident : $pty: ty),*) {}) => { $crate::indirect_fn!(resolver = $resolver; fn $name($($arg: $pty),*) -> () {})};
    (resolver = $resolver: ident ; fn $name:ident($($arg: ident : $pty: ty),*) -> $ret: ty {})=> {
	fn $name($($arg: $pty),*) -> $ret {
	    static mut IMPL: Option<&'static fn($($pty),*) -> $ret> = None;
	    static INIT: Once = Once::new();

	    unsafe {
		INIT.call_once(|| {
		    IMPL = Some($resolver());
		});
		(IMPL.unwrap())($($arg),*)
	    }
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn incr(x: i32) -> i32 {
	x + 1
    }

    fn decr(x: i32) -> i32 {
	x - 1
    }
    
    fn foo() -> &'static fn(i32) -> i32 {
	&(incr as fn(i32) -> i32)
    }

    fn bar() -> &'static fn(i32) -> i32 {
	&(decr as fn(i32) -> i32)
    }
    
    #[test]
    #[cfg(feature = "proc-macro")]
    fn it_works() {
	#[indirect(resolver = "foo")]
	fn hello_world(arg: i32) -> i32 {}

	assert_eq!(hello_world(10), 11);
    }

    #[test]
    #[cfg(feature = "proc-macro")]
    fn it_works_2() {
	#[indirect(resolver = "bar")]
	fn hello_hello(arg: i32) -> i32 {}

	assert_eq!(hello_hello(10), 9);
    }

    #[test]
    #[cfg(feature = "proc-macro")]
    fn multiple_arguments() {
	fn do_thingy(one: i32, two: i32, really: bool) -> (bool, i32) {
	    if really {
		(really, one + two)
	    } else {
		(really, two)
	    }
	}

	fn pick() -> &'static fn(i32, i32, bool) -> (bool, i32) {
	    &(do_thingy as _)
	}

	#[indirect(resolver = "pick")]
	fn foo(one: i32, two: i32, three: bool) -> (bool, i32) {}

	assert_eq!(foo(1, 2, true), (true, 3))
    }

    #[test]
    fn macro_impl() {
	indirect_fn! {
	    resolver = foo; fn dog(param: i32) -> i32 {}
	}

	assert_eq!(dog(41), 42);
    }

    #[test]
    fn runs_once() {
	use std::sync::atomic::{AtomicU32, Ordering};
	static A: AtomicU32 = AtomicU32::new(0);
	
	fn chooser() -> &'static fn(i32) -> i32 {
	    A.fetch_add(1, Ordering::SeqCst);
	    &(incr as _)
	}

	#[indirect(resolver = "chooser")]
	fn foo(val: i32) -> i32 {}

	assert_eq!(foo(1), 2);
	assert_eq!(foo(2), 3);
	assert_eq!(A.load(Ordering::SeqCst), 1)
    }
}
