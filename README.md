# lac\_to\_lbrn2

> Convert **BambuStudio LAC** laser files into **LightBurn `.lbrn2`** projects — instantly, from the command line.

![Rust](https://img.shields.io/badge/rust-%3E%3D_1.75-orange?style=flat-square&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)
![Status](https://img.shields.io/badge/status-active-brightgreen?style=flat-square)

---

## What is this?

BambuStudio exports laser-ready designs as `.lac` files — a ZIP archive containing JSON scene data, process settings, and embedded assets. LightBurn uses its own `.lbrn2` XML format.

`lac_to_lbrn2` bridges that gap. It parses the full scene graph from a `.lac` file and produces a ready-to-open `.lbrn2` project, preserving:

- **Vector paths** (lines, bezier curves) with full transform hierarchy
- **Ellipses** with correct radii and positioning
- **Raster images** embedded as Base64 PNG with image adjustment settings
- **Cut settings** (Line Engrave, Fill Engrave, Cut) mapped to proper LightBurn layer types
- **Laser parameters** — power and speed read directly from the material process configs
- **Dither modes** for image layers (Stucki, Floyd-Steinberg, Threshold, Ordered)

---

## Requirements

- [Rust](https://rustup.rs/) `>= 1.75` (with `cargo`)

---

## Build

```sh
cargo build --release
```

The compiled binary lands at `target/release/convert`.

---

## Usage

```sh
bin/convert input.lac
```

`bin/convert` is a thin shell wrapper that builds the release binary on first run if it is not already present, then delegates to it. You can also invoke the binary directly after building.

This produces `input.lbrn2` alongside the source file.

### Custom output path

```sh
bin/convert input.lac output.lbrn2
```

### Normalize to origin

Shift the entire scene so the bounding box top-left corner lands at `(0, 0)`:

```sh
bin/convert --normalize input.lac
```

---

## How it works

```
.lac (ZIP)
  ├── 2D/2dmodel.json          ← scene graph, object list, canvas
  ├── 2D/Objects/              ← embedded PNG raster images
  └── Metadata2D/
        ├── project_settings.json   ← process type assignments + material name
        └── *.config                ← per-material laser parameters (power, speed)
```

The converter pipeline:

1. **Extract** — unzip relevant folders into a temp directory (`tempfile`)
2. **Parse scene** — `SceneBuilder` walks the component tree, composing affine transforms at each level to produce a flat list of `Instance` values
3. **Inject process types** — map each object to its `LaserLineEngrave` / `LaserFillEngrave` / `LaserLineCut` process type from `project_settings.json`
4. **Load process params** — resolve power & speed from the material `.config` file
5. **Emit XML** — `XmlBuilder` serialises the LightBurn project:
   - `CutSetting` / `CutSetting_Img` nodes per layer
   - `Shape` nodes for each object (`Path`, `Ellipse`, `Bitmap`)
   - Embedded Base64 image data for rasters

### Source layout

```
src/
  main.rs                  ← CLI (clap)
  converter.rs             ← top-level orchestration
  constants.rs             ← process-type and dither maps
  types.rs                 ← Segment, Vertex, Instance, ProcessParams
  transform.rs             ← 2-D affine Transform
  path_parser.rs           ← tokeniser + SVG-like path parser
  subpath_converter.rs     ← segments → LightBurn VertList / PrimList
  scene_builder.rs         ← component-tree walker
  xml_builder.rs           ← indent-aware XML emitter
  utils.rs                 ← fnum (%.10g formatter)
  transformers/
    ellipse.rs             ← Ellipse shape emitter
    path.rs                ← Path shape emitter
    raster_image.rs        ← Bitmap shape emitter
  visitors/
    cut_setting.rs         ← CutSetting / CutSetting_Img emitter
    shape.rs               ← dispatches instances to transformers
```

---

## Layer mapping

| BambuStudio process type | LightBurn type | Default layer name |
|--------------------------|----------------|--------------------|
| `LaserLineEngrave`       | `Cut`          | Line               |
| `LaserFillEngrave`       | `Scan`         | Fill               |
| `LaserLineCut`           | `Cut`          | Cut                |
| Raster image             | `Image`        | `C03`, …           |

Additional process types not in the table above are assigned to sequential layers.

---

## License

MIT