# 0.1.3 (Nov. 24th, 2022)

* Use `zopfli` `ZLib` format, since the raw `Deflate` format doesn't seem to work with Chrome anymore.
* Bump dependencies

# 0.1.2 (Aug. 25th, 2022)

* Bump `zopfli` crate and use `deflate` compression because it stores slightly less metadata.

# 0.1.1 (Feb. 8th, 2022)

* Replaced the `png` crate with `lodepng` combined with `zopfli` for improved compression.

# 0.1.0 (Sep. 23rd, 2021)

* Initial release.
