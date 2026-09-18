import { EventEmitter } from "events"
import { SMTCMonitor as SMTC } from "./binding"
import type {
  MediaInfo,
  MediaProps,
  PlaybackCapabilities,
  PlaybackInfo,
  TimelineProps,
  MediaPropsCallbackData,
  PlaybackInfoCallbackData,
  TimelinePropsCallbackData,
} from "./binding"

export enum PlaybackStatus {
  CLOSED = 0,
  OPENED = 1,
  CHANGING = 2,
  STOPPED = 3,
  PLAYING = 4,
  PAUSED = 5,
}

declare class SMTCMonitor extends EventEmitter {
  constructor()

  private smtc: SMTC
  private _mediaSessions: Map<string, MediaInfo>

  private _initialize(): void
  private _preloadSessions(): void
  private _bindEvents(): void

  private _onMediaPropertiesChanged(data: MediaPropsCallbackData): void
  private _onTimelinePropertiesChanged(data: TimelinePropsCallbackData): void
  private _onPlaybackInfoChanged(data: PlaybackInfoCallbackData): void
  private _onSessionAdded(media: MediaInfo): void
  private _onSessionRemoved(sourceAppId: string): void
  private _onCurrentSessionChanged(sourceAppId: string): void

  static getMediaSessions(): MediaInfo[]
  static getCurrentMediaSession(): MediaInfo | null
  static getMediaSessionByAppId(sourceAppId: string): MediaInfo | null

  /**
   * Transport controls, each aimed at one session by its AUMID.
   *
   * The boolean reports that the request was accepted, not that the session
   * already changed state — that arrives later through the events. `false`
   * also covers "no session with that id".
   */
  static tryPlay(sourceAppId: string): boolean
  static tryPause(sourceAppId: string): boolean
  static trySkipNext(sourceAppId: string): boolean
  static trySkipPrevious(sourceAppId: string): boolean
  /** Absolute seek, in seconds. */
  static tryChangePlaybackPosition(sourceAppId: string, positionSeconds: number): boolean
  static getCapabilities(sourceAppId: string): PlaybackCapabilities | null

  get sessions(): MediaInfo[]

  on(event: "session-media-changed", listener: (sourceAppId: string, mediaProps: MediaProps) => void): this
  on(event: "session-timeline-changed", listener: (sourceAppId: string, timelineProps: TimelineProps) => void): this
  on(event: "session-playback-changed", listener: (sourceAppId: string, playbackInfo: PlaybackInfo) => void): this
  on(event: "session-added", listener: (sourceAppId: string, media: MediaInfo) => void): this
  on(event: "session-removed", listener: (sourceAppId: string) => void): this
  on(event: "current-session-changed", listener: (sourceAppId: string) => void): this

  destroy(): void
}

export { SMTCMonitor, MediaInfo, MediaProps, PlaybackCapabilities, PlaybackInfo, TimelineProps }
