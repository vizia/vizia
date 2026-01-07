#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"

SIGNALS_ROOT=${SIGNALS_ROOT:-"$ROOT_DIR"}
MAIN_ROOT=${MAIN_ROOT:-"$HOME/vizia"}
SIGNALS_TITLE_SUFFIX=${SIGNALS_TITLE_SUFFIX:-" (signals)"}
MAIN_TITLE_SUFFIX=${MAIN_TITLE_SUFFIX:-" (main)"}
EXAMPLES_CONFIG=${EXAMPLES_CONFIG:-"$ROOT_DIR/scripts/examples.json"}

if [[ ! -f "$SIGNALS_ROOT/Cargo.toml" ]]; then
  echo "Signals workspace not found at ${SIGNALS_ROOT}. Set SIGNALS_ROOT to override." >&2
  exit 1
fi

if [[ ! -f "$MAIN_ROOT/Cargo.toml" ]]; then
  echo "Main workspace not found at ${MAIN_ROOT}. Set MAIN_ROOT to override." >&2
  exit 1
fi

CARGO_BIN=${CARGO_BIN:-}
if [[ -z "${CARGO_BIN}" ]]; then
  if command -v cargo >/dev/null 2>&1; then
    CARGO_BIN="cargo"
  elif [[ -x "$HOME/.cargo/bin/cargo" ]]; then
    CARGO_BIN="$HOME/.cargo/bin/cargo"
  else
    echo "cargo not found in PATH or at $HOME/.cargo/bin/cargo" >&2
    exit 1
  fi
fi

PYTHON_BIN=${PYTHON_BIN:-}
if [[ -z "${PYTHON_BIN}" ]]; then
  if command -v python3 >/dev/null 2>&1; then
    PYTHON_BIN="python3"
  elif command -v python >/dev/null 2>&1; then
    PYTHON_BIN="python"
  else
    echo "python3 not found in PATH" >&2
    exit 1
  fi
fi

metadata_err="$(mktemp)"
metadata_signals="$("$CARGO_BIN" metadata --format-version 1 --no-deps --manifest-path "$SIGNALS_ROOT/Cargo.toml" 2> "$metadata_err" || true)"
if [[ -z "${metadata_signals}" ]]; then
  echo "Failed to read Cargo metadata for signals workspace." >&2
  if [[ -s "$metadata_err" ]]; then
    cat "$metadata_err" >&2
  fi
  rm -f "$metadata_err"
  exit 1
fi
rm -f "$metadata_err"

metadata_err="$(mktemp)"
metadata_main="$("$CARGO_BIN" metadata --format-version 1 --no-deps --manifest-path "$MAIN_ROOT/Cargo.toml" 2> "$metadata_err" || true)"
if [[ -z "${metadata_main}" ]]; then
  echo "Failed to read Cargo metadata for main workspace." >&2
  if [[ -s "$metadata_err" ]]; then
    cat "$metadata_err" >&2
  fi
  rm -f "$metadata_err"
  exit 1
fi
rm -f "$metadata_err"

examples=()
signals_file="$(mktemp)"
main_file="$(mktemp)"
python_script="$(mktemp)"
printf '%s' "$metadata_signals" > "$signals_file"
printf '%s' "$metadata_main" > "$main_file"
cat > "$python_script" <<'PY'
import json
import pathlib
import sys
import os

signals_path = sys.argv[1]
main_path = sys.argv[2]
config_path = sys.argv[3] if len(sys.argv) > 3 else ""

config = None
if config_path and os.path.exists(config_path):
    with open(config_path, "r", encoding="utf-8") as f:
        raw = f.read().strip()
        if raw:
            config = json.loads(raw)
        else:
            config = {}

def read_examples(path):
    with open(path, "r", encoding="utf-8") as f:
        raw = f.read().strip()
    if not raw:
        return {}
    meta = json.loads(raw)
    root = pathlib.Path(meta["workspace_root"]).resolve()
    manifest = root / "Cargo.toml"
    entries = {}
    for pkg in meta.get("packages", []):
        pkg_manifest = pathlib.Path(pkg["manifest_path"]).resolve()
        is_root = "1" if pkg_manifest == manifest else "0"
        for target in pkg.get("targets", []):
            if "example" in target.get("kind", []):
                key = f"{pkg['name']}/{target['name']}"
                entries[key] = is_root
    return entries

