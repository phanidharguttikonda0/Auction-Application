## sqlx crate tutorial

where we can use sqlx for many databases that includes postgres, Mysql, Maria Db etc.

where we can create a connection as follows:

let pool = PgPoolOptions::new()
.max_connections(5)
.connect("postgres://user:password@localhost/mydb").await?;

Now let's see the different ways to query:

basically we have totally 4 methods to execute a query

-> execute(&pool) used for operations like insert, update, delete, returns Result<sqlx::postgres::PgQueryResult, sqlx::Error>.

-> fetch_one(&pool) returns first occurance returns Result<Row, sqlx::Error> if no row was found

-> fetch_optional(&pool) it returns Result<Option<Row>, sqlx::Error>,None if no row presents

-> fetch_all(&pool) for getting all the records

examples given below :

let rows = sqlx::query("SELECT id, name FROM users")
.fetch_all(&pool)
.await?;

query is used for executing the query

we also have query_as::<_,T>() which requires a return type to be mentioned
used to map results directly to the struct.

query_scalar() where it returns the out-put in form of tuple.
let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
.fetch_one(&pool)
.await?;

All these forms of query executions will be available in macros which helps in compile time checks
let row = sqlx::query!(
"SELECT id, name FROM users WHERE id = $1",
42
)
.fetch_one(&pool)
.await?;

println!("User name: {}", row.name);
same for query_as! and query_scalar!.

But these requires sqlx-cli.

