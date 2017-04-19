extern crate redis;
use redis::Commands;

fn main() {
    //println!("{}", hello_world());
}

pub fn get_redis_data(key: &str) -> redis::RedisResult<(redis::Value)> {
    let con = try!(redis_client().get_connection());
    let result = con.lrange("users", 0, -1);
    result
}

pub fn create_redis_data() -> redis::RedisResult<()> {
    let con = redis_client().get_connection().unwrap();

    con.lpush("users", "Sylvanas")?;
    con.lpush("users", "Arthas")?;

    Ok(())
}

pub fn redis_cleanup() -> redis::RedisResult<()> {
    let con = redis_client().get_connection().unwrap();

    con.del("users")?;

    Ok(())
}

fn redis_client() -> redis::Client {
    let client = redis::Client::open("redis://127.0.0.1/");
    client.unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redis_connection() {
        create_redis_data();
        let redis_data = get_redis_data("users").unwrap();

        let data_results = redis::from_redis_value::<Vec<String>>(&redis_data).unwrap();


        assert_eq!(data_results[0], "Arthas");
        assert_eq!(data_results[1], "Sylvanas");


        redis_cleanup();
    }
}

