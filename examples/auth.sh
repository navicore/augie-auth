#!/usr/bin/env bash

curl -i --request POST \
  --url http://localhost:3000/api/auth \
  --header 'content-type: application/json' \
  --data '{"email": "ed@onextent.com","password":"secret"}'
