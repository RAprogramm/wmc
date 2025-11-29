# World Map Component - Technical Architecture (2025)

## Project Overview

Blazing-fast WebComponent for world map visualization with markers, built with Leptos (Rust→WASM) and WebGL2.

**Tech Stack:**
- Rust edition 2024
- Leptos 0.8+ (reactive framework)
- web-sys (WebGL2 bindings)
- wasm-bindgen (JS interop)
- masterror (error handling)

## Design Principles

1. **Separation of Concerns**: Clear boundaries between rendering, geometry, state, and presentation
2. **Zero-Cost Abstractions**: No runtime overhead, compile-time optimizations
3. **Type Safety**: Leverage Rust type system for correctness
4. **Performance First**: GPU rendering, instancing, memory reuse
5. **Modularity**: Each module has single responsibility, no circular dependencies
6. **Testability**: Pure functions, mockable dependencies, deterministic behavior

## Architecture Layers

```
┌─────────────────────────────────────────────────────┐
│              WebComponent API Layer                  │
│  (Leptos component, props, events, lifecycle)       │
└─────────────────────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────┐
│             Application Logic Layer                  │
│  (State management, validation, orchestration)       │
└─────────────────────────────────────────────────────┘
                         │
        ┌────────────────┼────────────────┐
        ▼                ▼                ▼
┌──────────────┐  ┌──────────────┐  ┌──────────────┐
│   Rendering  │  │   Geometry   │  │    Themes    │
│    Engine    │  │   Pipeline   │  │    System    │
└──────────────┘  └──────────────┘  └──────────────┘
        │                │                │
        └────────────────┼────────────────┘
                         ▼
┌─────────────────────────────────────────────────────┐
│              Platform Abstraction Layer              │
│         (web-sys, wasm-bindgen, memory)              │
└─────────────────────────────────────────────────────┘
```

## Directory Structure

