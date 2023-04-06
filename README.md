Using the tool
Run ./hello-from-rust.sh This contains the following query:

cargo lambda invoke --remote \
  --data-ascii '{"name": "hello from rust"}' \
  --output-format json \
  rust-lambda-handshake
And we get back:

{
"msg": "hello from aws lambda!",
"req_id": "61e53964-68fa-445f-86d8-5f5d4a28edd9"
}
However, if we run ./huh.sh, which contains:

cargo lambda invoke --remote \
  --data-ascii '{"name": "hello"}' \
  --output-format json \
  rust-lambda-handshake
We get back:

{
  "msg": "huh?",
  "req_id": "22bd9a8a-3e62-4f8f-b8f2-4f9dd84720d0"
}
