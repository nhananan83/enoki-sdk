# Enoki Server

This serves as an example PIN registry server implementation as part of the Enoki SDK. By calling the 

Under the hood, it uses HKDF to derive unique PINs for each user assuming the user identifier is  unique. Alternatively, the server maintains an incremental counter and the endpoint can be called without the identifier, an unique identifier is assigned each time it is called. 

# Run locally

```bash
SEED=$SEED cargo run # $Seed here should be a 32-byte random value encoded in base64 e.g. head -c32 /dev/urandom | base64
```

# Curl


```bash
curl http://localhost:8000/get_pin_and_id {"id":"0x0000000000000001","pin":"0x9ceaa18b2c318f67b7219192d1d227d8"}%

curl http://localhost:8000/get_pin_and_id 
{"id":"0x0000000000000002","pin":"0xa25671aabc23cd451ecadc6c55d77763"}%

curl http://localhost:8000get_pin_and_id 
{"id":"0x0000000000000003","pin":"0xd27bff7e9e2cd4d520d4aad73cf1f03b"}%


curl http://localhost:8000/get_pin/joy
{"id":"0x6a6f79","pin":"0x69f4bbc615054d2ae5a60a6d92bcee04"}%

curl http://localhost:8000/get_pin/joy
{"id":"0x6a6f79","pin":"0x69f4bbc615054d2ae5a60a6d92bcee04"}%

curl http://localhost:8000/get_pin/pavlos
{"id":"0x7061766c6f73","pin":"0x09b918e2a7958461bd85b895d721fe3f"}%
```

# Live

Use https://enoki-server-7e33d356b89c.herokuapp.com instead of http://localhost:8000 
