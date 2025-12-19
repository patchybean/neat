# Homebrew Formula for neatcli
# This file should be placed in your homebrew-tap repository
#
# Repository: https://github.com/patchybean/homebrew-tap
# File path: Formula/neatcli.rb
#
# Users can then install with:
#   brew tap patchybean/tap
#   brew install neatcli

class Neatcli < Formula
  desc "A smart CLI tool to organize and clean up messy directories"
  homepage "https://github.com/patchybean/neat"
  version "0.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/patchybean/neat/releases/download/v#{version}/neatcli-aarch64-apple-darwin.tar.gz"
      # sha256 "REPLACE_WITH_ACTUAL_SHA256"
    else
      url "https://github.com/patchybean/neat/releases/download/v#{version}/neatcli-x86_64-apple-darwin.tar.gz"
      # sha256 "REPLACE_WITH_ACTUAL_SHA256"
    end
  end

  on_linux do
    url "https://github.com/patchybean/neat/releases/download/v#{version}/neatcli-x86_64-unknown-linux-gnu.tar.gz"
    # sha256 "REPLACE_WITH_ACTUAL_SHA256"
  end

  def install
    bin.install "neatcli"
  end

  test do
    system "#{bin}/neatcli", "--version"
  end
end
