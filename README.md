# average color

gets top colors from a image

## how to use

install rust first if you dont have it

then run:

```
cargo run path/to/your/image.png
```

it will show the top 15 most used colors in hex format with percentages

## flags

```
--pixel              show pixel counts too
--json               output as json
--csv                output as csv
--xml                output as xml
--toml               output as toml
--threshold N        set color grouping threshold (default 5)
--no-bw              exclude black and white colors
```

## what it does

- reads image
- downsamples if bigger than 4k to keep things fast
- skips mostly transparent pixels (alpha < 95)
- buckets colors by threshold value to group similar ones
- shows top 15 colors with percentages

## example output

default:
```
#ffffff - 45.23%
#000000 - 23.45%
#ff5733 - 12.34%
```

with --pixel:
```
#ffffff - 45.23% (12534 pixels)
#000000 - 23.45% (8234 pixels)
#ff5733 - 12.34% (3421 pixels)
```

with --json:
```json
[
  {"hex": "#ffffff", "percentage": 45.23, "pixels": 12534},
  {"hex": "#000000", "percentage": 23.45, "pixels": 8234}
]
