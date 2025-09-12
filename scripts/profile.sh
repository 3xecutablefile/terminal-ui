#!/usr/bin/env bash
set -euo pipefail

# Quick CPU/RAM profiler for eDEX-UI (or any process)
#
# Usage examples:
#   scripts/profile.sh                 # auto-detect eDEX/Electron, 1s, 120 samples, label=active
#   scripts/profile.sh -l idle -n 300  # 300 samples labeled 'idle'
#   scripts/profile.sh -p 12345        # profile specific PID
#   scripts/profile.sh -P "myproc"       # match by process name regex
#   scripts/profile.sh -i 0.5 -d 60    # 0.5s interval for 60 seconds
#
# Output: CSV written to ./profile_YYYYmmdd_HHMMSS.csv

interval=1
samples=120
duration=""
label="active"
pid=""
pattern="(eDEX-UI|Electron)"

while getopts ":i:n:d:l:p:P:h" opt; do
  case $opt in
    i) interval="$OPTARG" ;;
    n) samples="$OPTARG" ;;
    d) duration="$OPTARG" ;;
    l) label="$OPTARG" ;;
    p) pid="$OPTARG" ;;
    P) pattern="$OPTARG" ;;
    h)
      echo "Usage: $0 [-i interval] [-n samples] [-d duration_secs] [-l label] [-p pid] [-P pattern]";
      exit 0
      ;;
    *) echo "Unknown option"; exit 1 ;;
  esac
done

ts() { date +"%Y-%m-%d %H:%M:%S"; }

select_pid() {
  if [[ -n "$pid" ]]; then
    echo "$pid"
    return
  fi
  if command -v pgrep >/dev/null 2>&1; then
    # pick the process with largest RSS among matches
    local cand
    cand=$(pgrep -f "$pattern" || true)
    if [[ -n "$cand" ]]; then
      local best_rss=0 best_pid=""
      for p in $cand; do
        rss=$(ps -p "$p" -o rss= | tr -d ' ' || echo 0)
        rss=${rss:-0}
        if [[ "$rss" =~ ^[0-9]+$ ]] && (( rss > best_rss )); then
          best_rss=$rss; best_pid=$p
        fi
      done
      if [[ -n "$best_pid" ]]; then echo "$best_pid"; return; fi
    fi
  fi
  echo "" # none
}

if [[ -n "$duration" ]]; then
  # compute samples from duration when provided
  samples=$(awk -v d="$duration" -v i="$interval" 'BEGIN { printf("%d", (d/i)+0.5) }')
fi

pid=$(select_pid)
if [[ -z "$pid" ]]; then
  echo "Could not find a matching process. Use -p PID or -P pattern." >&2
  exit 2
fi

out="profile_$(date +%Y%m%d_%H%M%S).csv"
echo "timestamp,pid,cpu_percent,mem_percent,rss_kb,label" > "$out"
echo "Profiling PID $pid every ${interval}s for ${samples} samples -> $out" >&2

for ((i=1; i<=samples; i++)); do
  if ! ps -p "$pid" > /dev/null 2>&1; then
    echo "Process $pid ended" >&2
    break
  fi
  # %cpu %mem rss(KB)
  line=$(ps -p "$pid" -o %cpu=,%mem=,rss= | awk '{gsub(/,/,".",$1); printf "%s,%s,%s", $1,$2,$3}')
  echo "$(ts),$pid,$line,$label" >> "$out"
  sleep "$interval"
done

echo "Done -> $out" >&2

