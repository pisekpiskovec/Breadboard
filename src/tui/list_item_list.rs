use appcui::{prelude::{ListScrollBars, OnKeyPressed, OnPaint, Surface, TextFormatBuilder}, system::Theme, ui::{ControlBase, Layout, listbox::Flags}};

#[CustomControl(overwrite = OnPaint+OnKeyPressed+OnMouseEvent+OnResize)]
pub struct ListItemList {
    items: Vec<String>,
    flags: Flags,
    top_view: usize,
    left_view: usize,
    max_chars: u32,
    comp: ListScrollBars,
    empty_message: String,
}
impl ListItemList {
    /// Creates a new list box with the specified layout
    ///
    /// # Example
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// let lbox = ListItemList::new(layout!("d:f"));
    /// ```
    pub fn new(layout: Layout) -> Self {
        Self::with_capacity(0, layout)
    }

    /// Creates a new list box with the specified layout and capacity
    ///
    /// # Example
    /// ```rust,no_run
    /// use appcui::prelude::*;
    ///
    /// // a listbox with a capacity of 100 items, with scrollbars
    /// let lbox = ListItemList::with_capacity(100, layout!("d:f"));
    /// ```   
    pub fn with_capacity(capacity: usize, layout: Layout) -> Self {
        let mut status_flags = StatusFlags::Enabled | StatusFlags::Visible | StatusFlags::AcceptInput;
        Self {
            base: ControlBase::with_status_flags(layout, status_flags),
            items: if capacity == 0 { Vec::new() } else { Vec::with_capacity(capacity) },
            top_view: 0,
            left_view: 0,
            max_chars: 0,
            flags: Flags::ScrollBars,
            empty_message: String::new(),
            comp: ListScrollBars::new(true, false),
        }
    }

    /// Adds a new item to the list by providing a string value
    pub fn add(&mut self, value: &str) {
        let value_str = value.to_string();
        if self.items.is_empty() {
            self.max_chars = value_str.len() as u32;
        } else {
            self.max_chars = self.max_chars.max(value_str.len() as u32);
        }
        self.items.push(value_str);

        let extra = 0;
        self.comp.resize(self.max_chars as u64 + extra, self.items.len() as u64, &self.base, self.size());
    }

    /// Clers all items from the list
    #[inline(always)]
    pub fn clear(&mut self) {
        self.items.clear();
        self.top_view = 0;
        self.max_chars = 0;
        self.comp.resize(0, 0, &self.base, self.size());
    }

    /// Returns the item from the listbox at the specified index
    #[inline(always)]
    pub fn item(&self, index: usize) -> Option<&String> {
        self.items.get(index)
    }

    /// Returns the total number of items fom the listbox
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// Sets the empty message that will be displayed when the listbox is empty
    pub fn set_empty_message(&mut self, message: &str) {
        self.empty_message.clear();
        self.empty_message.push_str(message);
    }

    fn update_scrollbars(&mut self) {
        self.comp.set_indexes(self.left_view as u64, self.top_view as u64);
    }
    fn update_left_position_for_items(&mut self) {
        let len = self.items.len();
        if len == 0 {
            return;
        }
        let last_index = (len - 1).min(self.top_view + self.size().height as usize);
        for i in self.items[self.top_view..=last_index].iter_mut() {
            i.update_left_pos(self.left_view as u32);
        }
    }
    fn update_position(&mut self, new_pos: usize, emit_event: bool) {
        let len = self.items.len();
        if len == 0 {
            return;
        }
        let new_pos = new_pos.min(len - 1);
        let h = self.size().height as usize;

        // check the top view
        if self.top_view + h >= len {
            self.top_view = len.saturating_sub(h);
        }
        if new_pos < self.top_view {
            self.top_view = new_pos;
        } else {
            let diff = new_pos - self.top_view;
            if (diff >= h) && (h > 0) {
                self.top_view = new_pos - h + 1;
            }
        }
        // update scrollbars
        self.update_scrollbars();
        self.update_left_position_for_items();
        let should_emit = (self.pos != new_pos) && emit_event;
        self.pos = new_pos;
        if should_emit {
            self.raise_event(ControlEvent {
                emitter: self.handle,
                receiver: self.event_processor,
                data: ControlEventData::ListBox(EventData {
                    event_type: ListBoxEventTypes::CurrentItemChanged,
                    index: new_pos,
                    checked: false, // not relevant for this event
                }),
            });
        }
    }

