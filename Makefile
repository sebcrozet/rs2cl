tmp=_git_distcheck

all:
	mkdir -p lib
	rust build src/rs2cl.rc -Llib/nalgebra/lib --opt-level=3 --out-dir lib

examples: test

test:
	mkdir -p bin
	rust build --opt-level=3 --out-dir bin -L lib/nalgebra/lib examples/addition.rs

deps:
	make -C lib/nalgebra


distcheck:
	rm -rf $(tmp)
	git clone --recursive . $(tmp)
	make -C $(tmp) deps
	make -C $(tmp)
	make examples -C $(tmp)
	rm -rf $(tmp)

# FIXME: uggly
.PHONY: examples
.PHONY: test 
