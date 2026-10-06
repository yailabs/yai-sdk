.PHONY: check check-platform-parity check-finite-native check-finite-remote qualify-distribution
check:
	python3 tools/yvex_management.py --check
	python3 tools/workflows.py --check
	python3 tests/test_workflow_contract.py
	python3 tests/test_finite_remote_contract.py
	python3 tests/test_management_contract.py
	cargo test --locked --workspace --all-targets
	npm ci --ignore-scripts
	npm run check
	python3 tools/check.py
	python3 tools/docs.py
	python3 tests/test_package_identity.py
	python3 tests/test_distribution_legal.py

# Explicit multi-repository lane. Standalone SDK checks never require private clones.
check-platform-parity:
	@test -n "$(YAI_BIN)" -a -n "$(STUDIO_ROOT)" -a -n "$(YVEX_ROOT)" || { echo 'Set YAI_BIN, STUDIO_ROOT and YVEX_ROOT to reviewed local checkouts'; exit 1; }
	python3 tools/check_platform_parity.py --yai-bin "$(YAI_BIN)" --studio-root "$(STUDIO_ROOT)" --yvex-root "$(YVEX_ROOT)"

# Public headers supplied explicitly; isolated synthetic ABI peer, never inference.
check-management-remote:
	python3 tests/test_management_remote.py

check-finite-remote:
	python3 tests/test_finite_remote.py

check-finite-native:
	@test -n "$(YVEX_CLIENT_INCLUDE_DIR)" || { echo 'Set YVEX_CLIENT_INCLUDE_DIR to reviewed public installed headers'; exit 1; }
	python3 tests/test_finite_native.py --include-dir "$(YVEX_CLIENT_INCLUDE_DIR)"

# A successful software build is not binary distribution qualification.
qualify-distribution:
	@test -n "$(DISTRIBUTION_PACKAGE)" || { echo 'BLOCKED: specify a reviewed distribution.legal.v1 package'; exit 1; }
	python3 tools/distribution_legal.py verify "$(DISTRIBUTION_PACKAGE)" --policy tools/distribution_policy.json --first-party-license LICENSE --first-party-terms MIT
