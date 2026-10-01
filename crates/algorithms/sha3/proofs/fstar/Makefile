JOBS ?= $(shell nproc --all 2>/dev/null || echo 4)

.PHONY: all extraction equivalence clean

all: extraction equivalence

extraction:
	$(MAKE) -C extraction -j $(JOBS)

equivalence:
	$(MAKE) -C equivalence -j $(JOBS)

clean:
	$(MAKE) -C extraction clean
	$(MAKE) -C equivalence clean
