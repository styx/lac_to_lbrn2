require 'base64'

class RasterImageTransformer
  def initialize(objects_path)
    @objects_path = objects_path
  end

  def transform(instance, xml, cut_index, offset)
    obj    = instance.obj
    t      = instance.transform
    ox, oy = offset

    w_mm = obj['width']  * t.scale_x.abs
    h_mm = obj['height'] * t.scale_y.abs
    cx   = t.tx - ox
    cy   = t.ty - oy

    png_path = File.join(@objects_path, obj['file_name'])
    b64      = Base64.strict_encode64(File.read(png_path, encoding: 'binary'))

    settings = obj['image_settings'] || {}
    contrast = (settings['contrast_adjust']   || 0).to_f / 100.0
    bright   = (settings['brightness_adjust'] || 0).to_f / 100.0
    enhance  = (settings['sharpness_adjust']  || 0).to_f

    xml.open("Shape",
      "Type"           => "Bitmap",
      "CutIndex"       => cut_index.to_s,
      "W"              => "%.10g" % w_mm,
      "H"              => "%.10g" % h_mm,
      "Gamma"          => "1",
      "Contrast"       => "%.10g" % contrast,
      "Brightness"     => "%.10g" % bright,
      "EnhanceAmount"  => "%.10g" % enhance,
      "EnhanceRadius"  => "0",
      "EnhanceDenoise" => "0",
      "File"           => png_path,
      "SourceHash"     => "0",
      "Data"           => b64
    )
    xml.inline("XForm", "1 0 0 1 %.10g %.10g" % [cx, cy])
    xml.close("Shape")
  end

  private

  attr_reader :objects_path
end
