#!/usr/bin/env bash
set -uCo pipefail
clear
cd "$(dirname -- "${BASH_SOURCE[0]}")" || exit 1

eval "$(luarocks path --lua-version '5.4')"
export LUA_CPATH="$PWD/target/debug/lib?.so;$LUA_CPATH"

# The `stty -raw echo` and `tput init` commands ensure that the terminal is
# always reset to normal settings. Without them, a reload will cause it to get
# stuck in raw mode or with mouse input enabled, as `cargo watch` kills the
# currently running command if it detects a file change, thus preventing any
# resetting logic from running as intended.
stty -raw echo
tput init

if cargo build; then
	lua5.4 -- './test.lua'
	echo
fi

stty -raw echo
# Commented out because it clears the screen on some terminal emulators.
#tput init
