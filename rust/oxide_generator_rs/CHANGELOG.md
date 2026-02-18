## 0.3.0
- Add isolated channels codegen pipeline (scan, validate, naming, event/callback generation) and compile tests
- Add navigation routes codegen
- Improve reducer expansion for sliced-state inference output
- Refresh UI test expected stderr outputs and update metadata

## 0.2.0
- Split reducer macro implementation into focused modules
- Generate state-specific slice enums (e.g. AppStateSlice) instead of StateSlice
- Detect Infer/Slices usage via AST and add #[reducer(sliced = ...)] override
- Drop tokio_handle/runtime-handle based codegen; engine constructors are async and use InitContext
- Introduce #[state(sliced = true)] for struct states to generate slice enums and fieldwise inference
- Add compile-time validation to prevent sliced enum usage and ensure compatibility
