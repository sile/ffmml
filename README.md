ffmml
=====

[![ffmml](https://img.shields.io/crates/v/ffmml.svg)](https://crates.io/crates/ffmml)
[![Documentation](https://docs.rs/ffmml/badge.svg)](https://docs.rs/ffmml)
[![Actions Status](https://github.com/sile/ffmml/workflows/CI/badge.svg)](https://github.com/sile/ffmml/actions)
![License](https://img.shields.io/crates/l/ffmml)

Parser and player of **F**amicon **F**lavored **M**usic **M**acro **L**anguage (FFMML).

Famicon is also known as NES (Nintendo Entertainment System).

🎵 [Web Player][demo]

The language specification of FFMML is based on [MCK].
But there are (known) differences between FFMML and MCK as follows:

- FFMML doesn't support the following features:
  - `#INCLUDE` directive
  - `#OCTAVE-REV` directive
  - `@n` command (direct frequency select)
  - `n` command (direct note select)
  - `y` command (direct memory entry)
  - DPCM channel
- FFMML features `#CHANNEL <CHANNEL_NAME> <OSCILLATOR>` directive that defines custom channels:
  - `<CHANNEL_NAME>`: `A..=Z`
  - `<OSCILLATOR>`: `1` (pulse wave), `2` (triangle wave), or `3` (noise)

[MCK]: https://www.nesdev.org/mckc-e.txt

Examples
--------

The following example parses an MML script and generates audio data:
```rust
let mml = r#"; From https://www.nesdev.org/mck_guide_v1.0.txt
#TITLE My First NES Chip
#COMPOSER Nullsleep
#PROGRAMER 2003 Jeremiah Johnson

@v0 = { 10 9 8 7 6 5 4 3 2 }
@v1 = { 15 15 14 14 13 13 12 12 11 11 10 10 9 9 8 8 7 7 6 6 }
@v2 = { 15 12 10 8 6 3 2 1 0 }
@v3 = { 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0 }

ABCD t150

A l8 o4 @01 @v0
A [c d e f @v1 g4 @v0 a16 b16 >c c d e f @v1 g4 @v0 a16 b16 >c<<]2

C l4 o3 q6
C [c e g8 g8 a16 b16 >c8 c e g8 g8 a16 b16 >c8<<]4

D l4 @v2 @0
D [@v2 b @v3 e @v2 b @v3 e @v2 b @v3 e @v2 b @v3 e8 @v2 b8]4"#;

let music = mml.parse::<ffmml::Music>().unwrap_or_else(|e| panic!("{e}"));
let mut player = music.play(48000);
let audio_data = (&mut player).collect::<Vec<_>>();
player.take_last_error().map(|e| panic!("{e}"));
```

To play the music defined by the above MML script, run the following commands:
```console
$ cargo install ffmmlc
$ cat examples/music01.mml | ffmmlc > music01.wav
$ play music01.wav
```

References
----------

### About MML

- [MCK reference](https://www.nesdev.org/mckc-e.txt)
- [MCK guide](https://www.nesdev.org/mck_guide_v1.0.txt)
- [MCK reference (Japanese)](https://wikiwiki.jp/mck/MML%E3%83%AA%E3%83%95%E3%82%A1%E3%83%AC%E3%83%B3%E3%82%B9)
- [What's MML? (Japanese)](https://geolog.mydns.jp/www.geocities.co.jp/Playtown-Denei/9628/whatsmml.html)

### Abount Famicon sound

- [NES APU (audio processing unit)](https://www.nesdev.org/wiki/APU)
- [FC sound source (Japanese)](https://dic.nicovideo.jp/t/a/fc%E9%9F%B3%E6%BA%90)
- [MCK Wiki (Japanese)](https://wikiwiki.jp/mck/%E3%83%95%E3%82%A1%E3%83%9F%E3%82%B3%E3%83%B3%E9%9F%B3%E6%BA%90%E8%A9%B3%E7%B4%B0)

[demo]: https://sile.github.io/ffmml?mml=%3B+From+https%3A%2F%2Fwww.nesdev.org%2Fmck_guide_v1.0.txt%0A%23TITLE+My+First+NES+Chip%0A%23COMPOSER+Nullsleep%0A%23PROGRAMER+2003+Jeremiah+Johnson%0A+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++%40v0+%3D+%7B+10+9+8+7+6+5+4+3+2+%7D%0A%40v1+%3D+%7B+15+15+14+14+13+13+12+12+11+11+10+10+9+9+8+8+7+7+6+6+%7D%0A%40v2+%3D+%7B+15+12+10+8+6+3+2+1+0+%7D%0A%40v3+%3D+%7B+15+14+13+12+11+10+9+8+7+6+5+4+3+2+1+0+%7D++++++++++++++++++++++++++++++++++++%0AABCD+t150%0A%0AA+l8+o4+%4001+%40v0%0AA+%5Bc+d+e+f+%40v1+g4+%40v0+a16+b16+%3Ec+c+d+e+f+%40v1+g4+%40v0+a16+b16+%3Ec%3C%3C%5D2%0A+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++C+l4+o3+q6+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++C+%5Bc+e+g8+g8+a16+b16+%3Ec8+c+e+g8+g8+a16+b16+%3Ec8%3C%3C%5D4%0A+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++D+l4+%40v2+%400%0AD+%5B%40v2+b+%40v3+e+%40v2+b+%40v3+e+%40v2+b+%40v3+e+%40v2+b+%40v3+e8+%40v2+b8%5D4
