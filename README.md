# Server/API built using Rust's Rocket.rs framework and MongoDB database running in a Docker container.

## To run the app:

Apart from the obvious prerequisite of having Rust and Docker installed, we must have an .env file in the project root with the following configuration:

```
MONGO_INITDB_ROOT_USERNAME=admin
MONGO_INITDB_ROOT_PASSWORD=xxxx
MONGO_INITDB_DATABASE=rust_mongodb

MONGODB_USER_COLLECTION=users
DATABASE_URL=mongodb://admin:xxxx@localhost:27017/rust_mongodb?authSource=admin
```

To create the container with a MongoDB image and a Docker-managed storage volume run in the project root:

```
$ docker compose up -d
```

If we have _cargo-watch_ installed using:

```
$ cargo install cargo-watch
```

we won't have to restart the server every time we make a code change; running the following command in the root of the project:

```
$ cargo watch -x run
```

rather:

```
$ cargo run
```

the server will restart automatically 😀.
