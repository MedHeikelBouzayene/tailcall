# test-invalid-query-in-http

###### sdl error

#### server:

```graphql
schema @server(vars: [{key: "id", value: "1"}]) {
  query: Query
}

type User {
  name: String
  id: Int
}

type Query {
  user: [User]
    @http(path: "/users", query: {key: "id", value: "{{vars.id}}"}, baseURL: "http://jsonplaceholder.typicode.com")
}
```