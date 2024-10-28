# Composed queries with field modify check

```graphql @config
schema @server(port: 8001, queryValidation: false, hostname: "0.0.0.0") @upstream(httpCache: 42) {
  query: Query
}

type Query {
  user(id: ID!): User!
    @graphQL(url: "http://upstream/graphql", name: "user", args: [{key: "id", value: "{{.args.id}}"}])
  page(id: ID!): Page!
    @graphQL(url: "http://upstream/graphql", name: "page", args: [{key: "id", value: "{{.args.id}}"}])
}

type User {
  id: ID!
  name: String! @modify(name: "newName")
  post: Post @modify(name: "userPost")
}

type Page {
  id: ID!
  name: String!
  post: Post @modify(name: "pagePost")
}

type Post {
  id: ID!
  content: String
}
```

```yml @mock
- request:
    method: POST
    url: http://upstream/graphql
    textBody: '{ "query": "query { user(id: 4) { name post { id content } } }" }'
  expectedHits: 1
  response:
    status: 200
    body:
      data:
        user:
          name: Tailcall
          post:
            id: 0
            content: Hello from user 4
- request:
    method: POST
    url: http://upstream/graphql
    textBody: '{ "query": "query { page(id: 4) { name post { id content } } }" }'
  expectedHits: 1
  response:
    status: 200
    body:
      data:
        page:
          name: Tailcall_page
          post:
            id: 1
            content: Hello from page 4
```

```yml @test
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: |
      query getUser {
        user(id: 4) {
          newName
          userPost {
            id
            content
          }
        }
      }
- method: POST
  url: http://localhost:8080/graphql
  body:
    query: |
      query getPage {
        page(id: 4) {
          name
          pagePost {
            id
            content
          }
        }
      }
```
