#!/bin/bash
# Hook: Check for migration-related changes and prompt for status update
# Event: Stop

PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"
MIGRATION_FILE="$PROJECT_DIR/MIGRATION_STATUS.md"

# Check if any view files were modified
MODIFIED_VIEWS=$(cd "$PROJECT_DIR" && git diff --name-only HEAD 2>/dev/null | grep -E "crates/vizia_core/src/views/.*\.rs$" || true)
MODIFIED_EXAMPLES=$(cd "$PROJECT_DIR" && git diff --name-only HEAD 2>/dev/null | grep -E "examples/.*\.rs$" || true)

# Check if MIGRATION_STATUS.md was modified
STATUS_MODIFIED=$(cd "$PROJECT_DIR" && git diff --name-only HEAD 2>/dev/null | grep "MIGRATION_STATUS.md" || true)

if [ -n "$MODIFIED_VIEWS" ] || [ -n "$MODIFIED_EXAMPLES" ]; then
    if [ -z "$STATUS_MODIFIED" ]; then
        echo ""
        echo "=== MIGRATION STATUS UPDATE NEEDED ==="
        echo ""
        echo "View files were modified but MIGRATION_STATUS.md was not updated:"

        if [ -n "$MODIFIED_VIEWS" ]; then
            echo ""
            echo "Modified core views:"
            echo "$MODIFIED_VIEWS" | sed 's/^/  - /'
        fi

        if [ -n "$MODIFIED_EXAMPLES" ]; then
            echo ""
            echo "Modified examples:"
            echo "$MODIFIED_EXAMPLES" | sed 's/^/  - /'
        fi

        echo ""
        echo "Consider updating MIGRATION_STATUS.md to reflect these changes."
        echo "=== END MIGRATION STATUS CHECK ==="
    fi
fi

# Update the "Last Updated" timestamp in MIGRATION_STATUS.md if it exists
if [ -f "$MIGRATION_FILE" ]; then
    TODAY=$(date '+%Y-%m-%d')
    if grep -q "Last Updated:" "$MIGRATION_FILE"; then
        # Update the date in place
        sed -i.bak "s/Last Updated: .*/Last Updated: $TODAY/" "$MIGRATION_FILE" && rm -f "$MIGRATION_FILE.bak"
    fi
fi

exit 0
