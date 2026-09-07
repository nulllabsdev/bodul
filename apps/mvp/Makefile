.PHONY: fmt test check

fmt:
	cargo fmt

test:
	cargo test

check:
	cargo check

gomd:
	gomd all .

now:
	cargo run --bin dev destructure 100 -r ankercom
	cargo run --bin dev valueless 100 -r ankercom
	make dupes

nowr:
	cargo build --release --bin dev
	../../target/release/dev destructure 10000
	../../target/release/dev valueless 10000
	make dupes

# Requires fdupes. Note -dN deletes duplicates without prompting.
dupes:
	@for parent in data/offers-destructed data/offers-valueless data/offers-valueless-segments; do \
	  for retailer in "$$parent"/*/; do \
	    [ -d "$$retailer" ] || continue; \
	    printf '\n### %s\n' "$$retailer"; \
	    fdupes -rdNI "$$retailer"; \
	  done; \
	done
