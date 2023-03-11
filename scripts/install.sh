#!/bin/env bash
if [ ! -f "./build/htodo" ]; then
    echo "WARN: Compile the program first"
    exit 0
fi

read -p "$* Use ~/.local/bin instead of /usr/bin? [y/N]: " yn

case $yn in
    [Yy])
        sudo mv ./build/htodo /usr/bin
        exit 0
    ;;

    *)
        mv ./build/htodo ~/.local/bin
    ;;
esac
