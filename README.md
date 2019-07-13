[![Build Status](https://travis-ci.org/navicore/augie-auth.svg?branch=master)](https://travis-ci.org/navicore/augie-auth)

Augie Auth
======================

A POC of a Rust auth server that generates JWTs and Service Account Credentials.

Learning project gratefully bootstrapped from [this terriric tutorial](https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/).

# UNDER CONSTRuCtIoN
# UNDER CONSTRuCtIoN
# UNDER CONSTRuCtIoN


## Local Dev

```console
docker run -p 5432:5432--name some-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres

echo "DATABASE_URL=postgres://postgres:mysecretpassword@192.168.0.46/augie_auth" >> .env

diesel setup
```
