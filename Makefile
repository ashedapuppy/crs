MAKE 	   = make --no-print-directory 
CC         = gcc
CFLAGS     = -Wall -Wextra -Werror -g3 -O2 -std=c99
CPPFLAGS   = -Iinclude/
LDFLAGS    = -Llib/ 
LDLIBS     = -lcrs

OUT        = project

# Add the source files to be compiled here, without the .c extension or the src/ directory
_SRC       = call_rust

SRC        = $(addsuffix .c, $(addprefix src/, $(_SRC)))
OBJ        = $(SRC:.c=.o)

LIB_OUT    = libcrs.so
LIB_HEADER = crs.h


all: build 

run: re
	
build: lib $(OUT)

$(OUT): obj_echo $(OBJ)
	@echo "compiling executable..."
	@${CC} $(OBJ) -o $(OUT) ${LDFLAGS} ${LDLIBS}
	@echo "Done!"

lib: crs include_dir
	@cp lib/$(LIB_HEADER) include

clean: lib_clean src_clean

fclean: lib_fclean src_fclean

re: fclean build

obj_echo:
	@echo "compiling object files..."

%.o: %.c
	@${CC} ${CFLAGS} ${CPPFLAGS} -c $< -o $@ ${LDFLAGS} ${LDLIBS}

crs:
	@$(MAKE) -C lib

include_dir: 
	@echo "creating include directory..."
	@mkdir -p include

lib_clean: lib_clean_header
	@$(MAKE) -C lib clean

lib_fclean: lib_clean_header
	@$(MAKE) -C lib fclean

lib_clean_header:
	$(RM) include/$(LIB_HEADER)

src_clean:
	$(RM) $(OBJ)

src_fclean: src_clean
	$(RM) $(OUT)
