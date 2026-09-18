use napi::{bindgen_prelude::*, Result};
use windows::core;
use windows::Foundation::IAsyncOperation;
use windows::Media::Control::{
  GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
};

use crate::types::{MediaInfo, PlaybackCapabilities};
use crate::utils;

#[napi]
pub fn get_current_session() -> Result<Option<MediaInfo>> {
  let manager = create_manager()?;

  manager
    .GetCurrentSession()
    .ok()
    .map_or(Ok(None), |session| {
      utils::get_media_info_for_session(&session)
    })
}

#[napi]
pub fn get_sessions() -> Result<Vec<MediaInfo>> {
  let manager = create_manager()?;
  let sessions = match manager.GetSessions() {
    Ok(s) => s,
    Err(_) => return Ok(Vec::new()),
  };

  let size = match sessions.Size() {
    Ok(s) => s,
    Err(_) => return Ok(Vec::new()),
  };

  let mut result = Vec::new();
  for i in 0..size {
    let session = match sessions.GetAt(i) {
      Ok(s) => s,
      Err(_) => continue,
    };

    if let Ok(Some(info)) = utils::get_media_info_for_session(&session) {
      result.push(info);
    }
  }

  Ok(result)
}

#[napi]
pub fn get_session_by_id(source_app_id: String) -> Result<Option<MediaInfo>> {
  match find_session(&source_app_id)? {
    Some(session) => utils::get_media_info_for_session(&session),
    None => Ok(None),
  }
}

// --- Transport controls ---------------------------------------------------
//
// Every command below targets one session by its AUMID instead of relying on
// the media keys, which always go to whichever session Windows picked. A
// `false` return means either "no such session" or "the session refused the
// request". `Try*` only reports that the request was *accepted* — the session
// state catches up later, so callers should not read it back immediately.

#[napi]
pub fn try_play(source_app_id: String) -> Result<bool> {
  run_transport(&source_app_id, |session| session.TryPlayAsync())
}

#[napi]
pub fn try_pause(source_app_id: String) -> Result<bool> {
  run_transport(&source_app_id, |session| session.TryPauseAsync())
}

#[napi]
pub fn try_skip_next(source_app_id: String) -> Result<bool> {
  run_transport(&source_app_id, |session| session.TrySkipNextAsync())
}

#[napi]
pub fn try_skip_previous(source_app_id: String) -> Result<bool> {
  run_transport(&source_app_id, |session| session.TrySkipPreviousAsync())
}

/// Seek to an absolute position, in seconds. WinRT takes 100-nanosecond ticks.
#[napi]
pub fn try_change_playback_position(source_app_id: String, position_seconds: f64) -> Result<bool> {
  if !position_seconds.is_finite() || position_seconds < 0.0 {
    return Err(Error::new(
      Status::InvalidArg,
      "positionSeconds must be a finite, non-negative number".to_owned(),
    ));
  }

  let ticks = (position_seconds * 10_000_000.0).round() as i64;
  run_transport(&source_app_id, move |session| {
    session.TryChangePlaybackPositionAsync(ticks)
  })
}

/// What the session says it accepts. Seeking in particular is not universal —
/// ask before offering the control.
#[napi]
pub fn get_capabilities(source_app_id: String) -> Result<Option<PlaybackCapabilities>> {
  match find_session(&source_app_id)? {
    Some(session) => utils::get_capabilities_for_session(&session),
    None => Ok(None),
  }
}

fn run_transport<F>(source_app_id: &str, op: F) -> Result<bool>
where
  F: FnOnce(&GlobalSystemMediaTransportControlsSession) -> core::Result<IAsyncOperation<bool>>,
{
  let session = match find_session(source_app_id)? {
    Some(s) => s,
    None => return Ok(false),
  };

  let operation = utils::win_to_napi_err(op(&session))?;
  utils::win_to_napi_err(operation.get())
}

fn find_session(
  source_app_id: &str,
) -> Result<Option<GlobalSystemMediaTransportControlsSession>> {
  let manager = create_manager()?;
  let sessions = match manager.GetSessions() {
    Ok(s) => s,
    Err(_) => return Ok(None),
  };

  let size = match sessions.Size() {
    Ok(s) => s,
    Err(_) => return Ok(None),
  };

  for i in 0..size {
    let session = match sessions.GetAt(i) {
      Ok(s) => s,
      Err(_) => continue,
    };

    let id = match session.SourceAppUserModelId() {
      Ok(id) => id,
      Err(_) => continue,
    };

    if id.to_string() == source_app_id {
      return Ok(Some(session));
    }
  }

  Ok(None)
}

pub fn create_manager() -> Result<GlobalSystemMediaTransportControlsSessionManager> {
  let operation = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

  operation
    .get()
    .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
}
