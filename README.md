# rust-rest-api
Rust Rest Api with Actix and sqlx

# add crate
- Actix
```
cargo add actix-web dotenv env_logger
```

# add crate for database and serialize
```
cargo add sqlx --features postgres
cargo add serde --features derive
cargo install sqlx-cli
```

## install sqlx cli
```
cargo install sqlx-cli
```

## create the migration
```
sqlx migrate add create_blog_posts_table
```

## run migration
```
sqlx migrate run --database-url "postgres://postgres:S3cret@localhost:5432/rustblog"
```