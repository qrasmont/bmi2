.PHONY: check-async-sync

check-async-sync:
	cargo fmt --check
	cat src/interface_async.rs | sed -e 's/[.]await//' -e 's/async //' -e 's/_async//' | grep -vF '#[allow(async_fn_in_trait)]' | rustfmt > /tmp/stripped-interface_async.rs
	diff -u /tmp/stripped-interface_async.rs src/interface.rs
	cat src/bmi2_async.rs | sed -e 's/[.]await//' -e 's/async //' -e 's/_async//' | grep -vF '#[allow(async_fn_in_trait)]' | rustfmt > /tmp/stripped-bmi2_async.rs
	diff -u /tmp/stripped-bmi2_async.rs src/bmi2.rs
