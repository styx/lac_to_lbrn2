class EllipseTransformer
  def transform(instance, xml, cut_index, offset)
    obj    = instance.obj
    t      = instance.transform
    ox, oy = offset

    cx, cy = t.apply(obj['center_x'], obj['center_y'])

    xml.open("Shape",
      "Type"     => "Ellipse",
      "CutIndex" => cut_index.to_s,
      "Rx"       => "%.10g" % obj['radius_x'],
      "Ry"       => "%.10g" % obj['radius_y']
    )
    xml.inline("XForm", "1 0 0 1 %.10g %.10g" % [cx - ox, cy - oy])
    xml.close("Shape")
  end
end
