require 'json'
require 'zip'
require 'tmpdir'
require_relative 'xml_builder'
require_relative 'scene_builder'
require_relative 'visitors/cut_setting_visitor'
require_relative 'visitors/shape_visitor'

class Converter
  def initialize(input, output, normalize: false)
    @input     = input
    @output    = output
    @normalize = normalize
  end

  def run
    Dir.mktmpdir do |tmp|
      extract(tmp)

      json_path     = File.join(tmp, '2D', '2dmodel.json')
      settings_path = File.join(tmp, 'Metadata2D', 'project_settings.json')
      objects_path  = File.join(tmp, '2D', 'Objects')

      data    = JSON.parse(File.read(json_path))
      canvas  = data['canvas_list'][0]
      obj_map = canvas['obj_list'].each_with_object({}) { |o, m| m[o['obj_id']] = o }

      inject_process_types(obj_map, settings_path)

      process_params = load_process_params(tmp, settings_path)

      scene     = SceneBuilder.new(canvas, obj_map)
      instances = scene.build

      process_types       = instances.map { |i| i.obj['process_type'] }.uniq.compact
      process_type_to_idx = build_index_map(process_types)
      process_type_to_obj = instances.each_with_object({}) { |i, m| m[i.obj['process_type']] ||= i.obj }
      offset              = @normalize ? scene.compute_offset(instances) : [0.0, 0.0]

      xml = XmlBuilder.new
      xml.open("LightBurnProject",
        "AppVersion"     => "2.1.00",
        "DeviceName"     => "LaserPecker_LX1_Rotation",
        "FormatVersion"  => "1",
        "MaterialHeight" => "0",
        "MirrorX"        => "False",
        "MirrorY"        => "True",
        "AskForSendName" => "True"
      )

      emit_header(xml)
      CutSettingVisitor.new(process_types, process_type_to_obj, process_type_to_idx, process_params).visit(xml)
      ShapeVisitor.new(process_type_to_idx, objects_path).visit(instances, xml, offset)
      xml.leaf("Notes", "ShowOnLoad" => "0", "Notes" => "")
      xml.close("LightBurnProject")

      File.write(@output, xml.to_s)
      puts "Done: #{instances.length} objects → #{@output}"
    end
  end

  private

  attr_reader :input, :output, :normalize

  def extract(tmp)
    Zip::File.open(@input) do |zip|
      zip.each do |entry|
        next unless entry.name.start_with?('2D/', 'Metadata2D/')
        dest = File.join(tmp, entry.name)
        FileUtils.mkdir_p(File.dirname(dest))
        entry.extract(dest)
      end
    end
  end

  def inject_process_types(obj_map, settings_path)
    return unless File.exist?(settings_path)

    settings = JSON.parse(File.read(settings_path))
    object_settings = settings.dig('canvas_settings', 0, 'object_settings') || []
    object_settings.each do |s|
      obj = obj_map[s['obj_id']]
      obj['process_type'] = s['process_type'] if obj
    end
  end

  def load_process_params(tmp, settings_path)
    return {} unless File.exist?(settings_path)

    settings  = JSON.parse(File.read(settings_path))
    batch_list = settings.dig('canvas_settings', 0, 'making_batch_list') || []

    batch_list.each_with_object({}) do |batch, result|
      material  = batch['material_settings_name'].to_s
      machine   = settings.dig('project_settings', 'machine_settings_name').to_s
      next if material.empty?

      pattern     = File.join(tmp, 'Metadata2D', "#{material} Process @*.config")
      config_path = Dir.glob(pattern).first
      next unless config_path

      config = JSON.parse(File.read(config_path))
      PROCESS_TYPE_MAP.each_key do |pt|
        next if result.key?(pt)
        section = config[pt]
        next unless section
        result[pt] = {
          max_power: section['max_power'],
          speed:     section['speed'],
        }
      end
    end
  end

  def build_index_map(process_types)
    fixed = PROCESS_TYPE_MAP.transform_values { |v| v[:index] }
    next_free = 3
    process_types.each_with_object({}) do |pt, map|
      if fixed.key?(pt)
        map[pt] = fixed[pt]
      else
        map[pt] = next_free
        next_free += 1
      end
    end
  end

  def emit_header(xml)
    xml.open("VariableText")
    [["Start", "0"], ["End", "999"], ["Current", "0"], ["Increment", "1"], ["AutoAdvance", "0"]].each do |n, v|
      xml.leaf(n, "Value" => v)
    end
    xml.close("VariableText")

    xml.open("UIPrefs")
    [
      ["Optimize_ByLayer",           "0"],
      ["Optimize_ByGroup",           "-1"],
      ["Optimize_ByPriority",        "1"],
      ["Optimize_WhichDirection",    "0"],
      ["Optimize_InnerToOuter",      "1"],
      ["Optimize_ByDirection",       "0"],
      ["Optimize_ReduceTravel",      "1"],
      ["Optimize_HideBacklash",      "0"],
      ["Optimize_ReduceDirChanges",  "0"],
      ["Optimize_ChooseCorners",     "0"],
      ["Optimize_AllowReverse",      "1"],
      ["Optimize_RemoveOverlaps",    "0"],
      ["Optimize_OptimalEntryPoint", "0"],
      ["Optimize_OverlapDist",       "0.025"],
    ].each { |n, v| xml.leaf(n, "Value" => v) }
    xml.close("UIPrefs")
  end
end
