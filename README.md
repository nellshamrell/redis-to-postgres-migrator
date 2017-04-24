Prototype for a redis to postgres data migrator

To use:

* Run redis on your machine
  * Enter the redis cli
  ```
    $ redis-cli
  ```
  * Create a users list and add two names
  ```
    LPUSH users "Sylvanas"
    LPUSH users "Arthas"
  ```

* Run Postgres on your machine (cli or GUI, either is fine)
  * Create a games database
  ```
    CREATE DATABASE games;
  ```
  * Create a users table
  ```
    CREATE TABLE users (id SERIAL PRIMARY KEY, name VARCHAR NOT NULL);
  ```

 * Now head back to your terminal, make sure you are in the directory for this repo.  Run.
 ```
  $ cargo run -- -- "redis://127.0.0.1/" "games://postgres_username@localhost"
 ```

* After it completes, check out the users table in your games database, you should see both Sylvanas and Arthas listed!
