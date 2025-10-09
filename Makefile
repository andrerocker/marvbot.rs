run:
	cargo run

build: 
	cargo build

server:
	docker run \
		--tty \
		--interactive \
		--publish 6667:6667 \
		inspircd/inspircd-docker
