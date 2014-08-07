RUSTC ?= rustc
RUSTC_FLAGS ?=
NAME ?= irsc

SRC = $(shell find src -name '*.rs')

all: $(NAME)

$(NAME): $(SRC)
	mkdir -p target
	$(RUSTC) --out-dir target $(RUSTC_FLAGS) src/$(NAME).rs

opt: RUSTC_FLAGS += --opt-level=3 -Z lto
opt: $(NAME)

small: opt
	upx -9 ./target/$(NAME) 

debug: RUSTC_FLAGS += -g
debug: $(NAME)

run: $(NAME)
	./target/$(NAME)

test: $(SRC)
	mkdir -p target
	$(RUSTC) --test --out-dir target src/$(NAME).rs
	./target/$(NAME)

clean:
	@rm -rf target

.PHONY: clean
