all:

clean:
	@cargo clean
	@rm -rf $$PWD/programs/primary/target

build:
	@cargo-build-sbf --tools-version v1.54 --debug --arch v1

test:
	@SBF_OUT_DIR=$$PWD/target/deploy/debug \
	SBF_TRACE_DIR=$$PWD/target/sbf/trace \
	cargo test -p primary --features no-entrypoint --test cpi -- --nocapture

debug:
	@SBF_OUT_DIR=$$PWD/target/deploy/debug \
	SBF_DEBUG_PORT=1212 \
	SBF_TRACE_DIR=$$PWD/target/sbf/trace \
	cargo test -p primary --features no-entrypoint --test cpi -- --nocapture