signals = read_examples(signals_path)
main = read_examples(main_path)

ordered_keys = []
if config is not None:
    rf = []
    r = []
    missing = []
    for key, flag in config.items():
        in_signals = key in signals
        in_main = key in main
        if not in_signals and not in_main:
            missing.append(key)
            continue
        if flag == "RF":
            rf.append(key)
        elif flag == "R":
            r.append(key)
    ordered_keys = rf + r
    for key in missing:
        print(f"Config entry not found in signals or main: {key}", file=sys.stderr)
else:
    common = sorted(set(signals.keys()) & set(main.keys()))
    ordered_keys = common

for key in ordered_keys:
    name, example = key.split("/", 1)
    sig_root = signals.get(key)
    main_root = main.get(key)
    sig_present = "1" if sig_root is not None else "0"
    main_present = "1" if main_root is not None else "0"
    print(f"{name}|{example}|{sig_present}|{sig_root or 0}|{main_present}|{main_root or 0}")
PY
while IFS= read -r line; do
  examples+=("$line")
done < <("$PYTHON_BIN" "$python_script" "$signals_file" "$main_file" "$EXAMPLES_CONFIG")
rm -f "$signals_file" "$main_file" "$python_script"

if (( ${#examples[@]} == 0 )); then
  if [[ -f "$EXAMPLES_CONFIG" ]]; then
    echo "No examples selected by config: $EXAMPLES_CONFIG" >&2
  else
    echo "No shared examples found between signals and main." >&2
  fi
  exit 0
fi

echo "Running examples side-by-side:"
echo "  signals: ${SIGNALS_ROOT}"
echo "  main:    ${MAIN_ROOT}"

printf 'Found %d selected example(s):\n' "${#examples[@]}"

for entry in "${examples[@]}"; do
  IFS='|' read -r pkg example sig_present is_root_signals main_present is_root_main <<< "${entry}"
  [[ -z "${example}" ]] && continue

  if [[ "${sig_present}" != "1" && "${main_present}" != "1" ]]; then
    continue
  fi

  echo ""
  if [[ "${sig_present}" == "1" && "${main_present}" == "1" ]]; then
    echo "=== Running example: ${pkg}/${example} (signals vs main) ==="
    echo "Close both windows to continue to the next example."
  elif [[ "${sig_present}" == "1" ]]; then
    echo "=== Running example: ${pkg}/${example} (signals only) ==="
    echo "Close the window to continue to the next example."
  else
    echo "=== Running example: ${pkg}/${example} (main only) ==="
    echo "Close the window to continue to the next example."
  fi

  pid_signals=""
  pid_main=""
  if [[ "${sig_present}" == "1" ]]; then
    if [[ "${is_root_signals}" == "1" ]]; then
      VIZIA_TITLE_SUFFIX="$SIGNALS_TITLE_SUFFIX" \
        "$CARGO_BIN" run --manifest-path "$SIGNALS_ROOT/Cargo.toml" --example "${example}" &
    else
      VIZIA_TITLE_SUFFIX="$SIGNALS_TITLE_SUFFIX" \
        "$CARGO_BIN" run --manifest-path "$SIGNALS_ROOT/Cargo.toml" -p "${pkg}" --example "${example}" &
    fi
    pid_signals=$!
  fi

  if [[ "${main_present}" == "1" ]]; then
    if [[ "${is_root_main}" == "1" ]]; then
      VIZIA_TITLE_SUFFIX="$MAIN_TITLE_SUFFIX" \
        "$CARGO_BIN" run --manifest-path "$MAIN_ROOT/Cargo.toml" --example "${example}" &
    else
      VIZIA_TITLE_SUFFIX="$MAIN_TITLE_SUFFIX" \
        "$CARGO_BIN" run --manifest-path "$MAIN_ROOT/Cargo.toml" -p "${pkg}" --example "${example}" &
    fi
    pid_main=$!
  fi

  set +e
  status_signals=0
  status_main=0
  if [[ -n "${pid_signals}" ]]; then
    wait "$pid_signals"
    status_signals=$?
  fi
  if [[ -n "${pid_main}" ]]; then
    wait "$pid_main"
    status_main=$?
  fi
  set -e

  if [[ $status_signals -ne 0 || $status_main -ne 0 ]]; then
    echo "Example ${pkg}/${example} exited with status: signals=${status_signals} main=${status_main}" >&2
    exit 1
  fi
done
