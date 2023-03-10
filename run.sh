#!/bin/bash

SOURCE_FILE=$1
if [ "$SOURCE_FILE" = "" ]; then
  echo "引数にファイルを指定してください"
  echo "例: sudo ./run.sh ./src/abc001/a.rs"
  exit 1
fi

docker run \
  -it \
  --rm \
  -v "$PWD"/Cargo.toml:/usr/src/myapp/Cargo.toml:ro \
  -v "$PWD"/"$SOURCE_FILE":/usr/src/myapp/src/main.rs:ro \
  -w /usr/src/myapp \
  rust:1.42.0-slim \
  bash -c "cargo build --release --offline && echo '>>' &&./target/release/main"