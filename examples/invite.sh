curl --request POST \
  --url http://localhost:3000/api/invitation \
  --header 'content-type: application/json' \
  --data '{"email":"ed@onextent.com"}'
# dbg! will print something like this in your terminal where you are runnig the app
# {
#     "id": "67a68837-a059-43e6-a0b8-6e57e6260f0d",
#     "email": "test@test.com",
#     "expires_at": "2018-10-23T09:49:12.167510"
# }
