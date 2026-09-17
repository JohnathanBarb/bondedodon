# bondedodon

API URL Shortener application written in Rust

### Overview

This project will provide a REST API having a semi CRUD(not update) operations to managing urls and shortening them.

- POST /api/v1/surl
- GET /api/v1/surl/:id:
- DELETE /api/v1/surl/:id:

We will be working on one table at first. maybe later add a users to relate and create authentication and authorization?

## tech

- postgres

```sh
docker run --name bondedodon-db \
    -e POSTGRES_PASSWORD=password \
    -e POSTGRES_USER=postgres \
    -e POSTGRES_DB=bondedodon \
    -p 5432:5432 \
    -d postgres
```
