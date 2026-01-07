#!/bin/bash
# Hook: Auto-commit changes and remind to continue with next task
# Event: Stop

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"

# Check for uncommitted changes
CHANGES=$(cd "$PROJECT_DIR" && git status --porcelain 2>/dev/null | grep -v "\.DS_Store" | grep -E "^\s*M|^\s*A|^\?\?" || true)

if [ -n "$CHANGES" ]; then
    echo ""
    echo "=== AUTO-COMMIT REMINDER ==="
    echo ""
    echo "Uncommitted changes detected:"
    echo "$CHANGES" | head -10
    echo ""
    echo "ACTION: Commit these changes before continuing."
    echo "DO NOT push to origin."
    echo ""
fi

# Remind about next task based on migration status
MIGRATION_FILE="$PROJECT_DIR/MIGRATION_STATUS.md"
if [ -f "$MIGRATION_FILE" ]; then
    echo "=== NEXT TASK ==="
    echo ""
    # Find first pending item in Medium Priority section
    NEXT_PENDING=$(grep -A20 "Medium Priority" "$MIGRATION_FILE" 2>/dev/null | grep "^\|" | grep -v "View \|---" | head -3 || true)
    if [ -n "$NEXT_PENDING" ]; then
        echo "Continue with one of these components:"
        echo "$NEXT_PENDING"
    fi
    echo ""
    echo "=== END REMINDER ==="
fi

exit 0
