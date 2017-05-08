#!/bin/sh
while true; do
  killall zentner-io-rocket
  cargo run &
  PID=$!
  inotifywait -r -e modify templates
  kill $PID
  sleep 1
done
