class Transform
  IDENTITY = new.tap { |t| t.instance_variable_set(:@m, [1.0, 0.0, 0.0, 1.0, 0.0, 0.0].freeze) }.freeze

  def self.parse(str)
    from_array((str || '1 0 0 1 0 0').split.map(&:to_f))
  end

  def self.from_array(arr)
    new.tap { |t| t.instance_variable_set(:@m, arr.freeze) }
  end

  def compose(other)
    a1, b1, c1, d1, tx1, ty1 = @m
    a2, b2, c2, d2, tx2, ty2 = other.to_a
    Transform.from_array([
      a2 * a1 + c2 * b1,  b2 * a1 + d2 * b1,
      a2 * c1 + c2 * d1,  b2 * c1 + d2 * d1,
      a2 * tx1 + c2 * ty1 + tx2,
      b2 * tx1 + d2 * ty1 + ty2,
    ])
  end

  def apply(x, y)
    a, b, c, d, tx, ty = @m
    [a * x + c * y + tx, b * x + d * y + ty]
  end

  def scale_x = @m[0]
  def scale_y = @m[3]
  def tx      = @m[4]
  def ty      = @m[5]
  def to_a    = @m
end
