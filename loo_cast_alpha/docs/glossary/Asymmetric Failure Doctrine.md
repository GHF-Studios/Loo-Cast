---
canonical_name: Asymmetric Failure Doctrine
status: WIP-draft
aliases: []
source_of_truth: []
---

Asymmetric Failure Doctrine means panic-fast is the default runtime integrity posture, while persistence-sensitive
paths (especially save/load) are handled with higher recovery care and corruption-avoidance policy.
This asymmetry is intentional.
It preserves fast failure and clear fault visibility without treating persistence risks as ordinary transient runtime
faults.

See also:

- [[Runtime Lock]]
- [[Project Runtime Representation]]

#glossary
