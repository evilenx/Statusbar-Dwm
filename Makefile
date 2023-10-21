# Nombre del binario que se generará
BIN_NAME = statusbar

# Directorio de instalación (puedes cambiarlo según tus necesidades)
INSTALL_DIR = /usr/local/bin

all: build

build:
	cargo build --release

install: build
	cp target/release/$(BIN_NAME) $(INSTALL_DIR)

uninstall:
	rm $(INSTALL_DIR)/$(BIN_NAME)

clean:
	cargo clean

.PHONY: all build install uninstall clean

