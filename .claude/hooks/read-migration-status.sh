#!/bin/bash
# Hook: Read MIGRATION_STATUS.md and provide context on every prompt
# Event: UserPromptSubmit

MIGRATION_FILE="${CLAUDE_PROJECT_DIR:-$(pwd)}/MIGRATION_STATUS.md"

if [ -f "$MIGRATION_FILE" ]; then
    # Extract key information from the migration status
    echo "=== MIGRATION STATUS CONTEXT ==="
    echo ""

    # Get the last updated date if present
    LAST_UPDATED=$(grep -m1 "Last Updated:" "$MIGRATION_FILE" 2>/dev/null || echo "Unknown")
    echo "Last Updated: $LAST_UPDATED"
    echo ""

    # Extract progress summary (the table at the top)
    echo "Current Progress:"
    grep -A4 "Migration Progress Summary" "$MIGRATION_FILE" 2>/dev/null | grep -E "^\|" | head -5
    echo ""

    # List migrated views
    echo "Migrated Views:"
    grep -A20 "Core Views - Migrated to Signals" "$MIGRATION_FILE" 2>/dev/null | grep "^\| " | grep -v "View \|---" | head -10
    echo ""

    # List high priority pending
    echo "High Priority Pending:"
    grep -A10 "High Priority (Simple State)" "$MIGRATION_FILE" 2>/dev/null | grep "^\| " | grep -v "View \|---" | head -5
    echo ""

    echo "=== END MIGRATION CONTEXT ==="
    echo ""
    echo "Remember: Update MIGRATION_STATUS.md when completing migration work."
fi

exit 0
