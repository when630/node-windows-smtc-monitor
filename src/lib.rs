#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod media_control;
mod monitor;
mod session_manager;
mod types;
mod utils;

pub use crate::media_control::{
  get_capabilities, get_current_session, get_session_by_id, get_sessions, try_change_playback_position,
  try_pause, try_play, try_skip_next, try_skip_previous,
};
pub use crate::monitor::SMTCMonitor;
pub use crate::types::{MediaInfo, MediaProps, PlaybackCapabilities, PlaybackInfo, TimelineProps};
