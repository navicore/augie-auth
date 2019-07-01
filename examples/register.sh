#!/usr/bin/env bash

curl --request POST \
  --url http://localhost:3000/api/register/f87910d7-0e33-4ded-a8d8-2264800d1783 \
  --header 'content-type: application/json' \
  --data '{"password":"password"}'
