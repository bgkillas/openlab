run:
    cargo run
run_rel:
    cargo run --release
run_full:
    cargo run --profile release_full
build:
    cargo build
build_rel:
    cargo build --release
build_full:
    cargo build --profile release_full
miri:
    cargo miri test -- --nocapture --test-threads=1
test:
    cargo test -- --nocapture --test-threads=1
test_rel:
    cargo test --release -- --nocapture --test-threads=1
bench:
    cargo bench --lib --quiet -- --color always --test-threads=1 --nocapture
clippy:
    cargo fmt
    cargo clippy
update:
    cargo upgrade --incompatible
    cargo update
