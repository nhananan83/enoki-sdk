# Enoki Server

This serves as an example salt backup server implementation as part of the Enoki SDK. There are two endpoints supported: 
1. `GET get_pin/${id_token}`: This uses uses HKDF to derive unique salts for each user based on the JWT token. Upon verifying the JWT token is valid, the salt is derived as `HKDF(seed, sub, iss || aud)`.
2. `GET get_pin_and_id`: Alternatively, the server maintains an incremental counter and the endpoint can be called without the identifier, then a salt and an user_Id is assigned each time it is called. 

# Run locally

```bash
SEED=$SEED cargo run # $Seed here should be a 32-byte random value encoded in base64 e.g. head -c32 /dev/urandom | base64
```

# Curl locally


```bash
curl http://localhost:8000/get_pin/eyJhbGciOiJSUzI1NiIsImtpZCI6ImM5YWZkYTM2ODJlYmYwOWViMzA1NWMxYzRiZDM5Yjc1MWZiZjgxOTUiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiI1NzU1MTkyMDQyMzctbXNvcDllcDQ1dTJ1bzk4aGFwcW1uZ3Y4ZDg0cWRjOGsuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiI1NzU1MTkyMDQyMzctbXNvcDllcDQ1dTJ1bzk4aGFwcW1uZ3Y4ZDg0cWRjOGsuYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTA0NjM0NTIxNjczMDM1OTgzODMiLCJlbWFpbCI6IndhbmdxaWFveWkuam95QGdtYWlsLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJub25jZSI6IkpNaTZjXzNxWG4xSDhVWDVsYTFQNllEd1Roa041TFp4cWFnVHlqZmlZd1UiLCJpYXQiOjE2ODMzMjMyNjksImV4cCI6MTY4MzMyNjg2OSwianRpIjoiMDEzMzA2YjY1MmY0Zjg1MjUxZTU1OGVhNGFhOWJkYWI3ZmQxNzk3ZiJ9.TCI2XSbEmFc3KVHn2MoZi4OwCM56l59hiSZBdWwMeaCQWCbcJ87OhqtDTOuJMtUclBfkvEDoX_F2VhLJEbUcsFc5XyH_wrPjEqLet3uK1NB8Pqvuk1Q8lMy9nTvSCugGyCRUVhGiOiUfITwq8DhP-NPQ_2vp0NVb_EmHEUxgRniNA-h5JXK2RRxKb1Sx0-yAnerxAamNcvYCOL679Ig9u0N_G_v2cwUVYEm-8XkKpyrUeMv60euxMdO0LDCa1qbOj_l0OmPtMweCMGtVJOCrR3upZ443ttALJ2slsXdXc0Ee9byDoEP9KaPsvMT2ZQX3ZDss_ce3opYDd0snUf-H8A 

{"id":"0x0000000000000001","pin":"0x9ceaa18b2c318f67b7219192d1d227d8"}%
```

# Curl deployed server

Use https://enoki-server-7e33d356b89c.herokuapp.com instead of http://localhost:8000 
