use std::env;
extern crate redis;
use redis::Commands;

extern crate postgres;
use postgres::{Connection, TlsMode};

#[cfg(not(test))]
fn main() {
    let args: Vec<_> = env::args().collect();
    println!("here be those args!");
    println!("{:?}", args);
    redis_to_postgres(redis_client(&args[1]), postgres_connection(&args[2]));
}

struct User {
    name: String,
}

fn redis_to_postgres(redis_conn: redis::Client, postgres_conn: postgres::Connection) {
    let users_redis_data = get_redis_data(redis_conn, "users").unwrap();
    let users_data = redis::from_redis_value::<Vec<String>>(&users_redis_data).unwrap();


    for row in users_data {

        let user = User { name: row };

        postgres_conn
            .execute("INSERT INTO users (name) VALUES ($1)", &[&user.name])
            .unwrap();

    }
}

fn redis_client(address: &str) -> redis::Client {
    let client = redis::Client::open(address);
    client.unwrap()
}

fn postgres_connection(address: &str) -> postgres::Connection {
    let connection = Connection::connect(address, TlsMode::None).unwrap();
    connection
}

fn get_redis_data(redis_conn: redis::Client, key: &str) -> redis::RedisResult<(redis::Value)> {
    let result = redis_conn.lrange(key, 0, -1);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn create_redis_data() -> redis::RedisResult<()> {
        let con = redis_client("redis://127.0.0.1/")
            .get_connection()
            .unwrap();

        con.lpush("users", "Sylvanas")?;
        con.lpush("users", "Arthas")?;

        Ok(())
    }

    pub fn redis_cleanup() -> redis::RedisResult<()> {
        let con = redis_client("redis://127.0.0.1/")
            .get_connection()
            .unwrap();

        con.del("users")?;

        Ok(())
    }

    fn create_postgres_schema() {
        let conn = postgres_connection("postgres://postgres@localhost");
        conn.execute("CREATE TABLE IF NOT EXISTS users (
                    id              SERIAL PRIMARY KEY,
                    name            VARCHAR NOT NULL
                  )",
                     &[])
            .unwrap();
    }

    fn postgres_cleanup() {
        let conn = postgres_connection("postgres://postgres@localhost");
        conn.execute("DROP TABLE IF EXISTS users", &[]).unwrap();
    }

    #[test]
    fn test_postgres() {
        redis_cleanup();
        postgres_cleanup();

        create_redis_data();
        create_postgres_schema();

        let redis_conn = redis_client("redis://127.0.0.1/");
        let postgres_conn = postgres_connection("postgres://postgres@localhost");

        redis_to_postgres(redis_conn, postgres_conn);

        let mut postgres_users: Vec<User> = vec![];

        let new_postgres_conn = postgres_connection("postgres://postgres@localhost");
        for row in &new_postgres_conn
                        .query("SELECT * FROM users", &[])
                        .unwrap() {
            let user = User { name: row.get(1) };
            postgres_users.push(user);
        }

        assert_eq!(postgres_users.len(), 2);

        redis_cleanup();
        postgres_cleanup();
    }
}
