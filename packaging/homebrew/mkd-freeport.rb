# SPDX-FileCopyrightText: 2026 Nathan A Chadwick
# SPDX-License-Identifier: MIT

class MkdFreeport < Formula
  desc "Discover and reserve TCP/UDP ports over IPv4 and IPv6 for scripts and AI agents"
  homepage "https://github.com/monkeyking-hq/mkd-freeport"
  version "0.4.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/monkeyking-hq/mkd-freeport/releases/download/v#{version}/mkd-freeport-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_MACOS_ARM64_SHA256"
    end
    on_intel do
      url "https://github.com/monkeyking-hq/mkd-freeport/releases/download/v#{version}/mkd-freeport-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_MACOS_X86_64_SHA256"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/monkeyking-hq/mkd-freeport/releases/download/v#{version}/mkd-freeport-v#{version}-aarch64-unknown-linux-musl.tar.gz"
      sha256 "REPLACE_WITH_LINUX_ARM64_SHA256"
    end
    on_intel do
      url "https://github.com/monkeyking-hq/mkd-freeport/releases/download/v#{version}/mkd-freeport-v#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "REPLACE_WITH_LINUX_X86_64_SHA256"
    end
  end

  def install
    bin.install "mkd-freeport"
    man1.install "man/mkd-freeport.1"
    bash_completion.install "completions/mkd-freeport.bash" => "mkd-freeport"
    zsh_completion.install "completions/_mkd-freeport"
    fish_completion.install "completions/mkd-freeport.fish"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/mkd-freeport --version")
    port = shell_output("#{bin}/mkd-freeport --plain").strip.to_i
    assert_operator port, :>, 1024
  end
end
