#!/usr/bin/env bash
set -uCo pipefail
clear
cd "$(dirname -- "${BASH_SOURCE[0]}")" || exit 1

function command_exists() {
	local cmd=''

	for cmd in "$@"; do
		if ! command -v -- "$cmd" &> '/dev/null'; then
			return 1
		fi
	done
}

function get_distro_id() {
	local release_info_file_path='/etc/os-release'

	if [[ ! -s "$release_info_file_path" ]]; then
		return 1
	fi

	grep -- '^ID_LIKE=.*$' "$release_info_file_path" ||
	grep -- '^ID=.*$' "$release_info_file_path"
}

if ! command_exists cargo; then
	curl \
		--proto '=https' \
		--tlsv1.2 \
		--silent \
		--show-error \
		--fail \
		'https://sh.rustup.rs' |
		sh
fi

if ! cargo install --list --quiet | grep --quiet -- '^cargo-watch v.*:$'; then
	cargo install -- 'cargo-watch'
fi

if ! command_exists lua5.4 luarocks; then
	case "$(get_distro_id)" in
		('debian'|'ubuntu')
			sudo -- sh -c '
				apt update &&
				apt upgrade --yes &&
				apt install --yes -- lua5.4 luarocks
			' || exit 1
		;;
		('arch')
			sudo -- pacman \
				-Syu \
				--needed \
				--noconfirm \
				-- lua luarocks ||
				exit 1
		;;
		(*)
			exit 1
		;;
	esac
fi

cargo watch \
	--clear \
	--watch './src/' \
	-- \
	"${BASH:-bash}" -- './test.sh'
