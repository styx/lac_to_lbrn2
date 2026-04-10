require_relative 'types'

module PathParser
  def self.parse(path_data)
    subpaths     = []
    current_segs = nil
    tokens       = path_data.scan(/[MCLZz]|-?\d+\.?\d*(?:[eE][+-]?\d+)?/)
    i            = 0

    while i < tokens.length
      cmd = tokens[i]
      i  += 1

      case cmd
      when 'M'
        current_segs = [Segment.new(:move, [tokens[i].to_f, tokens[i + 1].to_f])]
        i           += 2
        subpaths << current_segs
        while i < tokens.length && tokens[i] !~ /\A[A-Za-z]\z/
          current_segs << Segment.new(:line, [tokens[i].to_f, tokens[i + 1].to_f])
          i += 2
        end
      when 'C'
        while i < tokens.length && tokens[i] !~ /\A[A-Za-z]\z/
          ex   = tokens[i].to_f;     ey   = tokens[i + 1].to_f
          cp1x = tokens[i + 2].to_f; cp1y = tokens[i + 3].to_f
          cp2x = tokens[i + 4].to_f; cp2y = tokens[i + 5].to_f
          current_segs << Segment.new(:bezier, [cp1x, cp1y, cp2x, cp2y, ex, ey])
          i += 6
        end
      when 'L'
        while i < tokens.length && tokens[i] !~ /\A[A-Za-z]\z/
          current_segs << Segment.new(:line, [tokens[i].to_f, tokens[i + 1].to_f])
          i += 2
        end
      when 'Z', 'z'
      end
    end

    subpaths
  end
end
