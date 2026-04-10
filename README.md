# lac\_to\_lbrn2

> Convert **BambuStudio LAC** laser files into **LightBurn `.lbrn2`** projects — instantly, from the command line.

![Ruby](https://img.shields.io/badge/ruby-%3E%3D_3.1-CC342D?style=flat-square&logo=ruby&logoColor=white)
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

- Ruby `>= 3.1`
- [Bundler](https://bundler.io/)

Install dependencies:

```sh
bundle install
```

This also adds `base64` to the bundle for Ruby 3.4+ compatibility.

---

## Usage

```sh
bin/convert input.lac
```

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

1. **Extract** — unzip relevant folders into a temp directory
2. **Parse scene** — `SceneBuilder` walks the component tree, composing affine transforms at each level to produce a flat list of `Instance` structs
3. **Inject process types** — map each object to its `LaserLineEngrave` / `LaserFillEngrave` / `LaserLineCut` process type from `project_settings.json`
4. **Load process params** — resolve power & speed from the material `.config` file
5. **Emit XML** — `XmlBuilder` serialises the LightBurn project:
   - `CutSetting` / `CutSetting_Img` nodes per layer
   - `Shape` nodes for each object (`Path`, `Ellipse`, `Bitmap`)
   - Embedded Base64 image data for rasters

---

## Layer mapping

| BambuStudio process type | LightBurn type | Default layer name |
|--------------------------|----------------|--------------------|
| `LaserLineEngrave`       | `Cut`          | Line               |
| `LaserFillEngrave`       | `Scan`         | Fill               |
| `LaserLineCut`           | `Cut`          | Cut                |
| `Raster image`           | `Image`        | `C03`, …           |

Additional process types (no idea, but just in case) not in the table above are assigned to sequential layers.

---

## License

MIT
