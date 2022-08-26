# frozen_string_literal: true

require_relative "lib/atc/version"

Gem::Specification.new do |spec|
  spec.name = "atc"
  spec.version = Atc::VERSION
  spec.authors = ["Jan David"]
  spec.email = ["jdno@jdno.dev"]

  spec.summary = "A video game for programmers about air traffic control"
  spec.description = "Ruby client for the video game Auto Traffic Control"
  spec.homepage = "https://auto-traffic-control.com"
  spec.license = "MIT or Apache-2.0"
  spec.required_ruby_version = ">= 3.0.0"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/jdno/auto-traffic-control"
  spec.metadata["changelog_uri"] = "https://github.com/jdno/auto-traffic-control/blob/main/CHANGELOG.md"
  spec.metadata["rubygems_mfa_required"] = "true"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(File.expand_path(__dir__)) do
    `git ls-files -z`.split("\x0").reject do |f|
      (f == __FILE__) || f.match(%r{\A(?:(?:bin|test|spec|features)/|\.(?:git|travis|circleci)|appveyor)})
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.add_dependency "grpc", "~> 1.48.0"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
