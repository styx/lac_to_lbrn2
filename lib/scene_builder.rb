require_relative 'types'
require_relative 'transform'
require_relative 'path_parser'

class SceneBuilder
  def initialize(canvas, obj_map)
    @canvas  = canvas
    @obj_map = obj_map
  end

  def build
    instances = []
    @canvas['components'].each { |comp| walk(comp, Transform::IDENTITY, instances) }
    instances
  end

  def compute_offset(instances)
    min_x = Float::INFINITY
    min_y = Float::INFINITY

    instances.each do |inst|
      obj = inst.obj
      t   = inst.transform

      case obj['type']
      when 'PathObject'
        PathParser.parse(obj['path_data']).each do |segs|
          segs.each do |seg|
            case seg.type
            when :move, :line
              wx, wy = t.apply(seg.params[0], seg.params[1])
              min_x  = [min_x, wx].min
              min_y  = [min_y, wy].min
            when :bezier
              [[0, 1], [2, 3], [4, 5]].each do |xi, yi|
                wx, wy = t.apply(seg.params[xi], seg.params[yi])
                min_x  = [min_x, wx].min
                min_y  = [min_y, wy].min
              end
            end
          end
        end
      when 'EllipseObject'
        wx, wy = t.apply(obj['center_x'], obj['center_y'])
        min_x  = [min_x, wx - obj['radius_x']].min
        min_y  = [min_y, wy - obj['radius_y']].min
      when 'RasterImage'
        w_mm = obj['width']  * t.scale_x.abs
        h_mm = obj['height'] * t.scale_y.abs
        min_x = [min_x, t.tx - w_mm / 2.0].min
        min_y = [min_y, t.ty - h_mm / 2.0].min
      end
    end

    [min_x.finite? ? min_x : 0.0, min_y.finite? ? min_y : 0.0]
  end

  private

  LEAF_TYPES = %w[PathObject EllipseObject RasterImage].freeze

  def walk(comp, parent_transform, instances)
    obj = @obj_map[comp['obj_id']]
    return unless obj

    total = parent_transform.compose(Transform.parse(comp['transform']))

    if obj['type'] == 'AttachedGroup'
      obj['components']&.each { |child| walk(child, total, instances) }
    elsif LEAF_TYPES.include?(obj['type']) && obj['color']
      instances << Instance.new(obj, total)
    end
  end
end
