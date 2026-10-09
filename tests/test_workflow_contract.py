"""Public contract generation is deterministic and refuses dangling type facts."""
import copy
import importlib.util
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('workflows', ROOT / 'tools/workflows.py')
generator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(generator)


class ContractTests(unittest.TestCase):
    def setUp(self):
        self.contract = json.loads((ROOT / 'contract/workflows.json').read_text())

    def test_checked_projections_are_exact(self):
        outputs = generator.render(self.contract)
        self.assertEqual(outputs, generator.render(copy.deepcopy(self.contract)))
        for path, result in zip(('src/workflows.rs', 'typescript/workflows.ts'), outputs):
            self.assertEqual((ROOT / path).read_text(), result)

    def test_unknown_type_refuses(self):
        self.contract['types']['CaseRefInput']['case_ref'] = 'UnknownPrivateState'
        with self.assertRaisesRegex(ValueError, 'unknown public type'):
            generator.render(self.contract)

    def test_operation_wrapper_type_collision_refuses(self):
        self.contract['operations'][0]['family'] = 'product'
        self.contract['operations'][0]['method'] = 'access'
        with self.assertRaisesRegex(ValueError, 'collides with public type'):
            generator.render(self.contract)

    def test_development_access_and_profile_are_explicit_not_entitlements(self):
        rust, ts = generator.render(self.contract)
        self.assertIn('pub development_capable: bool', rust)
        self.assertIn('pub development_enabled: bool', rust)
        self.assertIn('pub commercial_auth: Option<ProductAuthObservation>', rust)
        self.assertIn('"local_development"', ts)
        self.assertIn('"not_applicable"', ts)
        self.assertIn('pub display_name: Option<String>', rust)
        self.assertIn('pub email_verification: EmailVerificationPosture', rust)
        self.assertNotIn('Francesco', rust)

    def test_duplicate_operation_refuses(self):
        self.contract['operations'].append(self.contract['operations'][0])
        with self.assertRaisesRegex(ValueError, 'duplicate operation'):
            generator.render(self.contract)

    def test_input_variants_preserve_old_methods_and_exact_operation(self):
        rust, _ = generator.render(self.contract)
        self.assertIn('pub fn send_text(', rust)
        self.assertIn('pub fn send_with_search(', rust)
        self.assertIn('pub fn send_progressive(', rust)
        self.assertIn('pub fn get_context(', rust)
        self.assertEqual(rust.count('const ID: &\'static str = "conversation.send"'), 3)

    def test_input_variant_collision_and_unknown_type_refuse(self):
        op = next(op for op in self.contract['operations'] if op['id'] == 'conversation.send')
        op['input_variants'][0]['method'] = 'send_text'
        with self.assertRaisesRegex(ValueError, 'duplicate workflow method'):
            generator.render(self.contract)
        op['input_variants'][0]['method'] = 'send_with_search'
        op['input_variants'][0]['input'] = 'Missing'
        with self.assertRaisesRegex(ValueError, 'unknown public type'):
            generator.render(self.contract)

    def test_generated_privacy_types_remain_closed(self):
        rust, _ = generator.render(self.contract)
        self.assertIn('#[serde(deny_unknown_fields)]\npub struct WorkCommit', rust)
        self.assertIn('#[serde(deny_unknown_fields)]\npub struct CaseVersionProjection', rust)

    def test_optional_and_nullable_contract_is_explicit(self):
        rust, ts = generator.render(self.contract)
        self.assertIn('skip_serializing_if = "Option::is_none"', rust)
        self.assertIn('thread_ref?: string | null', ts)

    def test_closed_type_must_exist(self):
        self.contract['closed_types'].append('Missing')
        with self.assertRaisesRegex(ValueError, 'unknown closed type'):
            generator.render(self.contract)

    def test_product_policy_projects_typed_limits_not_marketing_logic(self):
        rust, ts = generator.render(self.contract)
        self.assertIn('pub limits: std::collections::BTreeMap<String, Option<u64>>', rust)
        self.assertIn('limits: Record<string, number | null>', ts)
        self.assertIn('pub policy: Option<ProductAccessPolicy>', rust)
        self.assertIn('"unsupported_policy"', ts)
        self.assertNotIn('2026-10-03', rust)  # Core, not SDK, admits revisions.

    def test_method_collision_refuses(self):
        operation = copy.deepcopy(self.contract['operations'][0])
        operation['id'] = 'different.operation'
        self.contract['operations'].append(operation)
        with self.assertRaisesRegex(ValueError, 'duplicate workflow method'):
            generator.render(self.contract)

    def test_wire_result_and_invalidation_have_one_authority(self):
        rust, ts = generator.render(self.contract)
        self.assertIn('pub enum ResultState', rust)
        self.assertIn('export type ResultState', ts)
        self.assertIn('pub struct CaseUpdate', rust)
        self.assertIn('export interface OperationResult<T = unknown>', ts)
        self.assertNotIn('pub struct CaseUpdate', (ROOT / 'src/contracts.rs').read_text())


if __name__ == '__main__':
    unittest.main()
