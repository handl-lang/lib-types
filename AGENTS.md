# lib-types

FFI type-alias declarations shared by codegen-generated bindings
(`types/c/*`).

## Layout

| path | role |
| --- | --- |
| `src/c/` | the `c` module — per-header type-alias files |
| `src/c/{index,netinet,socket,stddef,stdint,sys,time}.hnl` | POSIX header type aliases |
| `pkg.hnl` | manifest — `modules: ["c"]` |
| `scripts/hnl-gate.sh`, `ci.sh` | the gate / bupha entry |

## Contract

- Shared vocabulary consumed by `hnl codegen c` output (e.g.
  `lib-libc`, `lib-tls-libssl` generated bindings).

## Verification

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.

<!-- myco:agent-awareness -->

## Agent awareness

- Session start / any work discussion: run `myco status --prompt`
  before proposing next steps — know the board first.
- User describes new work → offer to capture it (`myco todo add`,
  `issue new`, `proposal new`). Don't wait to be asked.
- Picking up a task: `myco resume <ID>` is the briefing.
- Wrapping up: `myco handoff <ID>` anything resumable; keep statuses
  honest (claimed → in-progress → blocked/done).

<!-- /myco:agent-awareness -->
