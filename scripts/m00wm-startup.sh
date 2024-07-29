#!/bin/sh
# ----------------------------------------
# Bootstrap the start of a penrose session
# >> This get's run on restart as well!
# ----------------------------------------
if [ -z "$XDG_RUNTIME_DIR" ]; then
	XDG_RUNTIME_DIR="/tmp/$(id -u)-runtime-dir"

	mkdir -pm 0700 "$XDG_RUNTIME_DIR"
	export XDG_RUNTIME_DIR
fi
#Podré exportar variaveis desde aquí?
#export XDG_SESSION_TYPE=wayland
#export XDG_SESSION_DESKTOP=sway
#export XDG_CURRENT_DESKTOP=sway
#export MIMOMU=mimomu

#Certificando que rode só uma vez
pid=$$
pgrep -f m00wm-startup.sh | grep -v "^$pid$" |xargs -I{} kill {}

<<<<<<< HEAD
#/usr/libexec/pipewire-launcher &
#amixer;
#/home/mimomu/.screenlayout/monitor.sh;
#feh --bg-max ~/Downloads/buddha.png;
#setxkbmap br;
xset s off -dpms;
#pkill -p picom; picom & 
=======
pkill -p plasmashell;
#/usr/libexec/pipewire-launcher &
#amixer;
#/home/mimomu/.screenlayout/monitor.sh;
feh --bg-max ~/Downloads/buddha.png;
#setxkbmap br;
xset s off -dpms;
pkill -p picom; picom & 
>>>>>>> develop
#xautolock -time 10 -locker "i3lock -c 3f0000 -f" &
#firefox &
#konsole &
#sleep 1;
#i3lock -c 3f0000 -f
#export $(dbus-launch)
#/usr/libexec/pipewire-launcher


# Set screen resolutions (add additional screens here)
#xrandr --output HDMI-2 --auto --right-of eDP-1 &

# fix a couple of quirks with my thinkpad: enable tap-click for the touchpad
# and slow down the track point accelleration
#xinput --set-prop "11" "libinput Tapping Enabled" 1
#xinput --set-prop "12" "libinput Accel Speed" 0.0

# Keyboard overrides
#setxkbmap -option caps:super
#xsetroot -cursor_name left_ptr

# pkill -fi stalonetray; stalonetray -bg '#282828' --icon-size 18 &
#pkill -fi picom; picom &
#pkill -fi nm-applet; nm-applet &
#pkill -fi udiskie; udiskie -a -n -t &
#pkill -fi dunst; dunst &
#pkill -fi blueman-applet; blueman-applet &
#pkill -fi xfce4-power-man; xfce4-power-manager &  # for some reason, this ends up running as xcfe4-power-man
#pkill -fi xfce4-screensaver; xfce4-screensaver &
#pkill -fi gnome-keyring-daemon; gnome-keyring-daemon --start --components=pkcs11,secrets,ssh &

#"$HOME/.fehbg"

# see /usr/local/bin/run-penrose
#[[ -z "$RESTARTED" ]]; /usr/local/scripts/unlock-ssh.sh &
