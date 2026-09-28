.PHONY: check qualify-distribution
check:
	cargo test --locked
	npm ci --ignore-scripts
	npm run check
	python3 tools/check.py
	python3 tests/test_package_identity.py
	python3 tests/test_distribution_legal.py

# A successful software build is not binary distribution qualification.
qualify-distribution:
	@test -n "$(DISTRIBUTION_PACKAGE)" || { echo 'BLOCKED: specify a reviewed distribution.legal.v1 package'; exit 1; }
	python3 tools/distribution_legal.py verify "$(DISTRIBUTION_PACKAGE)" --policy tools/distribution_policy.json --first-party-license LICENSE --first-party-terms MIT
