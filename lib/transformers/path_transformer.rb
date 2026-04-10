require_relative '../path_parser'
require_relative '../subpath_converter'

class PathTransformer
  def initialize
    @vert_id = 0
    @prim_id = 0
  end

  def transform(instance, xml, cut_index, offset)
    obj    = instance.obj
    t      = instance.transform
    ox, oy = offset

    PathParser.parse(obj['path_data']).each do |segs|
      result = SubpathConverter.convert(segs, obj['is_closed'], t, ox, oy)
      next unless result

      vert_list, prim_list = result
      xml.open("Shape",
        "Type"     => "Path",
        "CutIndex" => cut_index.to_s,
        "VertID"   => @vert_id.to_s,
        "PrimID"   => @prim_id.to_s
      )
      xml.inline("XForm",    "1 0 0 1 0 0")
      xml.inline("VertList", vert_list)
      xml.inline("PrimList", prim_list)
      xml.close("Shape")
      @vert_id += 1
      @prim_id += 1
    end
  end

  private

  attr_reader :vert_id, :prim_id
end
