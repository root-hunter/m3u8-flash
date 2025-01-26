docker-build:
	docker build -t m3u8-flash-engine ./engine
	docker build -t m3u8-flash-server ./server

docker-up: docker-build
	docker compose up

engine-start:
	cd engine; cargo run