# Axum POC

> Basic HTTP Server 

* GET /items → returns all values
* POST /items → inserts a value

## Usage

* Run
```
cargo run
```

* Create item
```
curl -X POST http://localhost:3000/items \
  -H "Content-Type: application/json" \
  -d '{"value":"gabriel"}'
```

* Get items
```
curl http://localhost:3000/items
```

* Create other item
```
curl -X POST http://localhost:3000/items \
  -H "Content-Type: application/json" \
  -d '{"value":"foo"}'
```

* Test
```
cargo test
```