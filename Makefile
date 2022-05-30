#!/usr/bin/make -f

GIR_EXEC = ./gir/target/release/gir

.PHONY: all clean test

all: hitaki-sys hitaki

clean:
	rm -rf gir-files/Hitaki-0.0.gir
	rm -rf hitaki-sys
	rm -rf hitaki/src/auto hitaki/target hitaki/Cargo.lock

gir/Cargo.toml:
	git submodule update --init gir

$(GIR_EXEC): gir/Cargo.toml
	cd gir ; cargo build --release ; cd -

gir-files/GLib-2.0.toml:
	git submodule update --init gir-files

gir-files/Hitaki-0.0.gir: Hitaki-0.0.gir gir-files/GLib-2.0.toml
	cp Hitaki-0.0.gir gir-files/Hitaki-0.0.gir

hitaki-sys/src: hitaki-sys/Gir.toml gir-files/Hitaki-0.0.gir $(GIR_EXEC)
	$(GIR_EXEC) -c hitaki-sys/Gir.toml -d gir-files -m sys -o hitaki-sys

hitaki-sys: hitaki-sys/src

hitaki/src/auto: hitaki/Gir.toml gir-files/Hitaki-0.0.gir $(GIR_EXEC)
	$(GIR_EXEC) -c hitaki/Gir.toml -d gir-files -m normal -o hitaki

hitaki: hitaki-sys hitaki/src/auto
