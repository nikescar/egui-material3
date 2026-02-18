use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use egui::{Response, Sense, Window};
use egui_material3::spreadsheet::{
    viewer::{default_hotkeys, CellWriteContext, DecodeErrorBehavior, RowCodec, UiActionContext},
    DataTable, Renderer, Style, RowViewer,
};

/* ----------------------------------------- Columns -------------------------------------------- */

mod columns {

    // column indices
    // columns can easily be reordered simply by changing the values of these indices.
    pub const NAME: usize = 0;
    pub const AGE: usize = 1;
    pub const GENDER: usize = 2;
    pub const IS_STUDENT: usize = 3;
    pub const GRADE: usize = 4;
    pub const ROW_LOCKED: usize = 5;

    /// count of columns
    pub const COLUMN_COUNT: usize = 6;

    pub const COLUMN_NAMES: [&str; COLUMN_COUNT] = [
        "Name (Click to sort)",
        "Age",
        "Gender",
        "Is Student (Not sortable)",
        "Grade",
        "Row locked",
    ];
}
use columns::*;

/* ----------------------------------------- Data Scheme ---------------------------------------- */

struct Viewer {
    name_filter: String,
    row_protection: bool,
    hotkeys: Vec<(egui::KeyboardShortcut, egui_material3::spreadsheet::viewer::UiAction)>,
}

#[derive(Debug, Clone)]
struct Row {
    name: String,
    age: i32,
    gender: Option<Gender>,
    is_student: bool,
    grade: Grade,
    row_locked: bool,
}

impl Default for Row {
    fn default() -> Self {
        Row {
            name: "".to_string(),
            age: 0,
            gender: None,
            is_student: false,
            grade: Grade::F,
            row_locked: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Grade {
    A,
    B,
    C,
    D,
    E,
    F,
}

impl Display for Grade {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Grade::A => write!(f, "A"),
            Grade::B => write!(f, "B"),
            Grade::C => write!(f, "C"),
            Grade::D => write!(f, "D"),
            Grade::E => write!(f, "E"),
            Grade::F => write!(f, "F"),
        }
    }
}

impl TryFrom<i32> for Grade {
    type Error = ();

    fn try_from(input: i32) -> Result<Self, Self::Error> {
        let value = match input {
            0 => Grade::A,
            1 => Grade::B,
            2 => Grade::C,
            3 => Grade::D,
            4 => Grade::E,
            5 => Grade::F,
            _ => return Err(()),
        };
        Ok(value)
    }
}

impl FromStr for Grade {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let value = match input {
            "A" => Grade::A,
            "B" => Grade::B,
            "C" => Grade::C,
            "D" => Grade::D,
            "E" => Grade::E,
            "F" => Grade::F,
            _ => return Err(()),
        };

        Ok(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Gender {
    Male,
    Female,
}

impl Display for Gender {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "Male"),
            Gender::Female => write!(f, "Female"),
        }
    }
}

/* -------------------------------------------- Codec ------------------------------------------- */

struct Codec;

impl RowCodec<Row> for Codec {
    type DeserializeError = &'static str;

    fn encode_column(&mut self, src_row: &Row, column: usize, dst: &mut String) {
        match column {
            NAME => dst.push_str(&src_row.name),
            AGE => dst.push_str(&src_row.age.to_string()),
            GENDER => dst.push_str(
                &src_row
                    .gender
                    .map(|g| g.to_string())
                    .unwrap_or_default(),
            ),
            IS_STUDENT => dst.push_str(&src_row.is_student.to_string()),
            GRADE => dst.push_str(src_row.grade.to_string().as_str()),
            ROW_LOCKED => dst.push_str(&src_row.row_locked.to_string()),
            _ => unreachable!(),
        }
    }

