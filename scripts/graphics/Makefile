SOURCES=bb02_logo.png bb02_btc_logo.png bb02_multi_logo.png BitBoxBase_W96_H13_x16_y25.png

%.pbm: %.png
	convert -compress none -depth 1 $< $@

%.c: %.pbm
	python3 convert.py $< > $@.tmp
	mv $@.tmp $@

all: $(SOURCES:.png=.c)

clean:
	rm -f *.c
