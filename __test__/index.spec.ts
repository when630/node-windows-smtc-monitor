import test from "ava"
import {
  getCapabilities,
  getCurrentSession,
  getSessionById,
  getSessions,
  tryChangePlaybackPosition,
  tryPause,
  tryPlay,
  trySkipNext,
  trySkipPrevious,
} from "../binding"

test("getCurrentSession() should return MediaInfo | null", (t) => {
  const session = getCurrentSession()
  t.true(session === null || "sourceAppId" in session)
})

test("getSessions() should return MediaInfo[]", (t) => {
  const sessions = getSessions()
  t.true(Array.isArray(sessions))
})

test("getSessionById() should return null", (t) => {
  t.is(getSessionById("nonexistent"), null)
})

// CI runners have no media session, so the transport commands can only be
// checked for the "no such session" contract: false, never a throw.

test("transport commands return false for an unknown session", (t) => {
  t.false(tryPlay("nonexistent"))
  t.false(tryPause("nonexistent"))
  t.false(trySkipNext("nonexistent"))
  t.false(trySkipPrevious("nonexistent"))
  t.false(tryChangePlaybackPosition("nonexistent", 30))
})

test("tryChangePlaybackPosition() accepts zero", (t) => {
  t.false(tryChangePlaybackPosition("nonexistent", 0))
})

test("tryChangePlaybackPosition() rejects a negative or non-finite position", (t) => {
  t.throws(() => tryChangePlaybackPosition("nonexistent", -1))
  t.throws(() => tryChangePlaybackPosition("nonexistent", Number.NaN))
  t.throws(() => tryChangePlaybackPosition("nonexistent", Number.POSITIVE_INFINITY))
})

test("getCapabilities() should return null for an unknown session", (t) => {
  t.is(getCapabilities("nonexistent"), null)
})
