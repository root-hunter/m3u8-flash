docker-build:
	./scripts/build.sh

docker-up: docker-build
	docker compose up

docker-start:
	./scripts/start.sh

engine-start:
	cd engine; cargo run