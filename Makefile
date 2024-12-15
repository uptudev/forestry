.DELETE_ON_ERROR:

INSTALL_DIR:=/usr/local/bin

forestry:
	@clang -I include -c -o libforestry.o src/forestry.c -fPIC
	@ar r libforestry.a libforestry.o
	@clang -shared -o libforestry.so libforestry.o

test:
	@clang -I include src/**/*.c -o test
	@./test

.PHONY: install
install: ctr
	install -m 755 ./ctr ${INSTALL_DIR}

.PHONY: clean
clean:
	@rm -f ctr
