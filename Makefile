test:
	cargo test -- --nocapture

doc:
	cargo doc
	echo '<meta http-equiv=refresh content=0;url=asteroid/index.html>' > target/doc/index.html
	ghp-import -n target/doc
	cp -r target/doc docs
