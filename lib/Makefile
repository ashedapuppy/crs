OUT = libcrs
OUT_STATIC = $(OUT).a
OUT_DYNAMIC = $(OUT).so

HEADER = crs.h

define colorecho
      @tput setaf 5
      @echo $1
      @tput sgr0
endef

all: build
	@cp ./target/release/$(OUT_STATIC) ./
	@cp ./target/release/$(OUT_DYNAMIC) ./

build:
	@$(call colorecho, "compiling rust library...")
	@(cargo build --release --quiet)
	@(cargo test --quiet --features c-headers -- generate_headers --nocapture)

test: 
	@(cargo test)

fclean:
	@(cargo clean)
	@$(RM) $(OUT_STATIC)
	@$(RM) $(OUT_DYNAMIC)
	@$(RM) $(HEADER)