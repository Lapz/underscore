extern crate underscore_syntax as syntax;
extern crate underscore_util as util;
mod frame;
mod temp;
mod x86;
mod translate;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
