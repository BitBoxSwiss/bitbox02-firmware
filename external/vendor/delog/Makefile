test: non-qemu-tests qemu-tests

non-qemu-tests: simple-tests delog-examples gate-tests

simple-tests:
	cargo test --lib
	cargo test --doc
	cargo test --examples

.PHONY: delog-examples
delog-examples:
	$(MAKE) -C delog-examples run

.PHONY: gate-tests
gate-tests:
	$(MAKE) -C gate-tests

.PHONY: qemu-tests
qemu-tests:
	$(MAKE) -C qemu-tests test

