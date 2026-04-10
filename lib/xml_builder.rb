class XmlBuilder
  def initialize
    @lines = ['<?xml version="1.0" encoding="UTF-8"?>']
    @depth = 0
  end

  def open(name, attrs = {})
    @lines << "#{pad}<#{name}#{attr_str(attrs)}>"
    @depth += 1
    self
  end

  def close(name)
    @depth -= 1
    @lines << "#{pad}</#{name}>"
    self
  end

  def leaf(name, attrs = {})
    @lines << "#{pad}<#{name}#{attr_str(attrs)}/>"
    self
  end

  def inline(name, text, attrs = {})
    @lines << "#{pad}<#{name}#{attr_str(attrs)}>#{text}</#{name}>"
    self
  end

  def to_s
    @lines.join("\n") + "\n"
  end

  private

  def pad
    "    " * @depth
  end

  def attr_str(attrs)
    attrs.map { |k, v| " #{k}=\"#{escape(v.to_s)}\"" }.join
  end

  def escape(s)
    s.gsub('&', '&amp;').gsub('"', '&quot;').gsub('<', '&lt;').gsub('>', '&gt;')
  end
end
