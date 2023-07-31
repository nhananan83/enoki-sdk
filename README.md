# Enoki Server

This serves as an example PIN registry server implementation as part of the Enoki SDK. By calling the 

It uses HKDF to derive unique PINs for each user assuming the user identifier is ensured unique, alternatively it can be called without the identifier, then an unique identifier is assigned as a incremental counter. 

# Run

cargo run --seed $SEED

# Call