```
wmc/
├── crates/
│   ├── wmc-core/              # Core library (no web dependencies)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── geometry/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── projection.rs      # Coordinate projections
│   │   │   │   ├── viewport.rs        # Viewport transformations
│   │   │   │   └── bounds.rs          # Bounding boxes, spatial queries
│   │   │   ├── markers/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── marker.rs          # Marker data structure
│   │   │   │   ├── buffer.rs          # Marker buffer management
│   │   │   │   ├── clustering.rs      # Spatial clustering/LOD
│   │   │   │   └── animation.rs       # Animation parameters
│   │   │   ├── themes/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── theme.rs           # Theme data structures
│   │   │   │   ├── presets.rs         # Built-in themes
│   │   │   │   └── validation.rs      # Theme validation
│   │   │   ├── geo/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── topology.rs        # World topology data
│   │   │   │   ├── parser.rs          # Binary format parser
│   │   │   │   └── simplify.rs        # Geometry simplification
│   │   │   └── error.rs               # Error types (masterror)
│   │   ├── Cargo.toml
│   │   └── tests/
│   │       ├── geometry.rs
│   │       ├── markers.rs
│   │       └── themes.rs
│   │
│   ├── wmc-render/            # WebGL rendering engine
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── webgl/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── context.rs         # WebGL2 context management
│   │   │   │   ├── program.rs         # Shader program abstraction
│   │   │   │   ├── buffer.rs          # GPU buffer management
│   │   │   │   ├── texture.rs         # Texture handling
│   │   │   │   └── framebuffer.rs     # Framebuffer for post-processing
│   │   │   ├── shaders/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── world.vert         # World contour vertex shader
│   │   │   │   ├── world.frag         # World contour fragment shader
│   │   │   │   ├── markers.vert       # Markers vertex shader (instanced)
│   │   │   │   ├── markers.frag       # Markers fragment shader (glow)
│   │   │   │   ├── bloom.vert         # Bloom pass vertex shader
│   │   │   │   └── bloom.frag         # Bloom pass fragment shader
│   │   │   ├── pipeline/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── world_renderer.rs  # World contour rendering
│   │   │   │   ├── marker_renderer.rs # Marker rendering (instanced)
│   │   │   │   └── bloom_renderer.rs  # Bloom post-process
│   │   │   ├── camera.rs              # Camera/viewport state
│   │   │   └── error.rs
│   │   ├── Cargo.toml
│   │   └── tests/
│   │
│   └── wmc-component/         # Leptos WebComponent
│       ├── src/
│       │   ├── lib.rs
│       │   ├── component.rs           # Main Leptos component
│       │   ├── props.rs               # Component props/attributes
│       │   ├── events.rs              # Custom events
│       │   ├── state.rs               # Component state management
│       │   ├── interaction.rs         # Mouse/touch interaction
│       │   ├── lifecycle.rs           # Component lifecycle hooks
│       │   └── error.rs
│       ├── Cargo.toml
│       └── tests/
│
├── assets/
│   ├── world-110m.bin         # Pre-processed world topology (110m resolution)
│   ├── world-50m.bin          # High-res topology (optional)
│   └── fallback.svg           # SVG fallback image
│
├── examples/
│   ├── basic/                 # Basic usage example
│   │   ├── index.html
│   │   └── main.rs
│   ├── react-integration/     # React integration example
│   ├── vue-integration/       # Vue integration example
│   └── stress-test/           # Performance stress test
│
├── benches/
│   ├── projection.rs          # Projection benchmarks
│   ├── rendering.rs           # Rendering benchmarks
│   └── markers.rs             # Marker buffer benchmarks
│
├── tests/
│   ├── integration/
│   │   ├── webgl_init.rs
│   │   ├── marker_rendering.rs
│   │   └── theme_switching.rs
│   └── e2e/
│       └── visual_regression.rs
│
├── scripts/
│   ├── build-geo.sh           # Process GeoJSON → binary format
│   ├── optimize-wasm.sh       # Post-build WASM optimization
│   └── generate-themes.rs     # Generate theme presets
│
├── .github/
│   └── workflows/
│       ├── ci.yml             # Main CI pipeline
│       ├── release.yml        # Release automation
│       └── benchmark.yml      # Performance tracking
│
├── Cargo.toml                 # Workspace root
├── .rustfmt.toml             # Formatting config
├── rust-toolchain.toml       # Toolchain specification
├── README.md
├── ARCHITECTURE.md           # This file
├── CONTRIBUTING.md
└── LICENSE
```

## Module Responsibilities

### wmc-core (Pure Rust, no web dependencies)

**Purpose:** Core domain logic, data structures, algorithms

**Responsibilities:**
- Coordinate projections (Mercator, Equirectangular)
- Marker data structures and buffers
- Spatial clustering and LOD
- Theme definitions and validation
- World topology data and parsing
- Pure computational functions

**Key Types:**
- `Marker`: Marker data structure
- `MarkerBuffer`: Fixed-size marker pool with reuse
- `Projection` trait: Coordinate transformation interface
- `Theme`: Theme configuration
- `WorldTopology`: Parsed world geometry
- `Viewport`: View transformation state

**Dependencies:** Only std, serde, masterror

### wmc-render (WebGL rendering, web-sys)

**Purpose:** GPU rendering pipeline, shader management

**Responsibilities:**
- WebGL2 context initialization and management
- Shader compilation and program management
- GPU buffer allocation and updates
- Rendering pipeline orchestration
- Camera and viewport transformations
- Post-processing effects (bloom, blur)

**Key Types:**
- `RenderContext`: WebGL2 context wrapper
- `ShaderProgram`: Compiled shader program
- `GpuBuffer<T>`: Type-safe GPU buffer
- `WorldRenderer`: World contour rendering
- `MarkerRenderer`: Instanced marker rendering
- `BloomRenderer`: Bloom post-process

**Dependencies:** web-sys (WebGL features), wmc-core, masterror

