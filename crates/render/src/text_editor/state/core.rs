use astra_core::{Duration, Instant};
use parley::editing::{Generation, PlainEditor, PlainEditorDriver};
use parley::{FontContext, LayoutContext, StyleProperty};
use peniko::Brush;

pub struct TextEditState {
    pub(super) editor: PlainEditor<Brush>,
    pub(super) cursor_visible: bool,
    pub(super) start_time: Option<Instant>,
    pub(super) blink_period: Duration,
    pub(super) is_dragging: bool,
    pub(super) cached_width: f32,
    pub(super) cached_height: f32,
}

impl TextEditState {
    pub fn new(text: &str, font_size: f32) -> Self {
        use parley::GenericFamily;

        let mut editor = PlainEditor::new(font_size);
        editor.set_text(text);
        editor.set_scale(1.0);

        let styles = editor.edit_styles();
        styles.insert(GenericFamily::SansSerif.into());
        styles.insert(StyleProperty::Brush(Brush::Solid(peniko::Color::BLACK)));

        Self {
            editor,
            cursor_visible: true,
            start_time: None,
            blink_period: Duration::ZERO,
            is_dragging: false,
            cached_width: 0.0,
            cached_height: 0.0,
        }
    }

    pub fn editor_mut(&mut self) -> &mut PlainEditor<Brush> {
        &mut self.editor
    }

    pub fn editor(&self) -> &PlainEditor<Brush> {
        &self.editor
    }

    pub fn driver<'a>(
        &'a mut self,
        font_cx: &'a mut FontContext,
        layout_cx: &'a mut LayoutContext<Brush>,
    ) -> PlainEditorDriver<'a, Brush> {
        self.editor.driver(font_cx, layout_cx)
    }

    pub fn text(&self) -> String {
        self.editor.text().to_string()
    }

    pub fn set_text(&mut self, text: &str) {
        self.editor.set_text(text);
    }

    pub fn set_brush(&mut self, brush: Brush) {
        let styles = self.editor.edit_styles();
        styles.insert(StyleProperty::Brush(brush));
    }

    pub fn set_font_size(&mut self, size: f32) {
        let styles = self.editor.edit_styles();
        styles.insert(StyleProperty::FontSize(size));
    }

    pub fn selection_range(&self) -> Option<std::ops::Range<usize>> {
        let range = self.editor.raw_selection().text_range();
        if range.start == range.end {
            None
        } else {
            Some(range)
        }
    }

    pub fn cursor_byte_offset(&self) -> usize {
        self.editor.raw_selection().text_range().start
    }

    pub fn set_width(&mut self, width: Option<f32>) {
        self.editor.set_width(width);
    }

    pub fn cursor_reset(&mut self) {
        self.start_time = Some(Instant::now());
        self.blink_period = Duration::from_millis(500);
        self.cursor_visible = true;
    }

    pub fn disable_blink(&mut self) {
        self.start_time = None;
    }

    pub fn next_blink_time(&self) -> Option<Instant> {
        self.start_time.map(|start_time| {
            let phase = Instant::now().duration_since(start_time);
            start_time
                + Duration::from_nanos(
                    ((phase.as_nanos() / self.blink_period.as_nanos() + 1)
                        * self.blink_period.as_nanos()) as u64,
                )
        })
    }

    pub fn cursor_blink(&mut self) {
        self.cursor_visible = self.start_time.is_some_and(|start_time| {
            let elapsed = Instant::now().duration_since(start_time);
            (elapsed.as_millis() / self.blink_period.as_millis()) % 2 == 0
        });
    }

    pub fn is_cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    pub fn generation(&self) -> Generation {
        self.editor.generation()
    }

    pub fn is_composing(&self) -> bool {
        self.editor.is_composing()
    }

    pub fn layout_size(&self) -> (f32, f32) {
        (self.cached_width, self.cached_height)
    }

    pub fn update_layout_cache(
        &mut self,
        font_cx: &mut FontContext,
        layout_cx: &mut LayoutContext<Brush>,
    ) {
        let layout = self.editor.layout(font_cx, layout_cx);
        self.cached_width = layout.width();
        self.cached_height = layout.height();
    }
}

impl Default for TextEditState {
    fn default() -> Self {
        Self::new("", 32.0)
    }
}
