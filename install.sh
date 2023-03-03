#!/usr/bin/env bash

GITHUB_USER=thesandybridge
GITHUB_REPO=random_file
BINARY=gf_amd64

download_url=$(curl -s https://api.github.com/repos/${GITHUB_USER}/${GITHUB_REPO}/releases/latest \
| grep "browser_download_url.*amd64"  \
| cut -d : -f 2,3 \
| tr -d \" \
| xargs)

wget $download_url -O ~/.local/bin/gf && chmod +x ~/.local/bin/gf



