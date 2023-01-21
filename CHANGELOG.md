Change Log
==========

All notable changes to the "ffmml" extension will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

[Unreleased]
------------

### Fixed

- Fix block comment handling bug
- Allow empty channels such as `A` in `A B cde`
- Fix typo: s/#CANNEL/#CHANNEL/

[0.1.2] - 2023-01-17
--------------------

### Fixed

- Fix tie and slur commands handling bug
- Fix a bug that `MusicPlayer::take_last_error()` always returns `None`

### Added

- Expose `Sample` type.
- Add `wav` feature
