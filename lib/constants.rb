PROCESS_TYPE_MAP = {
  "LaserLineEngrave" => { type: "Cut",  name: "Line", index: 0 },
  "LaserFillEngrave" => { type: "Scan", name: "Fill", index: 1 },
  "LaserLineCut"     => { type: "Cut",  name: "Cut",  index: 2 },
}.freeze

DITHER_MAP = {
  "IF_Relief"    => "stucki",
  "IF_Threshold" => "threshold",
  "IF_Ordered"   => "ordered",
  "IF_Dither"    => "floyd",
}.freeze
