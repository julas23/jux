# xSight (`xsight`)

**Image viewer for JUX.**

Opens instantly, shows the image correctly, and gets out of the way. Fast enough to be
the default handler for a directory of thousands of photographs.

## Scope

- Common formats, plus HEIC, AVIF and camera RAW
- GPU-accelerated pan and zoom
- Directory browsing, basic rotation and cropping
- EXIF display

## Status

**Early planning.** No implementation yet.

## Known hard parts

- **Colour management.** Honouring embedded ICC profiles is what separates a viewer that
  shows the right colours from one that shows approximately the right colours.
- **Very large images**, without loading the whole thing into memory.
- **RAW decoding**, which varies per camera vendor.
