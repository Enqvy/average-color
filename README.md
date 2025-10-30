# average color

gets top colors from a image

## how to use

install rust first if you dont have it

then run:

```
cargo run path/to/your/image.png
```

it will show the top 15 most used colors in hex format

## what it does

- reads image
- counts all the pixels colors
- combines colors that are really similar (less than 5 rgb difference)
- shows top 15 colors with how many pixels

## example output

```
#ffffff - 12534 pixels
#000000 - 8234 pixels
#ff5733 - 3421 pixels
```

thats it

