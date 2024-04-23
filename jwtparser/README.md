# JWTParser

Simple jwt parsing tool. Useful when you want the check the content of your
JWT token without using jwt.io.

The code is pretty simple and uses one single file, you can check the whole structure
to be asure there is absolutelly no risk of using this tool.

## How-to

Run

```bash
cargo run -- --file mytoken.txt
```

```json
Header: {
  "alg": "HS256",
  "typ": "JWT"
},
Payload: {
  "iat": 1516239022,
  "name": "John Doe",
  "sub": "1234567890"
},
```