### wmc-component (Leptos WebComponent)

**Purpose:** WebComponent API, reactivity, lifecycle

**Responsibilities:**
- Leptos component definition
- Props/attributes parsing and validation
- State management (Leptos signals)
- Mouse/touch event handling
- Component lifecycle (mount, unmount, update)
- Custom events dispatch
- Integration glue between core and render

**Key Types:**
- `WorldMap`: Main Leptos component
- `WorldMapProps`: Component properties
- `WorldMapState`: Internal state
- `InteractionHandler`: Mouse/touch handling
- `WorldMapEvent`: Custom event types

**Dependencies:** leptos, web-sys, wmc-core, wmc-render, masterror

## Data Flow

### Initialization Flow

```
User HTML
    ↓
<world-map theme="dark" markers='[...]'>
    ↓
Leptos parses attributes → WorldMapProps
    ↓
Component mount → create_effect
    ↓
Initialize RenderContext (WebGL2)
    ↓
Load WorldTopology from binary
    ↓
Parse Markers → MarkerBuffer
    ↓
Apply Theme → ShaderProgram uniforms
    ↓
Start animation loop (requestAnimationFrame)
    ↓
Dispatch 'ready' event
```

### Update Flow (Reactive)

```
User updates attribute (markers=...)
    ↓
Leptos signal triggers
    ↓
create_effect re-runs
    ↓
Parse new markers → validate
    ↓
Update MarkerBuffer (in-place if possible)
    ↓
Upload to GPU (GpuBuffer::update)
    ↓
Next frame renders new state
```

### Rendering Loop

```
requestAnimationFrame callback
    ↓
Update time uniform
    ↓
Clear framebuffer
    ↓
Render world contour (WorldRenderer::draw)
    ↓
Render markers (MarkerRenderer::draw_instanced)
    ↓
Apply bloom (if enabled, BloomRenderer::apply)
    ↓
Present to canvas
```

## Memory Management Strategy

### Marker Buffer (Ring Buffer Design)

```
┌─────────────────────────────────────────────┐
│   MarkerBuffer (fixed size, pre-allocated)  │
│                                             │
│  [M1][M2][M3][M4][  ][  ][  ][  ]          │
│   ↑           ↑                             │
│  head        tail                           │
│                                             │
│  Capacity: maxMarkers (e.g., 10000)        │
│  Active count: tail - head                  │
└─────────────────────────────────────────────┘

Updates:
- Add: append at tail (if space), O(1)
- Remove: mark as inactive, compact on threshold
- Update: in-place modification, O(1)
- No reallocations during normal operation
```

### GPU Buffer Management

```
CPU Side (WASM memory):
┌──────────────────────────────┐
│  MarkerBuffer (Rust Vec)     │
│  [x,y,i,p,r,g,b,a] * N       │
└──────────────────────────────┘
         │
         │ (zero-copy view via TypedArray)
         ▼
┌──────────────────────────────┐
│  Float32Array (JS side)      │
└──────────────────────────────┘
         │
         │ gl.bufferSubData
         ▼
┌──────────────────────────────┐
│  GPU ARRAY_BUFFER            │
│  (WebGL2 vertex buffer)      │
└──────────────────────────────┘
```

**Key Points:**
- Single allocation per buffer
- Updates via `bufferSubData` (partial updates)
- Double buffering for large updates (minimize stalls)
- Memory aligned to 16 bytes (SIMD optimization)

## Error Handling Strategy

### Error Types Hierarchy

```rust
// Using masterror::AppError

// Core errors (wmc-core)
InvalidCoordinates { lat, lon }
InvalidMarkerId { id }
ThemeValidationFailed { reason }
TopologyParseError { details }

// Render errors (wmc-render)
WebGLContextCreationFailed
ShaderCompilationFailed { shader_type, log }
ProgramLinkingFailed { log }
BufferAllocationFailed { size }
UnsupportedWebGLVersion

// Component errors (wmc-component)
AttributeParseError { attribute, value }
MarkerUrlFetchFailed { url, status }
ComponentNotMounted
```