    fn mouse_to_pos(&self, x: i32, y: i32) -> Option<usize> {
        let size = self.size();
        if x < 0 || y < 0 || x >= size.width as i32 || y >= size.height as i32 {
            return None;
        }
        let idx = self.top_view + y as usize;
        if idx < self.items.len() {
            return Some(idx);
        }
        None
    }
    fn update_scroll_pos_from_scrollbars(&mut self) {
        self.top_view = (self.comp.vertical_index() as usize).min(self.items.len().saturating_sub(1));
        self.left_view = (self.comp.horizontal_index() as usize).min(self.max_chars as usize);
        self.update_left_position_for_items();
    }
    fn move_scroll_to(&mut self, new_poz: usize) {
        if new_poz == self.top_view {
            return;
        }
        let max_value = self.items.len().saturating_sub(self.size().height as usize);
        self.top_view = new_poz.min(max_value);
        self.update_scrollbars();
    }
    fn find_first_item(&mut self, pos: usize) {
        let mut i = if pos >= self.items.len() { 0 } else { pos };
        let mut count = self.items.len();
        while count > 0 {
            if self.items[i].filtered {
                self.update_position(i, true);
                return;
            }
            i = (i + 1) % self.items.len();
            count -= 1;
        }
    }
    fn search(&mut self) {
        let text_to_search = self.comp.search_text();
        if text_to_search.is_empty() {
            for item in self.items.iter_mut() {
                item.filtered = true;
            }
            self.comp.clear_match_count();
        } else {
            let mut count = 0usize;
            for item in self.items.iter_mut() {
                item.filtered = item.visible_text().index_ignoring_case(text_to_search).is_some();
                if item.filtered {
                    count += 1;
                }
            }
            self.comp.set_match_count(count);
            if count > 0 {
                self.find_first_item(self.pos);
            }
        }
    }
}

impl OnPaint for ListItemList {
    fn on_paint(&self, surface: &mut Surface, theme: &Theme) {
        let has_focus = self.has_focus();
        if has_focus {
            self.comp.paint(surface, theme, self);
            surface.reduce_clip_by(0, 0, 1, 1);
        }

        let attr = match () {
            _ if !self.is_active() => theme.text.inactive,
            _ if has_focus => theme.text.focused,
            _ => theme.text.normal,
        };

        let mut y = 0;
        let mut idx = self.top_view;
        let count = self.items.len();
        let h = self.size().height as i32;
        let w = self.size().width as i32;

        // empty message
        if (count == 0) && (!self.empty_message.is_empty()) {
            let empty_attr = match () {
                _ if !self.is_active() => theme.text.inactive,
                _ if has_focus => theme.text.highlighted,
                _ => theme.text.inactive,
            };
            let format = TextFormatBuilder::new()
                .position(w / 2, h / 2)
                .attribute(empty_attr)
                .align(appcui::prelude::TextAlignment::Center)
                .wrap_type(appcui::prelude::WrapType::WordWrap(w as u16))
                .build();
            surface.write_text(&self.empty_message, &format);
            return;
        }

        while (y < h) && (idx < count) {
            surface.write_string(
                0,
                y,
                &self.items[idx],
                attr,
                false,
            );
            y += 1;
            idx += 1;
        }
    }
}

impl OnKeyPressed for ListItemList {
    fn on_key_pressed(&mut self, key: Key, character: char) -> EventProcessStatus {
        if self.comp.process_key_pressed(key, character) {
            self.search();
            return EventProcessStatus::Processed;
        }
        match key.value() {
            key!("Up") => {
                self.update_position(self.pos.saturating_sub(1), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Down") => {
                self.update_position(self.pos.saturating_add(1), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Left") => {
                self.left_view = self.left_view.saturating_sub(1);
                self.update_left_position_for_items();
                self.update_scrollbars();
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Left") => {
                self.left_view = 0;
                self.update_left_position_for_items();
                self.update_scrollbars();
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Right") => {
                let d = if self.flags.contains(Flags::CheckBoxes) { 2 } else { 0 };
                let w = self.size().width.saturating_sub(d);
                self.left_view = (self.left_view + 1).min(self.max_chars.saturating_sub(w) as usize);
                self.update_left_position_for_items();
                self.update_scrollbars();
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Right") => {
                let d = if self.flags.contains(Flags::CheckBoxes) { 2 } else { 0 };
                let w = self.size().width.saturating_sub(d);
                self.left_view = self.max_chars.saturating_sub(w) as usize;
                self.update_left_position_for_items();
                self.update_scrollbars();
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Up") => {
                self.move_scroll_to(self.top_view.saturating_sub(1));
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Ctrl+Alt+Down") => {
                self.move_scroll_to(self.top_view.saturating_add(1));
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("Home") => {
                self.update_position(0, true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("End") => {
                self.update_position(self.items.len(), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("PageUp") => {
                self.update_position(self.pos.saturating_sub(self.size().height as usize), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            key!("PageDown") => {
                self.update_position(self.pos.saturating_add(self.size().height as usize), true);
                self.comp.exit_edit_mode();
                return EventProcessStatus::Processed;
            }
            _ => {}
        }
        if self.comp.should_repaint() {
            EventProcessStatus::Processed
        } else {
            EventProcessStatus::Ignored
        }
    }
}