    fn decode_column(
        &mut self,
        src_data: &str,
        column: usize,
        dst_row: &mut Row,
    ) -> Result<(), DecodeErrorBehavior> {
        match column {
            NAME => dst_row.name.replace_range(.., src_data),
            AGE => dst_row.age = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?,
            GENDER => {
                dst_row.gender = match src_data {
                    "Male" => Some(Gender::Male),
                    "Female" => Some(Gender::Female),
                    _ => None,
                }
            }
            IS_STUDENT => {
                dst_row.is_student = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?
            }
            GRADE => {
                dst_row.grade = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?;
            }
            ROW_LOCKED => {
                dst_row.row_locked = src_data.parse().map_err(|_| DecodeErrorBehavior::SkipRow)?
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn create_empty_decoded_row(&mut self) -> Row {
        Row::default()
    }
}

/* ------------------------------------ Viewer Implementation ----------------------------------- */

impl RowViewer<Row> for Viewer {
    fn on_highlight_cell(&mut self, row: &Row, column: usize) {
        println!("cell highlighted: row: {:?}, column: {}", row, column);
    }

    fn try_create_codec(&mut self, _: bool) -> Option<impl RowCodec<Row>> {
        Some(Codec)
    }

    fn num_columns(&mut self) -> usize {
        COLUMN_COUNT
    }

    fn column_name(&mut self, column: usize) -> Cow<'static, str> {
        COLUMN_NAMES[column].into()
    }

    fn is_sortable_column(&mut self, column: usize) -> bool {
        [true, true, true, false, true, true][column]
    }

    fn is_editable_cell(&mut self, column: usize, _row: usize, row_value: &Row) -> bool {
        let row_locked = row_value.row_locked;
        match column {
            ROW_LOCKED => true,
            _ => !row_locked,
        }
    }

    fn compare_cell(&self, row_l: &Row, row_r: &Row, column: usize) -> std::cmp::Ordering {
        match column {
            NAME => row_l.name.cmp(&row_r.name),
            AGE => row_l.age.cmp(&row_r.age),
            GENDER => row_l.gender.cmp(&row_r.gender),
            IS_STUDENT => unreachable!(),
            GRADE => row_l.grade.cmp(&row_r.grade),
            ROW_LOCKED => row_l.row_locked.cmp(&row_r.row_locked),
            _ => unreachable!(),
        }
    }

    fn new_empty_row(&mut self) -> Row {
        Row::default()
    }

    fn set_cell_value(&mut self, src: &Row, dst: &mut Row, column: usize) {
        match column {
            NAME => dst.name.clone_from(&src.name),
            AGE => dst.age = src.age,
            GENDER => dst.gender = src.gender,
            IS_STUDENT => dst.is_student = src.is_student,
            GRADE => dst.grade = src.grade,
            ROW_LOCKED => dst.row_locked = src.row_locked,
            _ => unreachable!(),
        }
    }

    fn confirm_cell_write_by_ui(
        &mut self,
        current: &Row,
        _next: &Row,
        _column: usize,
        _context: CellWriteContext,
    ) -> bool {
        if !self.row_protection {
            return true;
        }
        !current.is_student
    }

    fn confirm_row_deletion_by_ui(&mut self, row: &Row) -> bool {
        if !self.row_protection {
            return true;
        }
        !row.is_student
    }

    fn show_cell_view(&mut self, ui: &mut egui::Ui, row: &Row, column: usize) {
        let _ = match column {
            NAME => ui.label(&row.name),
            AGE => ui.label(row.age.to_string()),
            GENDER => ui.label(
                row.gender
                    .map(|g| g.to_string())
                    .unwrap_or_else(|| "Unspecified".to_string()),
            ),
            IS_STUDENT => ui.checkbox(&mut { row.is_student }, ""),
            GRADE => ui.label(row.grade.to_string()),
            ROW_LOCKED => ui.checkbox(&mut { row.row_locked }, ""),
            _ => unreachable!(),
        };
    }

    fn on_cell_view_response(
        &mut self,
        _row: &Row,
        _column: usize,
        resp: &egui::Response,
    ) -> Option<Box<Row>> {
        resp.dnd_release_payload::<String>().map(|x| {
            Box::new(Row {
                name: (*x).clone(),
                age: 9999,
                gender: Some(Gender::Female),
                is_student: false,
                grade: Grade::A,
                row_locked: false,
            })
        })
    }

    fn show_cell_editor(
        &mut self,
        ui: &mut egui::Ui,
        row: &mut Row,
        column: usize,
    ) -> Option<Response> {
        match column {
            NAME => egui::TextEdit::multiline(&mut row.name)
                .desired_rows(1)
                .code_editor()
                .show(ui)
                .response,
            AGE => ui.add(egui::DragValue::new(&mut row.age).speed(1.0)),
            GENDER => {
                let gender = &mut row.gender;
                egui::ComboBox::new(ui.id().with("gender"), "".to_string())
                    .selected_text(
                        gender
                            .map(|g| g.to_string())
                            .unwrap_or_else(|| "Unspecified".to_string()),
                    )
                    .show_ui(ui, |ui| {
                        if ui
                            .add(egui::Button::selectable(
                                matches!(gender, Some(g) if *g == Gender::Male),
                                "Male",
                            ))
                            .clicked()
                        {
                            *gender = Some(Gender::Male);
                        }
                        if ui
                            .add(egui::Button::selectable(
                                matches!(gender, Some(g) if *g == Gender::Female),
                                "Female",
                            ))
                            .clicked()
                        {
                            *gender = Some(Gender::Female);
                        }
                    })
                    .response
            }
            IS_STUDENT => ui.checkbox(&mut row.is_student, ""),
            GRADE => {
                let grade = &mut row.grade;
                ui.horizontal_wrapped(|ui| {
                    ui.radio_value(grade, Grade::A, "A")
                        | ui.radio_value(grade, Grade::B, "B")
                        | ui.radio_value(grade, Grade::C, "C")
                        | ui.radio_value(grade, Grade::D, "D")
                        | ui.radio_value(grade, Grade::E, "E")
                        | ui.radio_value(grade, Grade::F, "F")
                })
                .inner
            }
            ROW_LOCKED => ui.checkbox(&mut row.row_locked, ""),
            _ => unreachable!(),
        }
        .into()
    }

    fn row_filter_hash(&mut self) -> &impl std::hash::Hash {
        &self.name_filter
    }

    fn filter_row(&mut self, row: &Row) -> bool {
        row.name.contains(&self.name_filter)
    }

    fn hotkeys(
        &mut self,
        context: &UiActionContext,
    ) -> Vec<(egui::KeyboardShortcut, egui_material3::spreadsheet::viewer::UiAction)> {
        let hotkeys = default_hotkeys(context);
        self.hotkeys.clone_from(&hotkeys);
        hotkeys
    }

    fn on_highlight_change(&mut self, highlighted: &[&Row], unhighlighted: &[&Row]) {
        println!("highlight {:?}", highlighted);
        println!("unhighlight {:?}", unhighlighted);
    }

    fn on_row_updated(&mut self, row_index: usize, new_row: &Row, old_row: &Row) {
        println!(
            "row updated. row_id: {}, new_row: {:?}, old_row: {:?}",
            row_index, new_row, old_row
        );
    }

    fn on_row_inserted(&mut self, row_index: usize, row: &Row) {
        println!("row inserted. row_id: {}, values: {:?}", row_index, row);
    }

    fn on_row_removed(&mut self, row_index: usize, row: &Row) {
        println!("row removed. row_id: {}, values: {:?}", row_index, row);
    }
}

/* ---------------------------------------- Sample Data ----------------------------------------- */

fn sample_rows() -> Vec<Row> {
    vec![
        Row { name: "Alice Smith".into(), age: 22, gender: Some(Gender::Female), is_student: true, grade: Grade::A, row_locked: false },
        Row { name: "Bob Johnson".into(), age: 19, gender: Some(Gender::Male), is_student: true, grade: Grade::B, row_locked: false },
        Row { name: "Carol White".into(), age: 25, gender: Some(Gender::Female), is_student: false, grade: Grade::C, row_locked: false },
        Row { name: "David Brown".into(), age: 21, gender: Some(Gender::Male), is_student: true, grade: Grade::A, row_locked: true },
        Row { name: "Eva Green".into(), age: 18, gender: Some(Gender::Female), is_student: true, grade: Grade::F, row_locked: false },
        Row { name: "Frank Lee".into(), age: 30, gender: Some(Gender::Male), is_student: false, grade: Grade::B, row_locked: false },
        Row { name: "Grace Kim".into(), age: 23, gender: Some(Gender::Female), is_student: true, grade: Grade::D, row_locked: false },
        Row { name: "Henry Park".into(), age: 20, gender: Some(Gender::Male), is_student: true, grade: Grade::E, row_locked: false },
        Row { name: "Iris Chen".into(), age: 27, gender: None, is_student: false, grade: Grade::A, row_locked: false },
        Row { name: "James Wilson".into(), age: 16, gender: Some(Gender::Male), is_student: true, grade: Grade::C, row_locked: false },
        Row { name: "Kate Davis".into(), age: 24, gender: Some(Gender::Female), is_student: false, grade: Grade::B, row_locked: true },
        Row { name: "Liam Taylor".into(), age: 29, gender: Some(Gender::Male), is_student: false, grade: Grade::F, row_locked: false },
        Row { name: "Mia Anderson".into(), age: 17, gender: Some(Gender::Female), is_student: true, grade: Grade::A, row_locked: false },
        Row { name: "Noah Martin".into(), age: 31, gender: Some(Gender::Male), is_student: false, grade: Grade::D, row_locked: false },
        Row { name: "Olivia Jackson".into(), age: 20, gender: Some(Gender::Female), is_student: true, grade: Grade::B, row_locked: false },
    ]
}

/* ------------------------------------------ Window -------------------------------------------- */

pub struct SpreadsheetWindow {
    pub open: bool,
    table: DataTable<Row>,
    viewer: Viewer,
    style_override: Style,
}

impl Default for SpreadsheetWindow {
    fn default() -> Self {
        Self {
            open: false,
            table: sample_rows().into_iter().collect(),
            viewer: Viewer {
                name_filter: String::new(),
                hotkeys: Vec::new(),
                row_protection: false,
            },
            style_override: Default::default(),
        }
    }
}

impl SpreadsheetWindow {
    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = self.open;

        Window::new("Spreadsheet Stories")
            .open(&mut open)
            .default_size([900.0, 600.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Name Filter:");
                    ui.text_edit_singleline(&mut self.viewer.name_filter);

                    ui.separator();

                    ui.checkbox(&mut self.viewer.row_protection, "Row Protection")
                        .on_hover_text(
                            "If checked, rows marked as 'Is Student' \
                             cannot be deleted or overwritten.",
                        );

                    ui.separator();

                    ui.checkbox(
                        &mut self.style_override.single_click_edit_mode,
                        "Single Click Edit",
                    );

                    ui.add(
                        egui::Button::new("Drag me → drop on any cell").sense(Sense::drag()),
                    )
                    .on_hover_text("Dropping this replaces the cell with a preset value.")
                    .dnd_set_drag_payload(String::from("Hallo~"));
                });

                ui.separator();

                let mut has_modifications = self.table.has_user_modification();
                ui.horizontal(|ui| {
                    ui.add_enabled(
                        false,
                        egui::Checkbox::new(&mut has_modifications, "Has modifications"),
                    );
                    ui.add_enabled_ui(has_modifications, |ui| {
                        if ui.button("Clear").clicked() {
                            self.table.clear_user_modification_flag();
                        }
                    });
                });

                ui.separator();

                ui.add(
                    Renderer::new(&mut self.table, &mut self.viewer)
                        .with_style(self.style_override),
                );
            });

        self.open = open;
    }
}
