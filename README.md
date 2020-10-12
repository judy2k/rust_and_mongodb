# Getting Started with MongoDB & Rust - Sample Code

This repository contains sample code that was used in the RustLab talk,
[Getting Started with MongoDB & Rust](https://www.rustlab.it/agenda/session/330778).

Most of the code you'll be interested in is in [main.rs](./src/main.rs).

If you want to run the code, set an environment variable `MDB_URL` to point to your MongoDB cluster, and then execute `cargo run`.

* [extra](./extras) contains some small Python scripts for generating the sample database.
* [extra/recipes](./extra/recipes) contains a small handful of my favourite cocktail recipes.

## More Information

* [Building with Patterns](https://www.mongodb.com/blog/post/building-with-patterns-a-summary) - A reference to various MongoDB design patterns.
* My colleague Lauren has produced a fantastic series of blog posts describing [Schema Design Anti-Patterns](https://developer.mongodb.com/article/schema-design-anti-pattern-massive-arrays)
* [rust-groceries-mongodb](https://github.com/pkdone/rust-groceries-mongo-api) - A full, yet simple example MongoDB Rust app, by my colleague Paul Done