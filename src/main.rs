extern crate redis;
use redis::Commands;

fn main() {
    //println!("{}", hello_world());
}

pub fn get_redis_data(key: &str) -> redis::RedisResult<isize> {
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let con = try!(client.get_connection());
    con.get("my_key")
}

pub fn create_redis_data() -> redis::RedisResult<()> {
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let con = try!(client.get_connection());

    let _ :() = try!(con.set("my_key", 42));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redis_connection() {
        create_redis_data();
        let redis_data = get_redis_data("my_key");
        assert_eq!(redis_data.unwrap(), 42);
    }
}

