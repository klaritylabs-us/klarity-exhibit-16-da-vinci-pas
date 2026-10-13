# Da Vinci PAS (preauthorization Claim) — Rust

Small library and CLI that sanity-check **Da Vinci PAS** **`Claim`** JSON before prior-auth messages hit your clearinghouse or payer API: resource type and **`use`** **`preauthorization`**.

Synthetic fixtures only. Not ONC certification, not a substitute for Da Vinci PAS test kits or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 16** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [Da Vinci PAS](http://hl7.org/fhir/us/davinci-pas/)

## What it checks

- `resourceType` is **`Claim`**
- **`use`** is **`preauthorization`** (not professional/institutional claim use)

| Problem | What breaks |
|--------|-------------|
| **Institutional claim use** | PAS router rejects or misroutes the message |
| **Wrong resource type** | CRD/PAS workflow never binds to Claim profile |
| **Missing `use`** | Server defaults to wrong claim category |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-pas -- examples/valid-pas-claim.json
cargo run --bin validate-pas -- examples/invalid-pas.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-pas -- examples/valid-pas-claim.json
```

**Pass** (`examples/valid-pas-claim.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-pas.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "use",
    "detail": "PAS Claim.use must be preauthorization",
    "rule_id": "pas-use"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken PAS Claim payloads for tests and demos

## Inferno vs this repo

Da Vinci **PAS** testing exercises end-to-end prior auth on live systems. This repo is **Claim header shape** — the fields your authoring tool emits before X12/FHIR gateways translate the request.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
