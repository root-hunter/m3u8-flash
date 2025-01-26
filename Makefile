docker-build:
	./scripts/build.sh

docker-up: docker-build
	docker compose up

docker-start:
	./scripts/start.sh

engine-start:
	cd engine; cargo run

cli-run:
	cd cli; cargo run -- $(ARGS)

cli-test-1:
	make cli-run \
	ARGS="\
	-p 'https://vixcloud.co/playlist/279721?b=1&token=8d562733677358e43c70f0bbf3d3cada&expires=1843100145' \
	-o '/mnt/07278d6f-dcd5-4540-ae3f-dc7f08c050e4/Dev/m3u8-flash/engine/generated/acab/ep_6.mp4'"