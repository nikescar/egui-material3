#!/bin/bash

# Verification script for multiline list item fixes

echo "🔍 Verifying Multiline List Item Fixes"
echo "======================================"
echo ""

echo "📋 Running tests..."
cargo test list_multiline --quiet
if [ $? -eq 0 ]; then
    echo "✅ All multiline layout tests passed"
else
    echo "❌ Tests failed"
    exit 1
fi

echo ""
echo "📋 Running full test suite..."
cargo test --lib --quiet
if [ $? -eq 0 ]; then
    echo "✅ All library tests passed"
else
    echo "❌ Library tests failed"
    exit 1
fi

echo ""
echo "🔨 Building stories example..."
cargo build --example stories --quiet
if [ $? -eq 0 ]; then
    echo "✅ Stories example builds successfully"
else
    echo "❌ Build failed"
    exit 1
fi

echo ""
echo "✨ All checks passed!"
echo ""
echo "📖 Manual Verification Steps:"
echo "   1. Run: cargo run --example stories"
echo "   2. Click 'List Stories' button"
echo "   3. Scroll to 'Editable List with Action Buttons'"
echo "   4. Verify:"
echo "      - First item has multiline description"
echo "      - Edit/Delete buttons stay aligned with title"
echo "      - Equal top/bottom padding around text"
echo ""
echo "   5. Scroll to 'Editable Drawer List with 5-Line Items'"
echo "   6. Verify:"
echo "      - 'Expand ▶' text visible at top right"
echo "      - Edit/Delete buttons centered below"
echo "      - No overlap between text and buttons"
echo ""
echo "See MULTILINE_LIST_FIX.md for detailed verification checklist"
