extern crate redis;

fn main() {
    println!("{}", hello_world());
}

pub fn hello_world() -> String {
  "hello world".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("hello world", hello_world());
    }
}

