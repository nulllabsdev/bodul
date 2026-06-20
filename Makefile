.PHONY: fmt test check

fmt:
	$(MAKE) -C apps/mvp fmt
	$(MAKE) -C lib/money fmt
	$(MAKE) -C lib/shared fmt

test:
	$(MAKE) -C apps/mvp test
	$(MAKE) -C lib/money test
	$(MAKE) -C lib/shared test

check:
	$(MAKE) -C apps/mvp check
	$(MAKE) -C lib/money check
	$(MAKE) -C lib/shared check
