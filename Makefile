YEAR=2023

# Aliases for executables
CARGO ?= cargo
CODE ?= code
CURL ?= curl
GH ?= gh
GIT ?= git
RM ?= rm
XDG_OPEN ?= xdg-open

CURRENT_DIR := $(shell basename $(shell pwd))

ifeq ($(YEAR),$(CURRENT_DIR))
	ifeq (,$(DAY))
		DAY=$(shell printf "%02d" $(shell expr $(shell find . -maxdepth 1 -type d -name 'day-[0-9][0-9]' | sort -r | head -n 1 | sed -e 's/[^0-9]//g') + 1))
	endif
else
	ifeq (,$(DAY))
		DAY=$(shell pwd | sed 's/.*day-\([0-9][0-9]\).*/\1/')
	endif
endif

ifndef DAY
$(error DAY is not set)
endif

ifneq (,$(wildcard day-$(DAY)))
$(error day-$(DAY) already exists)
endif

NONZERO_DAY=$(shell echo $(DAY) | sed 's/^0*//')

.PHONY: debug
debug:
	@echo "CURRENT_DIR: $(CURRENT_DIR)"
	@echo "DAY: $(DAY)"
	@echo "NONZERO_DAY: $(NONZERO_DAY)"
	@echo "YEAR: $(YEAR)"

.PHONY: new
new:
	$(GIT) checkout -b feat/day-$(DAY)
	$(CARGO) new day-$(DAY) --vcs none
	$(RM) -rf day-$(DAY)/src/main.rs
	cp ./boilerplate.rs day-$(DAY)/src/main.rs
	echo "-include ../Makefile" >> day-$(DAY)/Makefile
	$(CURL) --silent --cookie "$$SESSION_COOKIE" --output day-$(DAY)/input.txt https://adventofcode.com/$(YEAR)/day/$(NONZERO_DAY)/input
	cd day-$(DAY) && $(CARGO) run >/dev/null 2>&1 || exit 0
	$(GIT) add .
	$(GIT) commit -am 'Add day $(DAY) boilerplate'
	$(CODE) --reuse-window day-$(DAY)/src/main.rs

.PHONY: test
test:
	$(CARGO) test

# Get code coverage
.PHONY: coverage
coverage:
	$(CARGO) tarpaulin -v --fail-under=100

# Build coverage report
.PHONY: coverage-report
coverage-report:
	$(CARGO) tarpaulin -v --fail-under=100 --out HTML; $(XDG_OPEN) tarpaulin-report.html

# Remove any built artifacts
.PHONY: clean
clean:
	$(RM) -rf target
	$(RM) -rf Cargo.lock
	$(RM) -rf tarpaulin-report.html
	$(RM) -rf main

# Submit the solution
.PHONY: submit
submit:
	$(GIT) diff-index --quiet HEAD || (echo "Uncommitted changes"; exit 1)
	echo "You need to code this"

# Finish the branch
.PHONY: finish
finish: coverage submit clean
	$(GIT) push -u origin feat/day-$(DAY)
	$(GH) pr create --fill
	$(GH) pr merge --merge --delete-branch
