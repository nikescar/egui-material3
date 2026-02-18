[![Latest version](https://img.shields.io/crates/v/egui-data-table.svg)](https://crates.io/crates/egui-data-table)
[![Documentation](https://docs.rs/egui-data-table/badge.svg)](https://docs.rs/egui-data-table)

---------------------------------------------------

Heres current implementation:

src/spreadsheet/mod.rs — Core data container
Defines DataTable<R>, the central generic container that wraps a Vec<R> of rows. Key responsibilities:

Owns the rows and the UiState (stored as Option<Box<UiState<R>>>) which is borrowed during rendering and returned on Drop
Tracks two separate dirty flags: cc_dirty (UI cache needs rebuild) and dirty_flag (user made edits via UI)
Re-exports Renderer, Style, RowViewer, UiAction as the public API of the spreadsheet module


src/spreadsheet/viewer.rs — Trait definitions & hotkeys
The "contract" layer. Contains:

RowViewer<R> — the main trait users implement to plug in their data. Covers column names/count, cell rendering (show_cell_view), cell editing (show_cell_editor), sorting, filtering, copy/paste codec, row lifecycle hooks (on_row_updated, on_row_inserted, etc.)
RowCodec<R> — optional trait for TSV clipboard encode/decode
UiAction enum — semantic user actions (MoveSelection, CopySelection, Undo, etc.)
default_hotkeys() — the default keyboard shortcut map, split by editing vs. selection mode


src/spreadsheet/draw/tsv.rs — TSV parsing/writing
A small, self-contained TSV codec:

ParsedTsv — parses a TSV string into a compact indexed structure (byte spans + row offsets), handling \t/\n/\r/\\ escape sequences
Writer helpers: write_tab, write_newline, write_content (escapes special chars on output)
Used by state.rs for system clipboard copy/paste interop


src/spreadsheet/draw/state.rs — UI state machine
The largest and most complex file. UiState<R> manages all mutable spreadsheet UI state:

Cache: cc_rows (filtered+sorted row indices), cc_row_heights, cc_row_id_to_vis (reverse lookup)
Cursor: CursorState enum — either Select(Vec<VisSelection>) or Edit { row, edition, ... }
Undo/redo: a VecDeque<UndoArg<R>> with a cursor pointer; every undoable Command stores its inverse restore commands
Clipboard: internal Clipboard<R> slab + paste descriptor list; also bridges to/from system TSV via tsv module
Commands: Command<R> enum covers everything — sort, hide/show/reorder columns, set cell values, insert/delete rows, edit start/commit/cancel, selection updates
push_new_command is the central dispatcher: validates, applies, records undo, manages the queue


src/spreadsheet/draw.rs — egui rendering layer
The Renderer<'a, R, V> struct that implements egui::Widget:

Header row: renders column names with sort indicators, supports drag-to-reorder columns, click-to-sort, context menu (hide/show columns, clear sort)
Body rows: for each visible row — renders row number header, then each cell via viewer.show_cell_view(); handles selection highlight, focus stroke, drag-selection via mouse
Edit overlay: when a cell is in edit mode, spawns an egui::Window overlay positioned over the cell, calling viewer.show_cell_editor()
Input handling: collects hotkeys/copy/paste events, converts them to UiActions, then to Commands via UiState::try_apply_ui_action
Translator: Translator trait + EnglishTranslator for localizable context menu strings


examples/stories/spreadsheet_window.rs — Demo / integration example
A full working example showing how to use the library:

Row struct with 6 columns (name, age, gender, is_student, grade, row_locked)
Viewer implements RowViewer<Row> — per-column sort, edit guards (row locking, student protection), cell view/editor widgets, name filter
Codec implements RowCodec<Row> for TSV clipboard support
SpreadsheetWindow wraps everything into an egui window with a filter text field, row-protection toggle, single-click-edit toggle, and a draggable button for testing DnD

---------------------------------------------------

