// SPDX-License-Identifier: MIT

//! Pacing for the preview beside the active column.
//!
//! Following the focused row means tearing down a column and building another
//! one around a fresh directory load. A held navigation key repeats faster than
//! that, so the preview is paced: the first move applies at once, a held key
//! refreshes it at a steady rate, and the last row always gets a preview.

use crate::ui::browser::ViewState;
use crate::ui::browser::collection::cancel_source;
use gtk::glib;
use std::rc::Rc;
use std::time::{Duration, Instant};

const REFRESH_INTERVAL: Duration = Duration::from_millis(220);

impl ViewState {
    pub(in crate::ui) fn schedule_child_preview(self: &Rc<Self>) {
        if !self.yazi_columns.get() || self.syncing_child_preview.get() {
            return;
        }
        let waited = self
            .last_child_preview
            .get()
            .map_or(REFRESH_INTERVAL, |last| last.elapsed());
        if let Some(remaining) = REFRESH_INTERVAL.checked_sub(waited) {
            if self.pending_child_preview.borrow().is_none() {
                let weak = Rc::downgrade(self);
                *self.pending_child_preview.borrow_mut() =
                    Some(glib::timeout_add_local_once(remaining, move || {
                        if let Some(state) = weak.upgrade() {
                            state.pending_child_preview.borrow_mut().take();
                            state.apply_child_preview();
                        }
                    }));
            }
            return;
        }
        cancel_source(&self.pending_child_preview);
        self.apply_child_preview();
    }

    pub(in crate::ui) fn cancel_child_preview(&self) {
        cancel_source(&self.pending_child_preview);
    }

    fn apply_child_preview(self: &Rc<Self>) {
        self.last_child_preview.set(Some(Instant::now()));
        self.syncing_child_preview.set(true);
        let active = self.browser.active_depth();
        self.browser.sync_child_preview();
        // A freshly built column is focusable, so GTK can move focus into the
        // preview; the user is still browsing the column beside it.
        if self.browser.active_depth() != active
            && let Some(depth) = active
        {
            self.browser.set_active_column(depth);
            self.browser.focus_active();
        }
        self.syncing_child_preview.set(false);
    }
}
