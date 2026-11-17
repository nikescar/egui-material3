# Drawer Height Expansion Fix

## Problem
When list items with drawers were expanded, the drawer content would be clipped because the total list height was calculated before measuring drawer content. This caused content below expanded drawers to be hidden.

## Root Cause
In `src/list/rendering.rs`:
- Total height was calculated at lines 23-92 **before** drawer state was loaded
- Drawer state was loaded at lines 114-117 **after** height allocation
- Drawer content was rendered at lines 422-468, but height was added to `current_y` **after** the list container was already allocated with fixed size
- Result: List container had fixed height that didn't account for expanded drawer content

## Solution

### 1. Load Drawer State Earlier
Moved drawer state loading from line 114 to before height calculation (line 46):
```rust
// Load persistent drawer state BEFORE calculating total height
let state_id = self.id.unwrap_or_else(|| ui.id().with("material_list"));
let state = ui
    .data_mut(|d| d.get_persisted::<MaterialListState>(state_id))
    .unwrap_or_default();
```

### 2. Created Helper Function
Added `measure_drawer_height()` function at line 9:
```rust
fn measure_drawer_height(
    drawer_fn: &dyn Fn(&mut Ui),
    ui: &mut Ui,
    width: f32,
) -> f32 {
    // Create temporary off-screen UI to measure content
    // Returns measured height without visual rendering
}
```

### 3. Include Drawer Heights in Total Calculation
Modified height calculation loop (line 89-97) to include drawer content:
```rust
// Add drawer height if drawer is open
if self.allow_drawer && item.drawer.is_some() && state.drawer_open_items.contains(&index) {
    let drawer_fn = item.drawer.as_ref().unwrap();
    let drawer_width = max_content_width.min(ui.available_width());
    let drawer_height = measure_drawer_height(&**drawer_fn, ui, drawer_width);
    total_height += drawer_height;
}
```

## Files Changed

### `src/lib.rs`
- **Line 314**: Added `MaterialListState` to public exports

### `src/list/rendering.rs`
- **Lines 9-34**: Added `measure_drawer_height()` helper function
- **Lines 46-50**: Moved drawer state loading before height calculation
- **Lines 57-58**: Added explicit type annotations `f32` to fix ambiguous float types
- **Lines 89-97**: Added drawer height calculation in total height loop
- **Lines 154-157**: Re-load drawer state as mutable for rendering updates

### `tests/list_drawer_height_test.rs` (NEW)
Created comprehensive integration tests:
- `test_drawer_closed_height`: Verify closed drawer has correct height
- `test_drawer_open_expands_height`: Verify open drawer expands list height
- `test_multiple_drawers_open`: Verify multiple drawers contribute to height
- `test_drawer_content_not_clipped`: Verify large drawer content is not clipped

## Test Results

### Before Fix
```
test test_drawer_closed_height ... PASSED
test test_drawer_open_expands_height ... FAILED (height 56, expected > 96)
test test_multiple_drawers_open ... FAILED (height 113, expected > 192)
test test_drawer_content_not_clipped ... FAILED (height 56, expected >= 272)
```

### After Fix
```
test test_drawer_closed_height ... ok
test test_drawer_open_expands_height ... ok
test test_multiple_drawers_open ... ok
test test_drawer_content_not_clipped ... ok

test result: ok. 4 passed; 0 failed
```

### Existing Tests
All existing tests continue to pass:
```
test test_list_item_can_have_drawer ... ok
test test_drawer_default_is_none ... ok
test test_drawer_if_conditional ... ok
```

## Behavior

### Before
- Drawer opens but list height remains fixed
- Content below drawer gets clipped and hidden
- User cannot see expanded drawer content

### After
- Drawer opens and list height expands dynamically
- All drawer content is visible
- List shrinks when drawer closes

## Technical Details

### Drawer Height Measurement
The `measure_drawer_height()` function:
1. Creates a temporary off-screen rect at `(-10000, -10000)`
2. Creates a child UI with the drawer's target width
3. Runs the drawer rendering function
4. Measures the resulting height
5. Returns height without visually rendering

This approach ensures accurate height measurement without:
- Visual flickering
- Multiple rendering passes
- Layout loops

### Performance Impact
Minimal performance impact:
- Drawer content is measured once during height calculation
- Measurement uses off-screen rendering (no visual cost)
- Only open drawers are measured (closed drawers skip measurement)
- No caching needed as state is stable during a single frame

## Follow-up Considerations

### Potential Improvements
1. **Cache drawer heights**: If drawer content is static, cache measured heights
2. **Animate transitions**: Add smooth height transitions when toggling drawers
3. **Lazy measurement**: Defer measurement until drawer is about to open

### Known Limitations
- Drawer content with dynamic heights requires re-measurement each frame
- Very large drawer content (1000+ items) may have measurable performance impact

## Validation

Build status: ✅ Success
```bash
cargo check
# Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.15s
```

Test status: ✅ All Pass
```bash
cargo test --test list_drawer_height_test
# test result: ok. 4 passed; 0 failed

cargo test --test list_drawer_test
# test result: ok. 3 passed; 0 failed
```
