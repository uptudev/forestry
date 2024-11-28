INSTALL_DIR:=/usr/local/lib
HEADER_DIR:=/usr/local/include

forestry: --cleandebug
	@zig build-lib src/forestry.zig
	@zig build-lib src/forestry.zig -dynamic
	@rm -f ./*.o ./*.obj
	@mkdir -p out/debug
	@-mv -t out/debug ./*.so ./*.dll ./*.dylib ./*.wasm 2>/dev/null || true
	@-mv -t out/debug ./*.a ./*.lib 2>/dev/null || true
	@cp ./forestry.h ./out/

release: --cleanrelease
	@read -rp "Enter version: " F_VERSION; \
	zig build-lib src/forestry.zig --name forestry_"$$F_VERSION"_windows_x86-64_gnu -target x86_64-windows-gnu -O ReleaseSmall -static; \
	zig build-lib src/forestry.zig --name forestry_"$$F_VERSION"_windows_x86-64_gnu -target x86_64-windows-gnu -O ReleaseSmall -dynamic; \
	zig build-lib src/forestry.zig --name forestry_"$$F_VERSION"_windows_x86-64_msvc -target x86_64-windows-msvc -O ReleaseSmall -static; \
	zig build-lib src/forestry.zig --name forestry_"$$F_VERSION"_windows_x86-64_msvc -target x86_64-windows-msvc -O ReleaseSmall -dynamic; \
	zig build-lib src/forestry.zig --name forestry_"$$F_VERSION"_linux_x86-64_gnu -target x86_64-linux-gnu -O ReleaseSmall -static; \
	zig build-lib src/forestry.zig --name forestry_"$$F_VERSION"_linux_x86-64_gnu -target x86_64-linux-gnu -O ReleaseSmall -dynamic; \
	zig build-lib src/forestry.zig --name forestry_"$$F_VERSION"_linux_x86-64_musl -target x86_64-linux-musl -O ReleaseSmall -static; \
	zig build-lib src/forestry.zig --name forestry_"$$F_VERSION"_linux_x86-64_musl -target x86_64-linux-musl -O ReleaseSmall -dynamic; \
	zig build-lib src/forestry.zig --name forestry_"$$F_VERSION"_macos_x86-64 -target x86_64-macos-none -O ReleaseSmall -static; \
	zig build-lib src/forestry.zig --name forestry_"$$F_VERSION"_macos_x86-64 -target x86_64-macos-none -O ReleaseSmall -dynamic; \
	zig build-lib src/forestry.zig --name forestry_"$$F_VERSION"_macos_aarch64 -target aarch64-macos-none -O ReleaseSmall -static; \
	zig build-lib src/forestry.zig --name forestry_"$$F_VERSION"_macos_aarch64 -target aarch64-macos-none -O ReleaseSmall -dynamic; \
	zig build-lib src/forestry.zig --name forestry_"$$F_VERSION"_wasm32 -target wasm32-wasi-musl -O ReleaseSmall -static; \
	zig build-lib src/forestry.zig --name forestry_"$$F_VERSION"_wasm32 -target wasm32-wasi-musl -O ReleaseSmall -dynamic; \
	rm -f ./*.o ./*.obj; \
	mkdir -p ./out/release/x86_64/linux/gnu ./out/release/x86_64/linux/musl ./out/release/x86_64/windows/gnu ./out/release/x86_64/windows/msvc ./out/release/x86_64/macos ./out/release/aarch64/macos ./out/release/wasm32; \
	mv -t ./out/release/x86_64/linux/gnu ./libforestry_*_linux_x86-64_gnu.* 2>/dev/null; \
	mv -t ./out/release/x86_64/linux/musl ./libforestry_*_linux_x86-64_musl.* 2>/dev/null; \
	mv -t ./out/release/x86_64/windows/gnu ./forestry_*_windows_x86-64_gnu.* 2>/dev/null; \
	mv -t ./out/release/x86_64/windows/msvc ./forestry_*_windows_x86-64_msvc.* 2>/dev/null; \
	mv -t ./out/release/x86_64/macos ./libforestry_*_macos_x86-64.* 2>/dev/null; \
	mv -t ./out/release/aarch64/macos ./libforestry_*_macos_aarch64.* 2>/dev/null; \
	mv -t ./out/release/wasm32 ./forestry_*_wasm32.* ./libforestry_*_wasm32.* 2>/dev/null; \
	tar -czf ./out/release/forestry_"$$F_VERSION"_linux_x86-64_gnu.tar.gz --add-file=./forestry.h -C ./out/release/x86_64/linux/gnu .; \
	tar -czf ./out/release/forestry_"$$F_VERSION"_linux_x86-64_musl.tar.gz --add-file=./forestry.h -C ./out/release/x86_64/linux/musl .; \
	tar -czf ./out/release/forestry_"$$F_VERSION"_windows_x86-64_gnu.tar.gz --add-file=./forestry.h -C ./out/release/x86_64/windows/gnu .; \
	tar -czf ./out/release/forestry_"$$F_VERSION"_windows_x86-64_msvc.tar.gz --add-file=./forestry.h -C ./out/release/x86_64/windows/msvc .; \
	tar -czf ./out/release/forestry_"$$F_VERSION"_macos_x86-64.tar.gz --add-file=./forestry.h -C ./out/release/x86_64/macos .; \
	tar -czf ./out/release/forestry_"$$F_VERSION"_macos_aarch64.tar.gz --add-file=./forestry.h -C ./out/release/aarch64/macos .; \
	tar -czf ./out/release/forestry_"$$F_VERSION"_wasm32.tar.gz --add-file=./forestry.h -C ./out/release/wasm32 .
	@rm -r ./out/release/aarch64
	@rm -r ./out/release/wasm32
	@rm -r ./out/release/x86_64

build: forestry release

.PHONY: clean
clean:
	@rm -rf ./out

.PHONY: install
install: forestry
	install -m755 ./out/debug/libforestry.so ${INSTALL_DIR}
	install -m644 ./forestry.h ${HEADER_DIR}

--cleandebug:
	@rm -rf ./out/debug/*

--cleanrelease:
	@rm -rf ./out/release/*
