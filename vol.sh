#!/bin/sh
# vol.sh — scroll/click para la statusbar de DWM
# Uso: vol.sh up | down | mute | mixer
# Poner en config.h de DWM:
#   { 0, XF86XK_AudioRaiseVolume, spawn, SHCMD("~/.local/bin/vol.sh up")   },
#   { 0, XF86XK_AudioLowerVolume, spawn, SHCMD("~/.local/bin/vol.sh down") },
#   { 0, XF86XK_AudioMute,        spawn, SHCMD("~/.local/bin/vol.sh mute") },
#   { MODKEY, XK_v,               spawn, SHCMD("volume-mixer")              },

STEP=5

case "$1" in
    up)   pactl set-sink-volume @DEFAULT_SINK@ "+${STEP}%" ;;
    down) pactl set-sink-volume @DEFAULT_SINK@ "-${STEP}%" ;;
    mute) pactl set-sink-mute   @DEFAULT_SINK@ toggle      ;;
    mixer) exec volume-mixer &                              ;;
    *)
        echo "Uso: $0 up|down|mute|mixer"
        exit 1
        ;;
esac
