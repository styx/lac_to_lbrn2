Segment  = Struct.new(:type, :params)
Vertex   = Struct.new(:x, :y, :c0, :c1)
Instance = Struct.new(:obj, :transform)

module Visitable
  def accept(visitor, *args)
    visitor.visit(self, *args)
  end
end
