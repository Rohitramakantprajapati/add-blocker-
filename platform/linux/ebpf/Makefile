
CLANG ?= clang
CFLAGS := -O2 -g -target bpf -Wall

all: xdp_filter.o tc_filter.o

xdp_filter.o: xdp_filter.c
	$(CLANG) $(CFLAGS) -c $< -o $@

tc_filter.o: tc_filter.c
	$(CLANG) $(CFLAGS) -c $< -o $@

install: all
	mkdir -p /etc/voidblock
	cp -f xdp_filter.o /etc/voidblock/xdp_filter.o

clean:
	rm -f *.o

.PHONY: all clean install
