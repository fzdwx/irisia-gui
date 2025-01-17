use std::sync::Arc;

use tokio::sync::Mutex;

use crate::event::{
    standard::{Blured, Focused},
    EventDispatcher,
};

use self::protected::Protected;

mod protected {
    use super::*;

    pub struct Protected(EventDispatcher);

    impl Protected {
        pub fn new(ed: EventDispatcher) -> Self {
            ed.emit_sys(Focused);
            Protected(ed)
        }

        pub fn get(&self) -> &EventDispatcher {
            &self.0
        }
    }

    impl Drop for Protected {
        fn drop(&mut self) {
            self.0.emit_sys(Blured);
        }
    }
}

pub(crate) type SharedFocusing = Arc<Mutex<Focusing>>;

pub(crate) struct Focusing {
    current_frame: CurrentFrame,
    next_frame: NextFrame,
}

enum NextFrame {
    Keep,
    ChangeTo(EventDispatcher),
    Clear,
}

enum CurrentFrame {
    NotConfirmed(Protected),
    NotConfirmedUnprotected(EventDispatcher),
    Confirmed { protected: Protected },
    None,
}

impl Focusing {
    pub fn new() -> Self {
        Focusing {
            current_frame: CurrentFrame::None,
            next_frame: NextFrame::Keep,
        }
    }

    pub fn focus_on(&mut self, ed: EventDispatcher) {
        self.next_frame = NextFrame::ChangeTo(ed);
    }

    pub fn blur_checked(&mut self, check: &EventDispatcher) {
        if let CurrentFrame::Confirmed { protected, .. } = &self.current_frame {
            if protected.get().is_same(check) {
                self.blur();
            }
        }
    }

    pub fn blur(&mut self) {
        if matches!(self.next_frame, NextFrame::Keep) {
            self.next_frame = NextFrame::Clear
        }
    }

    pub(super) fn to_not_confirmed(&mut self) {
        match std::mem::replace(&mut self.next_frame, NextFrame::Keep) {
            NextFrame::Keep => take_mut::take(&mut self.current_frame, |mut this| {
                if let CurrentFrame::Confirmed { protected: ev, .. } = this {
                    this = CurrentFrame::NotConfirmed(ev);
                }
                this
            }),
            NextFrame::ChangeTo(next) => {
                take_mut::take(&mut self.current_frame, |this| match this {
                    CurrentFrame::Confirmed { protected, .. } if protected.get().is_same(&next) => {
                        CurrentFrame::NotConfirmed(protected)
                    }
                    _ => CurrentFrame::NotConfirmedUnprotected(next),
                })
            }
            NextFrame::Clear => self.current_frame = CurrentFrame::None,
        }
    }

    pub(super) fn drop_not_confirmed(&mut self) {
        if let CurrentFrame::NotConfirmed(_) | CurrentFrame::NotConfirmedUnprotected(_) =
            &self.current_frame
        {
            self.current_frame = CurrentFrame::None;
        }
    }

    pub(super) fn try_confirm(&mut self, other: &EventDispatcher) {
        take_mut::take(&mut self.current_frame, |this| match this {
            CurrentFrame::NotConfirmed(protected) if protected.get().is_same(other) => {
                CurrentFrame::Confirmed { protected }
            }
            CurrentFrame::NotConfirmedUnprotected(ed) if ed.is_same(other) => {
                CurrentFrame::Confirmed {
                    protected: Protected::new(ed),
                }
            }
            _ => this,
        });
    }
}
