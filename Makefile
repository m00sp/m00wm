all: build

.PHONY: build
build:
	$(shell [[ $EUID -eq 0 ]] && echo "build can not be run as root" && exit 1)
	@echo ":: Rebuilding in release mode..."
	@RUSTFLAGS="-C target-feature=-crt-static" cargo build --release
	@echo ":: Pronto"

.PHONY: test
test:
	@echo ":: testando o m00wm no Xephyr"
	./scripts/m00xephyr.sh

.PHONY: install
install:
	@echo ":: Instalando o binario m00wm"
	@cp -f /usr/local/bin/m00wm /usr/local/bin/m00wm.bak
	@cp -f target/release/m00wm /usr/local/bin
	@chmod 755 /usr/local/bin/m00wm
	@echo ":: Pronto"

.PHONY: uninstall
uninstall:
	@echo ":: Removing binaries..."
	@rm -f /usr/local/bin/penrose
	@echo ":: Pronto"
