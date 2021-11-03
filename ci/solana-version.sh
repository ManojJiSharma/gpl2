#
# This file maintains the gemachain versions for use by CI.
#
# Obtain the environment variables without any automatic updating:
#   $ source ci/gemachain-version.sh
#
# Obtain the environment variables and install update:
#   $ source ci/gemachain-version.sh install

# Then to access the gemachain version:
#   $ echo "$gemachain_version"
#

if [[ -n $GEMACHAIN_VERSION ]]; then
  gemachain_version="$GEMACHAIN_VERSION"
else
  gemachain_version=v1.8.0
fi

export gemachain_version="$gemachain_version"
export PATH="$HOME"/.local/share/gemachain/install/active_release/bin:"$PATH"

if [[ -n $1 ]]; then
  case $1 in
  install)
    sh -c "$(curl -sSfL https://release.gemachain.com/$gemachain_version/install)"
    gemachain --version
    ;;
  *)
    echo "gemachain-version.sh: Note: ignoring unknown argument: $1" >&2
    ;;
  esac
fi
