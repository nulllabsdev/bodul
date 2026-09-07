.PHONY: fmt test check cargo-fix clippy-fix check-strict docker-build docker-up docker-down gomd wip

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

cargo-fix:
	cd apps/mvp && cargo fix --tests
	cd lib/money && cargo fix --tests
	cd lib/shared && cargo fix --tests
	cd lib/retailer-sourcing && cargo fix --tests

clippy-fix:
	cd apps/mvp && cargo clippy --fix --tests
	cd lib/money && cargo clippy --fix --tests
	cd lib/shared && cargo clippy --fix --tests
	cd lib/retailer-sourcing && cargo clippy --fix --tests

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
