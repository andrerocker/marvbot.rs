run:
	RUST_BACKTRACE=1 RUST_LOG=info cargo run

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
	cargo watch -x test

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
	DATABASE_URL=postgres://deploy42:deploy42@localhost:5432/deploy42 \
	DIESEL_CONFIG_FILE=marv_plugins/diesel.toml \
		diesel migration run

setup: produce migrate

fix:
	cargo fix --allow-dirty
