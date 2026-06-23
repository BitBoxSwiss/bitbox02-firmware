PKG_CONFIG ?= pkg-config

FREETYPE_CFLAGS := $(shell $(PKG_CONFIG) --cflags freetype2 2>/dev/null)
FREETYPE_LIBS := $(shell $(PKG_CONFIG) --libs freetype2 2>/dev/null)
ICU_CFLAGS := $(shell $(PKG_CONFIG) --cflags icu-uc 2>/dev/null)
ICU_LIBS := $(shell $(PKG_CONFIG) --libs icu-uc 2>/dev/null)

ifeq ($(FREETYPE_CFLAGS),)
FREETYPE_CFLAGS := -I/usr/local/include/freetype2 -I/usr/include/freetype2 \
	-I/opt/homebrew/include/freetype2
endif

ifeq ($(FREETYPE_LIBS),)
FREETYPE_LIBS := -L/usr/local/lib -L/opt/homebrew/lib -lfreetype
endif

ifeq ($(ICU_LIBS),)
ICU_LIBS := -licuuc -licudata
endif

CFLAGS ?= -Wall -Wextra -g
CPPFLAGS += $(FREETYPE_CFLAGS) $(ICU_CFLAGS)
LDLIBS += $(FREETYPE_LIBS) $(ICU_LIBS)

OBJS = ttf2lvgl.o

.PHONY: all clean

all: ttf2lvgl

ttf2lvgl: $(OBJS)
	$(CC) $(LDFLAGS) -o $@ $(OBJS) $(LDLIBS)

clean:
	rm -f $(OBJS)
	rm -f ttf2lvgl
