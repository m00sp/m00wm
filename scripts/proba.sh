#!/bin/sh
# Make sure we only run once
echo ola &
pid=$$
pgrep -f proba.sh | grep -v "^$pid$" | xargs -I{} kill {}
sleep 10
echo ola de novo
#export SPAM=mimomu
echo $SPAM
[[ -z "$SPAM" ]]; echo ola de SPAM &
[[ -z "$XDG_RUNTIME_DIR" ]]; echo ola de XDG_RUNTIME_DIR &