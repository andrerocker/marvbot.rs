run:
	RUST_LOG=info cargo run

build: 
	cargo build

irc-server:
	docker run \
		--tty \
		--interactive \
		--publish 6667:6667 \
		inspircd/inspircd-docker

kafka-server:
	docker run \
		--tty \
		--interactive \
		--publish 9092:9092 \
		apache/kafka

test:
	cargo test