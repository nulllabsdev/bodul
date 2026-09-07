.PHONY: fmt test check check-strict docker-build docker-up docker-down gomd wip

fmt:
	$(MAKE) -C apps/mvp fmt
	$(MAKE) -C lib/money fmt
	$(MAKE) -C lib/shared fmt
	$(MAKE) -C lib/retailer-sourcing fmt

test:
	$(MAKE) -C apps/mvp test
	$(MAKE) -C lib/money test
	$(MAKE) -C lib/shared test
	$(MAKE) -C lib/retailer-sourcing test

check:
	$(MAKE) -C apps/mvp check
	$(MAKE) -C lib/money check
	$(MAKE) -C lib/shared check
	$(MAKE) -C lib/retailer-sourcing check

check-strict:
	RUSTFLAGS="-Awarnings" cargo check

docker-build:
	docker build -t bodul-mvp .

docker-up:
	docker compose up --build

docker-down:
	docker compose down

gomd:
	gomd all .

wip:
	git add . && git commit -am 'wip'
