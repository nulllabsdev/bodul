.PHONY: fmt test check

fmt:
	$(MAKE) -C apps/mvp fmt

test:
	$(MAKE) -C apps/mvp test

check:
	$(MAKE) -C apps/mvp check
