# Basic Example

Simple demonstration of World Map Component rendering world contours.

## Build and Run

```bash
# From project root
wasm-pack build crates/wmc-component --target web --out-dir ../../examples/basic/pkg

# Serve (requires simple HTTP server)
cd examples/basic
python3 -m http.server 8080

# Open browser
open http://localhost:8080
```

## Features Demonstrated

- WebGL2 world map rendering
- Mercator projection
- Dark minimal theme
- World continent contours
