#!/usr/bin/env python3
"""SDK Markdown profile: unique owners, hidden metadata and resolvable local routes."""
import re
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
pages = [ROOT / 'README.md', *sorted((ROOT / 'docs').rglob('*.md'))]
ids = set()
incoming = set()
for page in pages:
    text = page.read_text()
    metadata = re.match(r'<!-- docs:metadata\n(.*?)\n-->', text, re.S)
    assert metadata, f'missing hidden metadata: {page}'
    identity = re.search(r'^id: (.+)$', metadata[1], re.M)
    assert identity and identity[1] not in ids, f'missing/duplicate ID: {page}'
    ids.add(identity[1])
    for field in ('title', 'document', 'status', 'owner', 'audience', 'publication'):
        assert re.search(rf'^{field}: .+', metadata[1], re.M), (page, field)
    assert not text.startswith('---'), f'visible metadata: {page}'
    prose = re.sub(r'```.*?```', '', text, flags=re.S)
    for target in re.findall(r'\]\(([^)]+)\)', prose):
        if re.match(r'[a-z]+:', target):
            continue
        path, _, anchor = target.partition('#')
        destination = (page.parent / path).resolve() if path else page
        assert destination.exists(), f'broken link: {page}: {target}'
        incoming.add(destination)
        if anchor and destination.suffix == '.md':
            headings = re.findall(r'^#+ (.+)$', destination.read_text(), re.M)
            anchors = {re.sub(r'[^\w\- ]', '', h.lower()).replace(' ', '-') for h in headings}
            assert anchor in anchors, f'broken anchor: {page}: {target}'
assert all(page.resolve() in incoming for page in pages), 'orphan document'
assert '| YAI.SDK.PLATFORM.0 |' in (ROOT / 'docs/TASKS.md').read_text()
print(f'PASS: {len(pages)} documentation owners, metadata, links, anchors and reachability')
