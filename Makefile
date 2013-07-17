all:
	mkdir -p lib
	rust build src/rs2cl.rc -Lnalgebra/lib -Lrust-opencl/ --opt-level=3 --out-dir lib

test:
	mkdir -p lib
	rustc -Lnalgebra/lib -Lrust-opencl/ --test src/rs2cl.rc -o rs2cl~ && ./rs2cl~
	rm rs2cl~

deps:
	make -C nalgebra
	make -C rust-opencl
