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
    
    fn resolve_i() -> &'static fn(i32) -> i32 {
	&(incr as fn(i32) -> i32)
    }

    fn resolve_d() -> &'static fn(i32) -> i32 {
	&(decr as fn(i32) -> i32)
    }
    
    #[test]
    fn it_works() {
	#[ifunc(resolver = "resolve_i")]
	fn hello_world(arg: i32) -> i32 {}

	assert_eq!(hello_world(10), 11);
    }

    #[test]
    fn it_works_2() {
	#[ifunc(resolver = "resolve_d")]
	fn hello_hello(arg: i32) -> i32 {}

	assert_eq!(hello_hello(10), 9);
    }
}
