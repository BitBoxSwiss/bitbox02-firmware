PKG_CONFIG ?= pkg-config

FREETYPE_CFLAGS := $(shell $(PKG_CONFIG) --cflags freetype2)
FREETYPE_LIBS := $(shell $(PKG_CONFIG) --libs freetype2)
ICU_CFLAGS := $(shell $(PKG_CONFIG) --cflags icu-uc)
ICU_LIBS := $(shell $(PKG_CONFIG) --libs icu-uc)

CFLAGS ?= -Wall -Wextra -g
CPPFLAGS += $(FREETYPE_CFLAGS) $(ICU_CFLAGS)
LDLIBS += $(FREETYPE_LIBS) $(ICU_LIBS)

OBJS = ttf2lvgl.o

.PHONY: all clean test

all: ttf2lvgl

ttf2lvgl: $(OBJS)
	$(CC) $(LDFLAGS) -o $@ $(OBJS) $(LDLIBS)

test: ttf2lvgl
	./test.sh

clean:
	rm -f $(OBJS)
	rm -f ttf2lvgl
