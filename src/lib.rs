pub fn add(left: usize, right: usize) -> usize {
    left + right
}
use add_two;
pub fn add_one(x: i32) -> i32 {
    return add_two::add_two(x)*2;
}
pub mod command;
use command::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
pub fn print_type_of<T>(_: T) {
   println!("sam def type print:: {}", std::any::type_name::<T>());
}