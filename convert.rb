#!/usr/bin/env ruby

require 'optparse'
require_relative 'lib/converter'

options = {}

OptionParser.new do |o|
  o.banner = "Usage: #{$0} [options] input.lac [output.lbrn2]"
  o.on('--normalize', 'Shift all shapes so the scene bounding box starts at origin') do
    options[:normalize] = true
  end
end.parse!

abort "Usage: #{$0} [options] input.lac [output.lbrn2]" unless ARGV.length.between?(1, 2)

input = ARGV[0]
output = ARGV[1] || input.sub(/\.lac$/i, '.lbrn2')

Converter.new(input, output, normalize: options[:normalize]).run
