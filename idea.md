
# Цель

Создать универсальный, надёжный и лёгко интегрируемый WebComponent для визуализации «тёмный контур мира + маркеры / светлячки» на основе Leptos (Rust→WASM). Модуль должен поддерживать несколько предустановленных стилей (тем), динамическую подачу меток/маркеров, реактивное обновление через props/attributes и работать как автономный компонент в любом веб-окружении. Предназначен для использования как фон (background) на сайтах, дашбордах и веб-приложениях.

# Ключевые требования (вкратце)

* Универсальность: WebComponent для использования в любом HTML-проекте (React, Vue, Svelte, vanilla HTML).
* Full Rust stack: весь код на Rust (Leptos framework), никакого ручного JS.
* Стили/темы: 3–5 предустановленных тем + возможность задания custom theme через attributes.
* Реактивность: автоматическое обновление при изменении props (markers, theme, mode).
* Производительность: поддерживать тысячи маркеров с плавной анимацией; GPU-рендеринг (WebGL2 через web-sys), instancing.
* Безопасность/приватность: обфускация/округление координат на стороне сервера по умолчанию; валидация данных на клиенте.
* Graceful degradation: static SVG fallback если WebGL2 недоступен.
* Малый бандл: целевой размер WASM bundle gzip < ~100–150 KB.
* Документация, тесты, CI.

# Парадигма архитектуры (в одном предложении)

Leptos WebComponent полностью на Rust: WebGL rendering через web-sys, реактивное управление состоянием через Leptos signals, геометрия и проекции в Rust, zero-JS codebase.

# Структура API (WebComponent — что должен знать интегратор)

## Использование

WebComponent `<world-map>` со следующими attributes: theme, mode, projection, interactive, markers (JSON string), markers-url, width, height.

## Attributes (WebComponent properties)

* theme: string (default: "dark-minimal") — имя предустановленной темы.
* theme-config: JSON string — кастомная конфигурация темы (если theme="custom").
* mode: "2d" | "globe" (default: "2d") — режим отображения.
* projection: "mercator" | "equirectangular" (default: "mercator") — тип проекции для 2D режима.
* interactive: boolean (default: true) — разрешить пан/зум.
* markers: JSON string — массив маркеров для отображения.
* markers-url: string (optional) — URL для загрузки маркеров (альтернатива прямой передаче).
* width: string (default: "100%") — ширина canvas.
* height: string (default: "100%") — высота canvas.

## DOM API

Можно динамически менять properties через DOM (map.markers = ..., map.theme = ...) и слушать CustomEvents (ready, error, marker-click, marker-hover).

## События

* ready — WebGL инициализирован, карта готова.
* error — ошибка загрузки/рендеринга (detail: error message).
* marker-click — клик по маркеру (detail: {id, lat, lon, meta}).
* marker-hover — hover над маркером (detail: {id, lat, lon, meta}).

## Формат Marker

Marker: id (string/number), lat (number), lon (number), intensity (0..1, optional), color (hex/css, optional), radius (px/world-units, optional), meta (object, opaque, optional).

## MarkerDiff

Частичные обновления: add (Marker[]), update ({id, fieldsToUpdate}[]), remove (id | id[]).

# Темы / стили (предустановленные)

Каждая тема — набор параметров визуала для карты и маркеров. Тема должна быть JSON-сериализуемой.

1. `dark-minimal`

   * фон: #0b0f10
   * контур: тонкая линия, rgba(30,37,41,0.6), slight blur
   * маркеры: мягкий бирюзовый glow, radius base 3–8px, additive blending
   * шум: subtle grain overlay 1.5% opacity

2. `neon-glow`

   * фон: почти чёрный с слабым градиентом
   * контур: тонкий wireframe с neon edge
   * маркеры: яркие пульсирующие точки (cyan/amber), сильный bloom
   * движение: лёгкий drift и perlin-noise мерцание

3. `subtle-texture`

   * фон: глубокий тёмно-синий
   * контур: мягкие полупрозрачные заливки
   * маркеры: warm glow, низкая интенсивность, no bloom
   * добавка: тонкий texture map (paper/noise)

4. `wireframe`

   * фон: тёмно-серый
   * контур: single stroke, dashed optional
   * маркеры: маленькие точки, sharp, без blur — для строгого корпоративного вида

5. `custom`

   * все параметры настраиваются: цвета, radii, blend mode, pulsation speed, blur params, contour thickness, show/islands true/false и т.д.

# Что реализует Rust (Leptos + WASM) — все обязанности

* Leptos WebComponent: регистрация custom element, props/attributes, lifecycle, события.
* WebGL rendering через web-sys: context init, shaders compilation, buffers, draw calls, instancing.
* Загрузка и парсинг минимизированного topojson/geojson контура (pre-baked binary format).
* Проекции координат (Mercator, Equirectangular).
* Пул/буферы маркеров: preallocated layout [x, y, intensity, phase, r, g, b, a] для каждого маркера.
* Кластеризация / LOD (опционально): на вход zoomLevel, возвращает set маркеров или агрегатов.
* Генерация per-marker animation parameters (phase offsets через hash от marker.id).
* Валидация входящих данных (отброс невалидных координат, NaN, Infinity).
* Реактивность: Leptos signals для автоматического re-render при изменении props.
* UI интеракция: обработка mouse/touch events для pan/zoom через web-sys.
* Animation loop: requestAnimationFrame через web-sys.
* Минимизировать аллокации в render-loop; использовать ring buffers и reuse.

# Рендеринг / шейдеры (технические указания)

