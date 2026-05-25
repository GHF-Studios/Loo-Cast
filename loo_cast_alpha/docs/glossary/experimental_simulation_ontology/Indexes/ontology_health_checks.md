---
canonical_name: Ontology Health Checks (Experimental)
status: WIP-experimental
aliases: []
source_of_truth: []
---

These checks are run-level commands for coherence auditing.

## Core Structural Checks

1. Node count and class distribution:

```bash
find loo_cast_alpha/docs/glossary/experimental_simulation_ontology/Nodes -type f | wc -l
rg '^node_class:' loo_cast_alpha/docs/glossary/experimental_simulation_ontology/Nodes/*.md | awk '{print $2}' | sort | uniq -c
```

2. Unchecked seed entries:

```bash
rg -n '\- \[ \] `' loo_cast_alpha/docs/glossary/experimental_simulation_ontology/Coverage/universe_seed_registry.md
```

3. Trait-module closure (spot-check pattern):

```bash
for t in variational stochastic nonlocal multi_scale structure_preserving discontinuity_handling long_range; do
  rg -l "^  - $t$" loo_cast_alpha/docs/glossary/experimental_simulation_ontology/Nodes/*.md | while read -r f; do
    rg -q "^  - trait\\.$t$" "$f" || echo "MISSING_MODULE $f trait.$t"
  done
done
```

4. Type-module closure (spot-check pattern):

```bash
for nt in PDE stochastic_process solver transform morphism; do
  rg -l "^  - $nt$" loo_cast_alpha/docs/glossary/experimental_simulation_ontology/Nodes/*.md | while read -r f; do
    rg -q "^  - type\\.$nt$" "$f" || echo "MISSING_MODULE $f type.$nt"
  done
done
```

## Link Integrity Check

```bash
python - <<'PY'
import re,os,glob
base='loo_cast_alpha/docs/glossary/experimental_simulation_ontology'
files=glob.glob(base+'/**/*.md',recursive=True)
missing=[]
for f in files:
    txt=open(f,'r',encoding='utf-8').read()
    for m in re.finditer(r'\\]\\(([^)]+\\.md)\\)',txt):
        link=m.group(1)
        if link.startswith('http'): continue
        target=os.path.normpath(os.path.join(os.path.dirname(f),link))
        if not os.path.exists(target):
            missing.append((f,link,target))
if missing:
    for f,link,target in missing:
        print(f'{f}: {link} -> {target} MISSING')
else:
    print('OK')
PY
```

## Governance Check

For any node moved to `status: deprecated`, ensure:

- it has a `gov:replaced_by` edge
- at least one active projection excludes it from default traversal

#tech_glossary
#experimental_ontology
