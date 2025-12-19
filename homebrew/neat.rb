# Homebrew Formula for Neat
# This file should be placed in your homebrew-tap repository
#
# Repository: https://github.com/patchybean/homebrew-tap
# File path: Formula/neat.rb
#
# Users can then install with:
#   brew tap patchybean/tap
#   brew install neat

class Neat < Formula
  desc "A smart CLI tool to organize and clean up messy directories"
  homepage "https://github.com/patchybean/neat"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/patchybean/neat/releases/download/v#{version}/neat-aarch64-apple-darwin.tar.gz"
      # sha256 "REPLACE_WITH_ACTUAL_SHA256"
    else
      url "https://github.com/patchybean/neat/releases/download/v#{version}/neat-x86_64-apple-darwin.tar.gz"
      # sha256 "REPLACE_WITH_ACTUAL_SHA256"
    end
  end

  on_linux do
    url "https://github.com/patchybean/neat/releases/download/v#{version}/neat-x86_64-unknown-linux-gnu.tar.gz"
    # sha256 "REPLACE_WITH_ACTUAL_SHA256"
  end

  def install
    bin.install "neat"
  end

  test do
    system "#{bin}/neat", "--version"
  end
end
