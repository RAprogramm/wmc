# World Map Component (WMC)

Blazing-fast WebComponent for world map visualization with markers, built with Leptos (Rust→WASM) and WebGL2.

## Features

- **Pure Rust**: Full stack Rust (Leptos + WebGL via web-sys)
- **High Performance**: GPU-accelerated rendering, instanced draw calls
- **Reactive**: Automatic updates via Leptos signals
- **Universal**: Works with any framework (React, Vue, Svelte, vanilla HTML)
- **Lightweight**: Target bundle size <150KB gzip
- **Type Safe**: Rust's type system guarantees correctness

## Architecture

```
wmc/
├── wmc-core       # Pure Rust: geometry, projections, markers, themes
├── wmc-render     # WebGL2 rendering engine (web-sys)
└── wmc-component  # Leptos WebComponent
```

## Quick Start

```html
<script type="module" src="wmc.js"></script>

<world-map
    theme="dark-minimal"
    markers='[{"id":1,"lat":55.7558,"lon":37.6173,"intensity":0.8}]'>
</world-map>
```

## Development

### Prerequisites

- Rust nightly (for formatting)
- wasm-pack
- Node.js (for examples)

### Build

```bash
# Build core library
cargo build -p wmc-core

# Build WASM component
cargo build -p wmc-component --target wasm32-unknown-unknown

# Format code
cargo +nightly fmt

# Run tests
cargo test --all-features

# Run lints
cargo clippy --all-targets --all-features -- -D warnings
```

## Project Status

**Phase 1 (MVP)** - In Progress
- [x] Project structure
- [x] wmc-core: projections, markers, themes
- [x] wmc-render: WebGL2 context, shaders, buffers
- [x] wmc-component: Leptos integration
- [ ] Marker rendering pipeline
- [ ] Theme system
- [ ] Examples
- [ ] Tests (95%+ coverage)

## License

MIT

## Author

RAprogramm <andrey.rozanov.vl@gmail.com>
