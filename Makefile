# project name
OUT        = project

# Add the source files to be compiled here, without the .c extension or the src/ directory
_SRC       = call_rust


# -- DO NOT MODIFY -- #
MAKE 	   = make --no-print-directory 
CC         = gcc
CFLAGS     = -Wall -Wextra -Werror -g3 -O2 -std=c99
CPPFLAGS   = -Iinclude/
LDFLAGS    = -Llib/ -Wl,-rpath=./lib/ # set rpath so we don't have to use LD_LIBRARY_PATH
LDLIBS     = -lcrs
SRC        = $(addsuffix .c, $(addprefix src/, $(_SRC)))
OBJ        = $(SRC:.c=.o)
LIB_OUT    = libcrs.so
LIB_HEADER = crs.h

define colorecho
      @tput setaf 5
      @echo $1
      @tput sgr0
endef
# ------------------- #
.PHONY: all
all: build 

.PHONY: run
run: re
	./$(OUT)

.PHONY: clean
clean: lib_clean src_clean
	@$(call colorecho, "cleaned")

.PHONY: fclean
fclean: lib_fclean src_fclean lib_clean_header
	@$(call colorecho, "fully cleaned")

.PHONY: ffclean
ffclean: lib_ffclean src_fclean lib_clean_header
	@$(call colorecho, "fully fully cleaned")

.PHONY: re
re: fclean build

build: lib $(OUT)

$(OUT): obj_echo $(OBJ)
	@$(call colorecho, "compiling executable...")
	@${CC} $(OBJ) -o $(OUT) ${LDFLAGS} ${LDLIBS}
	@$(call colorecho, "Done!")

%.o: %.c
	@${CC} ${CFLAGS} ${CPPFLAGS} -c $< -o $@ ${LDFLAGS} ${LDLIBS}

lib: crs include_dir
	@cp lib/$(LIB_HEADER) include

crs:
	@$(MAKE) -C lib

include_dir: 
	@$(call colorecho, "creating include directory...")
	@mkdir -p include

src_clean:
	@$(RM) $(OBJ)

lib_clean:
	@$(MAKE) -C lib clean

src_fclean: src_clean
	@$(RM) $(OUT)

lib_fclean:
	@$(MAKE) -C lib fclean

lib_ffclean:
	@$(MAKE) -C lib ffclean

lib_clean_header:
	@$(RM) include/$(LIB_HEADER)

obj_echo:
	@$(call colorecho, "compiling object files...")
# ------------------- #
