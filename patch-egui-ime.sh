#!/usr/local/bin/bash
# Patch egui 0.33 to fix IME blocking backspace/arrow keys
# Based on https://github.com/emilk/egui/pull/7967

set -e

EGUI_FILE=$(find ~/.cargo/registry/src -path "*/egui-0.33*/src/widgets/text_edit/builder.rs" | head -1)

if [ -z "$EGUI_FILE" ]; then
    echo "Error: Could not find egui 0.33 builder.rs"
    echo "Run 'cargo build' first to download dependencies"
    exit 1
fi

echo "Found egui at: $EGUI_FILE"

# Check if already patched
if grep -q "PATCHED.*IME" "$EGUI_FILE"; then
    echo "Already patched - skipping"
    exit 0
fi

# Backup original
cp "$EGUI_FILE" "$EGUI_FILE.backup"

# Apply patch: Comment out the event filtering when IME is enabled
# Lines 934-938 in egui 0.33.3
sed -i.tmp '
    /if state\.ime_enabled {/,/^    }$/ {
        s|^        remove_ime_incompatible_events|        // PATCHED: Allow backspace/arrows - see PR #7967\n        // remove_ime_incompatible_events|
        s|^        // Process IME events first:|        // PATCHED: Don'\''t reorder events\n        // Process IME events first:|
        s|^        events\.sort_by_key|        // events.sort_by_key|
    }
' "$EGUI_FILE"

rm -f "$EGUI_FILE.tmp"

echo "✓ Patch applied successfully"
echo ""
echo "Changes:"
echo "  - Disabled remove_ime_incompatible_events() call"
echo "  - Disabled IME event sorting"
echo ""
echo "Original backed up to: $EGUI_FILE.backup"
echo ""
echo "Run 'cargo clean' then 'cargo build' to apply"
