# Eligibility FHIR and 271 sketch — Rust

Small library and CLI that sanity-check **CoverageEligibilityRequest/Response** JSON and a minimal **271** segment sketch before real-time eligibility traffic hits your gateway. Structural only — not an eligibility decision.

Synthetic fixtures only. Not ONC certification, not a substitute for the HL7 FHIR Validator or Inferno, and not legal/clinical advice.

**RCM / Claims edition** — separate from the US Core exhibit series (synthetic EDI sketches + FHIR where noted).

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 22** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [CoverageEligibilityRequest/Response](http://hl7.org/fhir/R4/coverageeligibilityrequest.html)
- X12 270/271 (structural sketch only)

## What it checks

- FHIR: **CoverageEligibilityRequest** or **Response**, **status**, patient or subscriber ref
- X12 sketch (`.txt`): **ISA** envelope and **EB** segment present

| Problem | What breaks |
|--------|-------------|
| **Wrong resource type** | Gateway routes to wrong handler |
| **271 without EB** | Benefit loop missing in payer response |
| **No patient/subscriber** | 270/271 loop cannot start |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-eligibility -- examples/valid-eligibility-request.json
cargo run --bin validate-eligibility -- examples/invalid-eligibility-request.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or input.

## Sample output

```bash
cargo run --bin validate-eligibility -- examples/valid-eligibility-request.json
```

**Pass** — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-eligibility-request.json`) — exit code `1`:

```json
[
  {
    "severity": "error",
    "path": "patient",
    "rule_id": "..."
  }
]
```

## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken fixtures for tests and demos

## GitHub topics

Paste in repository **About** (also: `fhir`, `hl7-fhir`, `healthcare-interoperability`, `rust`):

`eligibility, x12, 270-271, coverage, fhir-r4, healthcare-interoperability, fhir, hl7-fhir, rust`

## Inferno vs this repo

Certification tests live **eligibility behavior** on endpoints. This repo is **payload construction** at the pipe.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
