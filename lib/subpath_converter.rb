require_relative 'types'

module SubpathConverter
  def self.convert(segments, is_closed, transform, ox, oy)
    return nil if segments.nil? || segments.length < 2

    vertices = []
    prims    = []

    sx, sy = transform.apply(segments[0].params[0], segments[0].params[1])
    vertices << Vertex.new(sx - ox, sy - oy, nil, nil)

    (1...segments.length).each do |si|
      seg  = segments[si]
      prev = vertices.last
      vi   = vertices.length

      case seg.type
      when :line
        x, y = transform.apply(seg.params[0], seg.params[1])
        prev.c0 = nil
        vertices << Vertex.new(x - ox, y - oy, nil, nil)
        prims << "L#{vi - 1} #{vi}"
      when :bezier
        cp1x, cp1y = transform.apply(seg.params[0], seg.params[1])
        cp2x, cp2y = transform.apply(seg.params[2], seg.params[3])
        ex,   ey   = transform.apply(seg.params[4], seg.params[5])
        prev.c0 = [cp1x - ox, cp1y - oy]
        vertices << Vertex.new(ex - ox, ey - oy, nil, [cp2x - ox, cp2y - oy])
        prims << "B#{vi - 1} #{vi}"
      end
    end

    if is_closed && vertices.length > 1
      vertices.last.c0  = nil
      vertices.first.c1 = nil
      prims << "L#{vertices.length - 1} 0"
    end

    vert_list = vertices.map { |v| fmt_vertex(v) }.join
    prim_list = prims.join

    return nil if prim_list.empty?

    [vert_list, prim_list]
  end

  private_class_method def self.fmt_vertex(v)
    s  = "V#{fnum(v.x)} #{fnum(v.y)}"
    s += v.c0 ? "c0x#{fnum(v.c0[0])}c0y#{fnum(v.c0[1])}" : "c0x1"
    s += v.c1 ? "c1x#{fnum(v.c1[0])}c1y#{fnum(v.c1[1])}" : "c1x1"
    s
  end

  private_class_method def self.fnum(n)
    "%.10g" % n
  end
end
