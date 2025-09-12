
# When you've made changes to the async version, updating the sync version is trivial
.PHONY: async-to-sync
async-to-sync:
	cat src/interface_async.rs | sed -e 's/[.]await//' -e 's/async //' -e 's/_async//' | grep -vF '#[allow(async_fn_in_trait)]' | rustfmt > src/interface.rs
	cat src/bmi2_async.rs | sed -e 's/[.]await//' -e 's/async //' -e 's/_async//' | grep -vF '#[allow(async_fn_in_trait)]' | rustfmt > src/bmi2.rs

# When you've made changes to the sync version, updating the async version is not always as clear
# because there are many possible places to insert async and await, so the best we can do is check.
.PHONY: check-async-sync
check-async-sync:
	# Everything must be formatted before we start, because the code might reflow differently in
	# sync vs async versions. For local dev we do it automatically. For CI we just check it.
	if [ "${CI}" == "" ]; then cargo fmt; fi
	cargo fmt --check
	# Strip out the async-related keywords, then format, then diff against the sync version.
	cat src/interface_async.rs | sed -e 's/[.]await//' -e 's/async //' -e 's/_async//' | grep -vF '#[allow(async_fn_in_trait)]' | rustfmt > /tmp/stripped-interface_async.rs
	diff -u /tmp/stripped-interface_async.rs src/interface.rs
	cat src/bmi2_async.rs | sed -e 's/[.]await//' -e 's/async //' -e 's/_async//' | grep -vF '#[allow(async_fn_in_trait)]' | rustfmt > /tmp/stripped-bmi2_async.rs
	diff -u /tmp/stripped-bmi2_async.rs src/bmi2.rs
	# Make sure everything actually compiles once we're done.
	cargo check --all-features