### Error Propagation

```
Component Layer:
  ↓ Result<T, AppError>
  ├─ Display user-friendly message
  ├─ Dispatch 'error' CustomEvent
  ├─ Fallback to safe state (SVG or empty)
  └─ Log to console (dev mode)

Render Layer:
  ↓ Result<T, AppError>
  ├─ Cleanup GL resources
  └─ Propagate to component

Core Layer:
  ↓ Result<T, AppError>
  └─ Pure error propagation (no side effects)
```

## Performance Optimizations

### Compile-Time Optimizations

1. **LTO (Link Time Optimization):**
   - `lto = true` in release profile
   - Cross-crate inlining

2. **WASM Optimization:**
   - `wasm-opt -O3` post-processing
   - Dead code elimination
   - Function inlining

3. **Shader Compilation:**
   - Shaders as const strings (embedded in binary)
   - Pre-validated syntax at compile time

### Runtime Optimizations

1. **GPU Instancing:**
   - Single draw call for all markers
   - Instanced attributes: position, color, phase
   - Base geometry (quad) shared

2. **Frustum Culling:**
   - Only render markers in viewport
   - Spatial index for quick lookup

3. **LOD (Level of Detail):**
   - Cluster distant markers
   - Reduce detail at low zoom levels

4. **Memory Reuse:**
   - Ring buffer for markers (no reallocations)
   - Object pools for temporary allocations
   - Arena allocators for batch operations

5. **Lazy Evaluation:**
   - Defer expensive computations until visible
   - Throttle/debounce user interactions

## Testing Strategy

### Unit Tests (95%+ coverage target)

**wmc-core:**
- Projection correctness (known coordinate pairs)
- Marker buffer operations (add, remove, update)
- Theme validation (valid/invalid configs)
- Topology parsing (edge cases, malformed data)

**wmc-render:**
- Shader compilation (valid/invalid GLSL)
- Buffer allocation and updates
- Rendering pipeline setup

**wmc-component:**
- Props parsing (valid/invalid attributes)
- Event dispatching
- State transitions

### Integration Tests

- WebGL initialization in headless browser
- Marker rendering with different themes
- Interactive pan/zoom
- Dynamic marker updates

### Visual Regression Tests

- Screenshot comparison for each theme
- Marker rendering accuracy
- Animation consistency

### Performance Tests (Criterion)

- Projection performance (1M coordinates)
- Marker buffer operations (10K markers)
- Rendering FPS (1K, 5K, 10K markers)
- Memory usage stability

### E2E Tests (Playwright)

- Component lifecycle
- Browser compatibility (Chrome, Firefox, Safari)
- Mobile touch interactions

## Build Pipeline

### Development Build

```bash
# Fast iteration, source maps, no optimization
cargo make dev

# Features:
- wasm-pack build --dev
- Source maps enabled
- No optimization
- Fast compile time
```

### Production Build

```bash
# Optimized, minified, CDN-ready
cargo make release

# Steps:
1. cargo build --release (with LTO)
2. wasm-pack build --release
3. wasm-opt -O3 (Binaryen optimizer)
4. gzip compression
5. Generate integrity hashes
6. Bundle size report
```

### CI/CD Pipeline

```yaml
on: [push, pull_request]

jobs:
  format:
    - cargo +nightly fmt --check

  lint:
    - cargo clippy --all-targets -- -D warnings

  test:
    - cargo test --all-features

  coverage:
    - cargo tarpaulin --out Xml
    - Upload to Codecov

  benchmark:
    - cargo bench
    - Compare with baseline
    - Fail if >10% regression

  build:
    - cargo build --release
    - wasm-pack build --release
    - Check bundle size (<150KB gzip)

  e2e:
    - playwright test
    - Visual regression tests
```

## Deployment Strategy

### NPM Package

