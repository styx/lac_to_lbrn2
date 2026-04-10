require_relative '../transformers/raster_image_transformer'
require_relative '../transformers/ellipse_transformer'
require_relative '../transformers/path_transformer'

class ShapeVisitor
  def initialize(process_type_to_idx, objects_path)
    @process_type_to_idx = process_type_to_idx
    @registry            = {
      'RasterImage'   => RasterImageTransformer.new(objects_path),
      'EllipseObject' => EllipseTransformer.new,
      'PathObject'    => PathTransformer.new,
    }
  end

  def visit(instances, xml, offset)
    instances.each do |inst|
      transformer = @registry[inst.obj['type']]
      next unless transformer

      cut_index = @process_type_to_idx[inst.obj['process_type']] || 0
      transformer.transform(inst, xml, cut_index, offset)
    end
  end

  private

  attr_reader :process_type_to_idx, :registry
end
