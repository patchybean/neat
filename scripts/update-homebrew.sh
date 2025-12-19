#!/bin/bash
# Script to update Homebrew formula with new release SHA256 checksums
#
# Usage: ./scripts/update-homebrew.sh v0.1.0
#

VERSION=${1:-"v0.1.0"}
REPO="patchybean/neat"

echo "Updating Homebrew formula for version $VERSION..."

# Download and calculate SHA256 for each platform
PLATFORMS=("aarch64-apple-darwin" "x86_64-apple-darwin" "x86_64-unknown-linux-gnu")

for platform in "${PLATFORMS[@]}"; do
    URL="https://github.com/$REPO/releases/download/$VERSION/neatcli-$platform.tar.gz"
    echo "Downloading $URL..."
    
    SHA=$(curl -sL "$URL" | shasum -a 256 | cut -d' ' -f1)
    echo "$platform: $SHA"
done

echo ""
echo "Update the homebrew/neatcli.rb file with these SHA256 values"
echo "Then push to your homebrew-tap repository"
