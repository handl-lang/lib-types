# lib-types

FFI type-alias declarations shared by sdkgen-generated bindings (`types/c/*`).

Part of the `handl-lang` org split (ADR-0128). History is a `git subtree
split` of `host/handl-packages/handl-packages/types/`.

## Layout

```text
src/        the .hnl sources
pkg.hnl     dependencies, when any: `{ deps: { <name>: "branch:<ref>" } }`
```

## Gate

```bash
HNL=../host-cli/target/debug/hnl scripts/hnl-gate.sh
```

## License

MIT.
