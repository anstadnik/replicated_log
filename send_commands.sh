curl --location --request POST 'localhost:7878' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "msg": "potato"
}'
curl --location --request GET 'localhost:7878' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
    "msg": "potato"
}'
