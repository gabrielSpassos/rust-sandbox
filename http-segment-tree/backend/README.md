# HTTP Segment Tree POC

| Method | Endpoint               | Description      |
| ------ | ---------------------- | ---------------- |
| POST   | `/segment-tree`        | Create tree      |
| GET    | `/segment-tree`        | Get array + tree |
| PUT    | `/segment-tree`        | Update index     |
| GET    | `/segment-tree/query`  | Range sum query  |

## Usage

* Run
```
cargo run
```

* Test
```
cargo test
```

* Create Tree
```
curl -X POST http://localhost:3000/segment-tree \
  -H "Content-Type: application/json" \
  -d '{"input":[5,8,7,2,10,2,2]}'
```

* Get Tree
```
curl -X GET http://localhost:3000/segment-tree
```

* Update Tree
```
curl -X PUT http://localhost:3000/segment-tree \
  -H "Content-Type: application/json" \
  -d '{"idx":3,"value":6}'
```

* Query Range
```
curl "http://localhost:3000/segment-tree/query?left=0&right=3"
```