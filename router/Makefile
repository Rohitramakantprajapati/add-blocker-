PKG_NAME := voidblock
PKG_VERSION := 0.1.0

all:
	@echo "VoidBlock OpenWRT packaging stub"

package:
	@mkdir -p build
	@tar -czf build/$(PKG_NAME)-$(PKG_VERSION).tar.gz files luci-app-voidblock

clean:
	@rm -rf build

.PHONY: all package clean
PKGNAME := voidblock
VERSION := 0.1
DEST := build/$(PKGNAME)-$(VERSION)

all: package

package:
	@echo "Creating minimal OpenWRT package layout"
	rm -rf $(DEST)
	mkdir -p $(DEST)/control $(DEST)/usr/bin
	cp -a ../platform/linux/install.sh $(DEST)/usr/bin/voidblock-install.sh
	cp -a files/usr/bin/voidblock $(DEST)/usr/bin/voidblock
	@echo "Package assembled at $(DEST)"
	tar -czf $(PKGNAME)-$(VERSION).tar.gz -C build $(PKGNAME)-$(VERSION)
	@echo "Packed $(PKGNAME)-$(VERSION).tar.gz"

clean:
	rm -rf build *.tar.gz
