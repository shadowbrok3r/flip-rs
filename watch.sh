#!/bin/sh


cargo watch -s 'cargo build --release && run-fap -p /dev/ttyACM0 target/thumbv7em-none-eabihf/release/flip-rs.fap'
storage -p /dev/ttyACM0 send target/thumbv7em-none-eabihf/release/flip-rs.fap /ext/apps/flip-rs.fap

