# bondrewdsay

![image](https://user-images.githubusercontent.com/44780846/184599502-ff3273d7-7499-4d01-b060-7f1cb4028a1e.png)

## Usage

```
bondrewdsay 0.1.0
( |) < May your journey overflow with curses and blessings

USAGE:
    bondrewdsay [OPTIONS] [TEXT]

ARGS:
    <TEXT>    Text to display (If omitted, standard input is accepted)

OPTIONS:
    -d, --direction <DIRECTION>    Direction of Text [default: right] [possible values: up, left,
                                   right]
    -h, --help                     Print help information
    -l, --lefthand <TEXT>          Sir Bondrewd's Left Hand
    -r, --righthand <TEXT>         Sir Bondrewd's Right Hand
    -V, --version                  Print version information
```

## Example

```
$ bondrewdsay -d right "ナナチ 元気そうで何よりです"
( |) < ナナチ 元気そうで何よりです

$ bondrewdsay -d left "君はかわいいですね"
君はかわいいですね > (| )

$ bondrewdsay -d up -l ✋ -r 🤚 慈しみ合う心こそがヒトを家族たらしめるのです
       ⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽⎽
     < 慈しみ合う心こそがヒトを家族たらしめるのです >
       v⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺⎺
 ✋（｜）🤚
```

## Build

```sh
cargo build --release
```
