tmp=_git_distcheck

all:
	mkdir -p lib
	rust build src/rs2cl.rc -Lnalgebra/lib --opt-level=3 --out-dir lib

test:
	mkdir -p lib
	rustc -Lnalgebra/lib --test src/rs2cl.rc -o rs2cl~ && ./rs2cl~
	rm rs2cl~

deps:
	make -C nalgebra


distcheck:
	rm -rf $(tmp)
	git clone --recursive . $(tmp)
	make -C $(tmp) deps
	make -C $(tmp)
	make test -C $(tmp)
	rm -rf $(tmp)
