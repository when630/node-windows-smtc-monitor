use napi_derive::napi;
use std::fmt;

#[napi(object)]
#[derive(Clone)]
pub struct TimelineProps {
  pub position: f64,
  pub duration: f64,
}

#[napi(object)]
#[derive(Clone)]
pub struct PlaybackInfo {
  pub playback_status: u8,
  pub playback_type: u8,
}

#[napi(object)]
#[derive(Clone)]
pub struct PlaybackCapabilities {
  pub is_play_enabled: bool,
  pub is_pause_enabled: bool,
  pub is_stop_enabled: bool,
  pub is_next_enabled: bool,
  pub is_previous_enabled: bool,
  pub is_playback_position_enabled: bool,
  pub is_fast_forward_enabled: bool,
  pub is_rewind_enabled: bool,
  pub is_play_pause_toggle_enabled: bool,
  pub is_playback_rate_enabled: bool,
  pub is_shuffle_enabled: bool,
  pub is_repeat_enabled: bool,
}

#[napi(object)]
#[derive(Clone)]
pub struct MediaProps {
  pub title: String,
  pub artist: String,
  pub album_title: String,
  pub album_artist: String,
  pub genres: Vec<String>,
  pub album_track_count: u32,
  pub track_number: u32,
  #[napi(ts_type = "Buffer | undefined")]
  pub thumbnail: Option<napi::bindgen_prelude::Buffer>,
}

#[napi(object)]
#[derive(Clone)]
pub struct MediaInfo {
  pub source_app_id: String,
  pub media: MediaProps,
  pub playback: PlaybackInfo,
  pub timeline: TimelineProps,
  pub last_updated_time: f64,
}

impl fmt::Debug for MediaInfo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("MediaInfo")
      .field("source_app_id", &self.source_app_id)
      .field("media", &self.media)
      .field("playback", &self.playback)
      .field("timeline", &self.timeline)
      .field("last_updated_time", &self.last_updated_time)
      .finish()
  }
}

impl fmt::Debug for MediaProps {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("MediaProps")
      .field("title", &self.title)
      .field("artist", &self.artist)
      .field("album_title", &self.album_title)
      .field("album_artist", &self.album_artist)
      .field("genres", &self.genres)
      .field("album_track_count", &self.album_track_count)
      .field("track_number", &self.track_number)
      .field("thumbnail", &"[Buffer]")
      .finish()
  }
}

impl fmt::Debug for PlaybackInfo {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("PlaybackInfo")
      .field("playback_status", &self.playback_status)
      .field("playback_type", &self.playback_type)
      .finish()
  }
}

impl fmt::Debug for PlaybackCapabilities {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("PlaybackCapabilities")
      .field("is_play_enabled", &self.is_play_enabled)
      .field("is_pause_enabled", &self.is_pause_enabled)
      .field("is_stop_enabled", &self.is_stop_enabled)
      .field("is_next_enabled", &self.is_next_enabled)
      .field("is_previous_enabled", &self.is_previous_enabled)
      .field("is_playback_position_enabled", &self.is_playback_position_enabled)
      .field("is_fast_forward_enabled", &self.is_fast_forward_enabled)
      .field("is_rewind_enabled", &self.is_rewind_enabled)
      .field("is_play_pause_toggle_enabled", &self.is_play_pause_toggle_enabled)
      .field("is_playback_rate_enabled", &self.is_playback_rate_enabled)
      .field("is_shuffle_enabled", &self.is_shuffle_enabled)
      .field("is_repeat_enabled", &self.is_repeat_enabled)
      .finish()
  }
}

impl fmt::Debug for TimelineProps {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("TimelineProps")
      .field("position", &self.position)
      .field("duration", &self.duration)
      .finish()
  }
}
