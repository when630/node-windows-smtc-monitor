# Node-Windows-SMTC-Monitor

<a href="https://github.com/LeagueTavern/node-windows-smtc-monitor/issues"><img src="https://img.shields.io/github/issues/LeagueTavern/node-windows-smtc-monitor?style=for-the-badge" alt="@coooookies/windows-smtc-monitor downloads"></a>
<a href="https://github.com/LeagueTavern/node-windows-smtc-monitor/actions"><img alt="GitHub CI Status" src="https://img.shields.io/github/actions/workflow/status/LeagueTavern/node-windows-smtc-monitor/CI.yml?style=for-the-badge"></a>
<a href="https://nodejs.org/en/about/releases/"><img src="https://img.shields.io/node/v/%40coooookies%2Fwindows-smtc-monitor?style=for-the-badge" alt="Node.js version"></a>
<a href="https://www.npmjs.com/package/@coooookies/windows-smtc-monitor"><img src="https://img.shields.io/npm/v/@coooookies/windows-smtc-monitor.svg?style=for-the-badge&sanitize=true" alt="@coooookies/windows-smtc-monitor npm version"></a>
<a href="https://npmcharts.com/compare/@coooookies/windows-smtc-monitor?minimal=true"><img src="https://img.shields.io/npm/dm/@coooookies/windows-smtc-monitor.svg?style=for-the-badge&sanitize=true" alt="@coooookies/windows-smtc-monitor downloads"></a>

![Screenshot](docs/screenshot-1.png)

