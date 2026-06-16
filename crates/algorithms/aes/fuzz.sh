#!/usr/bin/env bash
set -e

# Color(s)
BLUE='\033[1;34m'
RESET='\033[0m'

# Use the first argument as the time limit, default to 60 if not provided
TIME_LIMIT=${1:-60}

echo "Running all fuzz targets for $TIME_LIMIT seconds each..."

for target in $(cargo fuzz list); do
    echo -e "${BLUE}==================================================="
    echo -e " Fuzzing target: $target"
    echo -e "===================================================${RESET}"
    
    # Run the fuzzer
    cargo +nightly fuzz run "$target" -- -max_total_time="$TIME_LIMIT"

    # Check if the fuzzer found a crash (exit code is non-zero)
    if [ $? -ne 0 ]; then
        echo "🚨 Crash detected in target: $target! Stopping."
        exit 1
    fi

    echo ""
done

echo "✅ All fuzz targets completed successfully!"
