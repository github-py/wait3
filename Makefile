.PHONY: all clean cls release debug fix fmt check build test examples run-$(WAITFOR_NAME) run-$(SCANDISK_NAME)

INSTALL_PATH		:= $(HOME)/usr/bin/

WAITFOR_NAME		:=wait3
WAITFOR_DEBUG_BIN	:=target/debug/$(WAITFOR_NAME)
WAITFOR_RELEASE_BIN	:=target/release/$(WAITFOR_NAME)
WAITFOR_BIN		:=$(WAITFOR_DEBUG_BIN)
WAITFOR_RUN		:=cargo run --bin $(WAITFOR_NAME) --
all: test debug release

$(INSTALL_PATH):
	mkdir -p $@

$(WAITFOR_RELEASE_BIN): $(INSTALL_PATH)
	cargo build --release

$(WAITFOR_DEBUG_BIN): $(INSTALL_PATH)
	cargo build

run-$(WAITFOR_NAME):
	cargo run --bin $(subst run-,,$@)

e2e-$(WAITFOR_NAME):
	$(WAITFOR_RUN) time 4s

release: check fix | $(WAITFOR_RELEASE_BIN)
	install $(WAITFOR_RELEASE_BIN) $(INSTALL_PATH)

debug: check fix | $(WAITFOR_DEBUG_BIN)
	install $(WAITFOR_DEBUG_BIN) $(INSTALL_PATH)

clean: cls
	@rm -rf target

cls:
	-@reset || tput reset

fix:
	cargo fix --allow-dirty --allow-staged

fmt:
	rustfmt --edition 2021 src/*.rs

check:
	cargo check --all-targets

build test: check
	cargo $@

publish: release
	cargo $@