> This is a [Node.js](https://nodejs.org/) toolkit for listening to [SMTC](https://learn.microsoft.com/en-us/uwp/api/windows.media.control.globalsystemmediatransportcontrolssessionmanager?view=winrt-26100) (System Media Transport Controls) media events in Windows. It is written in [Rust](https://www.rust-lang.org/) and utilizes [napi-rs](https://napi.rs/) to implement bindings with Node.js.

English | [简体中文](./README_CN.md)

## 🍴 About this fork

This is [when630/node-windows-smtc-monitor](https://github.com/when630/node-windows-smtc-monitor), a fork of [LeagueTavern/node-windows-smtc-monitor](https://github.com/LeagueTavern/node-windows-smtc-monitor). Upstream reads SMTC sessions; this fork adds **session-targeted transport controls** — see [Controlling a session](#controlling-a-session). It exists for [WHENMUSIC](https://github.com/when630/whenmusic), which needs to seek a specific player, something the media keys cannot do.

It is **not published to npm.** Download the prebuilt binary for your architecture from [Releases](https://github.com/when630/node-windows-smtc-monitor/releases). MIT, same as upstream; the original author's notice is kept intact.

## ⚠️ Warning

`node-windows-smtc-monitor` only supports Windows 10 1809 and later versions (>= 10.0.17763)

## 🚀 Features

- Listen to media events such as play, pause, next track, previous track.
- Get the current playback state and track information.
- Control a session you name: play, pause, next, previous and absolute seek.
- Support for both JavaScript and TypeScript.
- Easy to use and integrate into existing Node.js applications.

## Installation

Upstream, from npm:

```shell
npm i @coooookies/windows-smtc-monitor
```

This fork, from [Releases](https://github.com/when630/node-windows-smtc-monitor/releases) — take the `.node` for your architecture (`win32-x64-msvc`, `win32-ia32-msvc` or `win32-arm64-msvc`), drop it next to your code and `require` it directly:

```Javascript
const smtc = require('./windows-smtc-monitor.win32-x64-msvc.node');
```

## 🍊 Example

[CommonJS Example](example/index.js) <br />
[ESModule Example](example/index.mjs) <br />
[TypeScript Example](example/index.ts) <br />

## Usage

#### Importing the library

```Typescript
// Typescript & ESModule
import { SMTCMonitor } from '@coooookies/windows-smtc-monitor';

// CommonJS
const { SMTCMonitor } = require('@coooookies/windows-smtc-monitor');
```

#### Gets all media sessions

Gets all of the available sessions.

```Typescript
const sessions = SMTCMonitor.getMediaSessions(); // MediaInfo[]
// [
//   {
//     sourceAppId: 'PotPlayerMini64.exe',
//     media: {
//       title: 'ぱられループ を歌ってみた (Jeku remix)',
//       artist: 'Jeku/aori',
//       albumTitle: '',
//       albumArtist: 'ぱられループ を歌ってみた (Jeku remix)',
//       genres: [],
//       albumTrackCount: 0,
//       trackNumber: 0,
//       thumbnail: <Buffer 42 4d 0e ... 1048526 more bytes> // The Album Cover/Thumbnail in Buffer
//     },
//     playback: { playbackStatus: 4, playbackType: 1 },
//     timeline: { position: 217.228, duration: 259 },
//     lastUpdatedTime: 1740000000000
//   },
//   {
//     sourceAppId: 'player.exe',
//     media: { ... },
//     playback: { ... },
//     timeline: { ... },
//     lastUpdatedTime: 1740000000000
//   }
// ]
```

#### Gets the current media session

Gets the current session. This is the session the system believes the user would most likely want to control.

```Typescript
const session = SMTCMonitor.getCurrentMediaSession(); // MediaInfo | null
// {
//   sourceAppId: 'PotPlayerMini64.exe',
//   media: { ... },
//   playback: { ... },
//   timeline: { ... },
//   lastUpdatedTime: 1740000000000
// }
```

#### Gets the specified media session

Gets the specified session by the sourceAppId.

```Typescript
const session = SMTCMonitor.getMediaSessionByAppId('player.exe'); // MediaInfo | null
// {
//   sourceAppId: 'player.exe',
//   media: { ... },
//   playback: { ... },
//   timeline: { ... },
//   lastUpdatedTime: 1740000000000
// }
```

#### Controlling a session

Commands are aimed at one session by its `sourceAppId`, so they land on the player you meant — unlike the keyboard media keys, which always go to whichever session Windows picked. Absolute seeking has no media key at all.

```Typescript
SMTCMonitor.tryPlay('player.exe');           // boolean
SMTCMonitor.tryPause('player.exe');          // boolean
SMTCMonitor.trySkipNext('player.exe');       // boolean
SMTCMonitor.trySkipPrevious('player.exe');   // boolean

// Absolute seek, in seconds
SMTCMonitor.tryChangePlaybackPosition('player.exe', 125.5); // boolean
```

The boolean reports that the request was **accepted**, not that the session already changed state — SMTC applies it a moment later and announces it through `session-playback-changed`. Reading the state back right away can still show the old value. `false` also covers "no session with that id".

Not every player accepts every command. Ask first:

```Typescript
const caps = SMTCMonitor.getCapabilities('player.exe'); // PlaybackCapabilities | null
// {
//   isPlayEnabled: true,
//   isPauseEnabled: true,
//   isStopEnabled: false,
//   isNextEnabled: true,
//   isPreviousEnabled: true,
//   isPlaybackPositionEnabled: true,
//   isFastForwardEnabled: false,
//   isRewindEnabled: false,
//   isPlayPauseToggleEnabled: true,
//   isPlaybackRateEnabled: false,
//   isShuffleEnabled: false,
//   isRepeatEnabled: false
// }
```

#### Using Listeners

If you need to continuously listen for media events, you might consider using the `getMediaSessions` method for polling. However, this approach can be resource-intensive. Instead, `node-windows-smtc-monitor` provides a listener class that allows you to listen for events such as
[GlobalSystemMediaTransportControlsSessionManager.CurrentSessionChanged](https://learn.microsoft.com/en-us/uwp/api/windows.media.control.globalsystemmediatransportcontrolssessionmanager.currentsessionchanged?view=winrt-26100)
[GlobalSystemMediaTransportControlsSessionManager.SessionsChanged](https://learn.microsoft.com/en-us/uwp/api/windows.media.control.globalsystemmediatransportcontrolssessionmanager.sessionschanged?view=winrt-26100) and other related events to monitor media sessions efficiently.

```Typescript
// Register the monitor
const monitor = new SMTCMonitor();

// Normal use
monitor.on('session-media-changed', (appId, mediaProps) => {
  console.log(`Media info changed for ${appId}`, mediaProps);
});

// Using a listener defined outside
const listener = (appId, playbackInfo) => {
  console.log(`Playback state changed for ${appId}`, playbackInfo);
};

monitor.on('session-playback-changed', listener); // Register the listener
monitor.off('session-playback-changed', listener); // Unregister the listener

console.log(monitor.sessions)
// Shows all the sessions

// Destroy monitoring when done
// monitor.destroy();
```

Here is a list of available events:

| Event Name               | Description                                 | Parameters                                    |
| ------------------------ | ------------------------------------------- | --------------------------------------------- |
| session-media-changed    | Triggered when media info changes           | (appId: string, mediaProps: MediaProps)       |
| session-timeline-changed | Triggered when position or duration changes | (appId: string, timelineProps: TimelineProps) |
| session-playback-changed | Triggered when playback state changes       | (appId: string, playbackInfo: PlaybackInfo)   |
| session-added            | Triggered when a new media session is added | (appId: string, mediaInfo: MediaInfo)         |
| session-removed          | Triggered when a media session is removed   | (appId: string)                               |
| current-session-changed  | Triggered when the current session changes  | (appId: string)                               |

## Using in Electron

To use `node-windows-smtc-monitor` in Electron, you need to run it in a Worker thread. Running it in the main process will cause the main thread to lock up, which will freeze the renderer process. An example of how to use it in a Worker is provided in `example/worker.js`. <br />

[Worker Example](example/worker.js)

## License

This project is licensed under the [MIT](LICENSE) License.
