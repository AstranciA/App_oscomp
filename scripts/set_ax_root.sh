#/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <AX_SOURCE_ROOT> <AX_ROOT>"
    exit 1
fi

AX_ROOT=$1

if [ ! -d "$AX_ROOT" ]; then
    echo "AX_ROOT ($AX_ROOT) does not exist"
    exit 1
fi

mkdir -p .cargo
sed -e "s|%AX_ROOT%|$AX_ROOT|g" scripts/config.toml > .cargo/config.toml

echo "Set AX_ROOT (ArceOS directory) to $AX_ROOT"