```
@wmc/world-map/
├── pkg/
│   ├── wmc_component.js          # JS glue
│   ├── wmc_component_bg.wasm     # WASM binary
│   ├── wmc_component.d.ts        # TypeScript definitions
│   └── package.json
├── assets/
│   ├── world-110m.bin
│   └── fallback.svg
└── README.md
```

### CDN Distribution

```
https://cdn.jsdelivr.net/npm/@wmc/world-map@1.0.0/
├── wmc_component.js
├── wmc_component_bg.wasm
├── world-110m.bin
└── fallback.svg

Versioned, immutable, CORS-enabled
```

### Integration Examples

**Vanilla HTML:**
```html
<script type="module" src="https://cdn.../wmc_component.js"></script>
<world-map theme="dark-minimal" markers='[...]'></world-map>
```

**React:**
```jsx
import '@wmc/world-map';
<world-map ref={mapRef} theme={theme} markers={JSON.stringify(markers)} />
```

**Vue:**
```vue
<template>
  <world-map :theme="theme" :markers="markersJson" />
</template>
```

## Security Considerations

### Input Validation

- Sanitize all attribute inputs (XSS prevention)
- Validate marker coordinates (reject NaN, Infinity)
- Limit marker count (DoS prevention)
- URL validation for markers-url (SSRF prevention)

### CORS Policy

- Strict CORS headers for asset loading
- Subresource Integrity (SRI) for CDN assets
- Content Security Policy recommendations

### Privacy

- No telemetry by default
- Coordinate obfuscation guidance
- Client-side validation only (no data sent)

## Browser Compatibility Matrix

| Browser         | Version | WebGL2 | WASM | Support |
|----------------|---------|--------|------|---------|
| Chrome         | 90+     | ✓      | ✓    | Full    |
| Firefox        | 88+     | ✓      | ✓    | Full    |
| Safari         | 15+     | ✓      | ✓    | Full    |
| Edge           | 90+     | ✓      | ✓    | Full    |
| Mobile Safari  | 15+     | ✓      | ✓    | Full    |
| Mobile Chrome  | 90+     | ✓      | ✓    | Full    |

**Fallback:** Static SVG image for unsupported browsers

## Performance Targets

| Metric                    | Target         | Measurement                |
|---------------------------|----------------|----------------------------|
| Cold start (load + init)  | <800ms         | Performance API            |
| Frame rate (1K markers)   | 60 FPS         | requestAnimationFrame      |
| Frame rate (5K markers)   | 45+ FPS        | Mid-tier laptop            |
| Frame rate (10K markers)  | 30+ FPS        | Desktop                    |
| Bundle size (gzip)        | <150KB         | wasm-pack output           |
| Memory usage (steady)     | <50MB          | Chrome DevTools            |
| Time to interactive       | <1s            | Lighthouse                 |

## Future Enhancements (Post-MVP)

### Phase 1 (MVP)
- ✓ 2D Mercator projection
- ✓ Single theme (dark-minimal)
- ✓ Basic marker rendering
- ✓ WebGL2 instanced rendering
- ✓ Props: markers, theme

### Phase 2 (Production)
- All 5 themes
- Interactive pan/zoom
- Marker clustering/LOD
- Dynamic updates (add/remove markers)
- Custom theme support
- Events (click, hover)

### Phase 3 (Advanced)
- Globe mode (3D orthographic projection)
- Bloom post-processing
- Heatmap mode
- Lines/paths between markers
- Custom marker shapes
- WebGPU backend (when stable)

## Glossary

- **Instanced Rendering:** GPU technique to draw multiple objects with one draw call
- **LOD:** Level of Detail - reduce complexity based on distance/zoom
- **Ring Buffer:** Fixed-size circular buffer for memory reuse
- **Frustum Culling:** Skip rendering objects outside viewport
- **SRI:** Subresource Integrity - verify asset integrity
- **Fine-grained Reactivity:** Update only changed parts of UI, not entire tree
