OUT = libcrs
OUT_STATIC = $(OUT).a
OUT_DYNAMIC = $(OUT).so

HEADER = crs.h

all: build
	@cp ./target/debug/$(OUT_STATIC) ./
	@cp ./target/debug/$(OUT_DYNAMIC) ./
	@echo "generating header file..."
	@cbindgen > $(HEADER)

build:
	@echo "compiling rust library..."
	@(cargo build)

test: 
	@(cargo test)

clean: 
	@(cargo clean)

fclean: clean
	$(RM) $(OUT_STATIC)
	$(RM) $(OUT_DYNAMIC)
	$(RM) $(HEADER)