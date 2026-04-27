# Scripting Diagram Atlas

Visual index: [00_manifest.puml](./00_manifest.puml)  
Notation rules: [00_conventions.puml](./00_conventions.puml)
Decision contracts: [50_decision_contracts.puml](./50_decision_contracts.puml)

This folder is intentionally diagram-first. The manifest diagram is the primary navigation surface.

Free tools to edit/view:
- JetBrains (RustRover/IntelliJ): PlantUML Integration plugin
- VS Code: `jebbs.plantuml`
- Browser editor: [planttext.com](https://www.planttext.com/)

Optional CLI rendering:
- `java -jar plantuml.jar <file>.puml`
- `docker run --rm -v "$PWD":/work -w /work plantuml/plantuml <file>.puml`
