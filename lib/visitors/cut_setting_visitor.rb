require_relative '../constants'

class CutSettingVisitor
  def initialize(process_types, process_type_to_obj, process_type_to_idx, process_params)
    @process_types       = process_types
    @process_type_to_obj = process_type_to_obj
    @process_type_to_idx = process_type_to_idx
    @process_params      = process_params
  end

  def visit(xml)
    sorted = @process_types.sort_by { |pt| @process_type_to_idx[pt] || 99 }
    sorted.each do |process_type|
      obj = @process_type_to_obj[process_type]
      idx = @process_type_to_idx[process_type]
      if obj&.fetch('type', nil) == 'RasterImage'
        emit_image(xml, obj, idx)
      else
        emit_cut(xml, process_type, idx)
      end
    end
  end

  private

  attr_reader :process_types, :process_type_to_obj, :process_type_to_idx, :process_params

  def emit_image(xml, obj, idx)
    settings  = obj['image_settings'] || {}
    dither    = DITHER_MAP[settings['filtering_type']] || "stucki"
    name      = "C#{idx.to_s.rjust(2, '0')}"
    params    = process_params[obj['process_type']] || {}
    max_power = params[:max_power]&.to_s || "0"
    speed     = params[:speed]&.to_s     || "0"

    xml.open("CutSetting_Img", "type" => "Image")
    xml.leaf("index",      "Value" => idx.to_s)
    xml.leaf("name",       "Value" => name)
    xml.leaf("maxPower",   "Value" => max_power)
    xml.leaf("maxPower2",  "Value" => max_power)
    xml.leaf("speed",      "Value" => speed)
    xml.leaf("priority",   "Value" => idx.to_s)
    xml.leaf("ditherMode", "Value" => dither)
    xml.close("CutSetting_Img")
  end

  def emit_cut(xml, process_type, idx)
    info      = PROCESS_TYPE_MAP[process_type] || { type: "Cut", name: "C#{idx.to_s.rjust(2, '0')}" }
    params    = process_params[process_type] || {}
    max_power = params[:max_power]&.to_s || "0"
    speed     = params[:speed]&.to_s     || "0"

    xml.open("CutSetting", "type" => info[:type])
    xml.leaf("index",     "Value" => idx.to_s)
    xml.leaf("name",      "Value" => info[:name])
    xml.leaf("maxPower",  "Value" => max_power)
    xml.leaf("maxPower2", "Value" => max_power)
    xml.leaf("speed",     "Value" => speed)
    xml.leaf("priority",  "Value" => idx.to_s)
    xml.close("CutSetting")
  end
end
