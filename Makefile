test:
	cargo test -- --nocapture
doc:
	cargo doc
	cp -rf target/doc doc
