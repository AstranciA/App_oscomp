#36M	sdcard-rv.img/bin/bash
echo offline=$OFFLINE

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

if [ -z "$OFFLINE" ]; then
    sed -e "s|%AX_ROOT%|$AX_ROOT|g" scripts/config.toml > .cargo/config.toml
else
    sed -e "s|%AX_ROOT%|$AX_ROOT|g" -e "s|offline\s*=\s*false|offline = true|" scripts/config.toml > .cargo/config.toml
fi

echo "Set AX_ROOT (ArceOS directory) to $AX_ROOT"
