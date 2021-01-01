pub use indirect_once_derive::*;

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
    fn it_works() {
	#[indirect(resolver = "foo")]
	fn hello_world(arg: i32) -> i32 {}

	assert_eq!(hello_world(10), 11);
    }

    #[test]
    fn it_works_2() {
	#[indirect(resolver = "bar")]
	fn hello_hello(arg: i32) -> i32 {}

	assert_eq!(hello_hello(10), 9);
    }
}
