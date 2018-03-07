#!/bin/sh

run_watch()
{
  if [ -z $1 ]; then
    echo "Watching main..."
    cargo watch -x "build" -s "RUST_BACKTRACE=1 ./target/debug/garage_rfid"
  else
    echo "Watching example $1..."
    cargo watch -x "build --example $1" -s "RUST_BACKTRACE=1 ./target/debug/examples/$1"
  fi
}

case $1 in
  "watch")
    run_watch $2
    ;;
  *)
    echo "Invalid option $1"
    ;;
esac
