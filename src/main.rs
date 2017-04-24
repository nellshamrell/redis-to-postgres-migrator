extern crate redis;
use redis::Commands;

extern crate postgres;
use postgres::{Connection, TlsMode};

fn main() {
    //println!("{}", hello_world());
}

struct User {
    name: String,
}

fn redis_to_postgres(redis_data: Vec<String>, postgres_conn: postgres::Connection) {
    let conn = postgres_connection();
    for row in redis_data {

		let user = User {
        name: row,
    };

		 conn.execute("INSERT INTO users (name) VALUES ($1)",
										 &[&user.name]).unwrap();

    }
}

fn redis_client() -> redis::Client {
    let client = redis::Client::open("redis://127.0.0.1/");
    client.unwrap()
}

fn postgres_connection() -> postgres::Connection {
    let connection = Connection::connect("postgres://postgres@localhost", TlsMode::None).unwrap();
    connection
}


#[cfg(test)]
mod tests {
    use super::*;

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

    fn create_postgres_schema() {
				let conn = postgres_connection();
				conn.execute("CREATE TABLE IF NOT EXISTS users (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL
                  )", &[]).unwrap();
    }

    fn postgres_cleanup() {
				let conn = postgres_connection();
				conn.execute("DROP TABLE IF EXISTS users", &[]).unwrap();
    }

    #[test]
    fn test_postgres() {
        redis_cleanup();
        postgres_cleanup();

        create_redis_data();
        create_postgres_schema();

        let postgres_conn = postgres_connection();

        let users_redis_data = get_redis_data("users").unwrap();
        let users_data = redis::from_redis_value::<Vec<String>>(&users_redis_data).unwrap();

        redis_to_postgres(users_data, postgres_conn);

        let mut postgres_users: Vec<User> = vec![];

        let new_postgres_conn = postgres_connection();
        for row in &new_postgres_conn.query("SELECT * FROM users", &[]).unwrap() {
            let user = User {
                name: row.get(1),
            };
            postgres_users.push(user);
        }

        assert_eq!(postgres_users.len(), 2);

        redis_cleanup();
        postgres_cleanup();
    }
}

