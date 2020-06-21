![Rust](https://github.com/alperhankendi/rust-sample-api/workflows/Rust/badge.svg?event=status)

![Rust](https://github.com/alperhankendi/rust-sample-api/workflows/Rust/badge.svg?event=push)

## start api
```
cargo run
```

## Testing curls
After you update the code, restart the server via cargo run and use these curls to post, update, get, and delete items.

#### POST
```curl --location --request POST 'localhost:3030/v1/basket' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "apple",
    "quantity": 3
}'
```
#### UPDATE
```
curl --location --request PUT 'localhost:3030/v1/basket' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "apple",
    "quantity": 5
}'
```
#### GET
```
curl --location --request GET 'localhost:3030/v1/basket' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain'
```

#### DELETE
```
curl --location --request DELETE 'localhost:3030/v1/basket' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "name": "apple"
}'
```
