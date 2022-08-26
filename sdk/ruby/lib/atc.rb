# frozen_string_literal: true

require_relative "atc/version"

Dir.glob(File.join(__FILE__, "atc", "**", "*.rb"), &method(:require))

module Atc
  class Error < StandardError; end
  # Your code goes here...
end