* WebGL2, instanced rendering: один буфер для вершин точек (quad) + instanced attributes из WASM.
* Vertex shader: преобразует instanced position/scale/phase → экранные координаты.
* Fragment shader: отрисовка мягкого glow через distance-based alpha (smoothstep) + additive blending.
* Uniforms: time, devicePixelRatio, projectionMatrix, theme params (glow color, radiusScale).
* Bloom/blur: легкий постэффект по желанию (опционально, в отдельный pass).
* Blend mode: additive для glow, premultiplied alpha для корректных композиций.

# Интеграция как фон (варианты)

1. Прямой WebComponent: canvas занимает full-bleed background, поверх обычный HTML content.
2. React/Vue/Svelte: импортируют WebComponent как обычный custom element.
3. Iframe: для сайтов где нельзя добавлять WASM (CMS) - iframe с standalone page.
4. Server-side render fallback: static SVG image для SSR/SEO.

# Privacy / security

* По умолчанию координаты округляются на бэкенде до 3–5 км (0.01°) и/или добавляется случайный noise.
* Маркеры с private flags не передаются в публичные инсталляции.
* Все входящие данные валидировать в Rust (serde validation).
* CORS, CSP — рекомендации в документации.
* Опция подписывания payload на сервере (HMAC) для предотвращения подделки данных.

# Fallbacks и graceful degradation

* Если WebGL2 недоступен: показать static SVG fallback image (embedded в WASM или data-url).
* Если WASM не загружен (очень редко): fallback на статическую картинку через noscript tag.

# Тесты и критерии приёмки

* Unit тесты (Rust): проекции, буферы, кластеризация, edge cases геометрии.
* Integration tests: WASM module load, memory mapping, JS glue.
* E2E: инициализация в headless браузере, API вызовы setMarkers/updateMarkers.
* Perf tests: замер FPS с 1k, 5k, 10k маркеров на target machines (см. ниже).
* Visual regression: скриншотные тесты для каждой темы.

Критерии приёмки (минимум):

* MapInstance инициализируется без ошибок в современном Chrome/Firefox/Safari.
* Рендер 5k маркеров @ ≥45 FPS на условном ноуте (см. целевые устройства).
* Fallback корректно работает при отключённом WASM.
* API соответствуют спецификации и покрыты тестами.

# Целевые платформы / устройства (на что оптимизировать)

* Desktop: современные ноутбуки (Intel i5/AMD Ryzen, integrated GPU), 4k поддержка.
* Mobile: современные iOS/Android — обеспечить режим low-quality (LOD) для слабых устройств.
* Browsers: latest Chrome, Firefox, Safari, Edge.

# Packaging / distribution

* NPM package: `@wmc/world-map` с wasm-pack output
* Содержимое: .wasm binary, JS glue (minimal), TypeScript definitions, assets (topology, fallback SVG)
* CDN hosting: jsDelivr/unpkg с версионированием и SRI hashes
* Build: wasm-pack --release + wasm-opt -O3 + gzip
* Интеграция: одна строка HTML для импорта, затем используем WebComponent
* Bundle size target: <150KB gzip

# Telemetry / diagnostics (опционально)

* Собираемые метрики (опционально): load time wasm, FPS, marker count. Собирается только с согласия клиента (privacy first).
* Логи ошибок в консоль + optional POST в telemetry endpoint при включённом флаге.

# Directory structure

Workspace with 3 crates:
- wmc-core: Pure Rust (geometry, markers, themes, topology)
- wmc-render: WebGL rendering via web-sys
- wmc-component: Leptos WebComponent

Assets: Pre-processed binary topology (world-110m.bin), fallback SVG
Tests: Unit, integration, E2E, visual regression, benchmarks

See ARCHITECTURE.md for detailed structure.

# Roadmap (phased delivery scope)

Phase 1 (MVP):
- wmc-core: Mercator projection, marker buffer, single theme
- wmc-render: WebGL2 context, basic shaders, world + markers rendering
- wmc-component: Leptos component, basic props (markers, theme)
- Static camera (no interaction)
- Binary topology format
- 95% test coverage

Phase 2 (Production):
- All 5 themes + custom theme support
- Interactive pan/zoom
- Marker clustering/LOD
- Dynamic marker updates (reactive)
- Events (ready, error, marker-click, marker-hover)
- WebComponent export
- Documentation + examples

Phase 3 (Advanced):
- Globe mode (orthographic projection)
- Bloom post-processing
- Heatmap visualization
- Lines/paths between markers
- CDN deployment
- Performance telemetry (opt-in)

# Метрики производительности для проверки

* Cold start time (load wasm + init GL) — target < 800 ms on broadband.
* Warm frame render: 1k markers ≥ 60 FPS, 5k markers ≥ 45 FPS on mid-tier laptop.
* JS main thread < 20% load during animation.
* WASM memory usage stable — no leak after 10k add/remove cycles.

# Риски и компенсации

* Большой wasm → longer download: компромисс — split wasm: core compute vs optional extras, lazy load heavy features.
* Debugging WASM harder: добавить source map и dev build with console instrumentation.
* DOM interop cost: держать WASM и DOM responsibilities разделёнными.

# Документация и примеры

* README: установка, quickstart, themes, API reference, FAQ.
* Demo page: live switcher тем, режим globe/2d, кнопки add/remove markers, performance meter.
* Integration recipes: React, Vue, plain HTML, WordPress iframe snippet.

# Что не входит (по умолчанию)

* Backend для хранения/обфускации координат (предоставляется как optional guidance).
* GIS heavy features: routing, geocoding, сложные пространственные аналитики — можно добавить как отдельный модуль.



