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
		apache/kafka:3.7.0

test:
	cargo test

compose:
	docker compose up

produce:
	 echo "hack3d" | kafka-console-producer.sh --bootstrap-server localhost:9092 --topic MARV.MESSAGES

consume:
	 kafka-console-consumer.sh \
	 	--bootstrap-server localhost:9092 \
		--topic MARV.MESSAGES  \
		--from-beginning

postgresql:
	docker run \
		--publish 5432:5432 \
		--env POSTGRES_PASSWORD=deploy42 \
		postgres

migrate:
	diesel migration run

setup: produce migrate