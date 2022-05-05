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
	@cp ./target/debug/$(OUT_STATIC) ./
	@cp ./target/debug/$(OUT_DYNAMIC) ./
	@$(call colorecho, "generating header file...")
	@cbindgen -q > $(HEADER)

build:
	@$(call colorecho, "compiling rust library...")
	@(cargo build)

test: 
	@(cargo test)

clean: 
	@(cargo clean)

fclean: clean
	@$(RM) $(OUT_STATIC)
	@$(RM) $(OUT_DYNAMIC)
	@$(RM) $(HEADER)