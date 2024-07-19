#!/bin/bash
cd "$(dirname "$0")"
git pull
cargo run -r -- 1
sudo shutdown now
