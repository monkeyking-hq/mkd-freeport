Output docs/src/assets/ranges-and-first.gif
Output docs/src/assets/ranges-and-first.webm

Set Shell "bash"
Set Theme "Catppuccin Mocha"
Set FontSize 16
Set Width 960
Set Height 520
Set Padding 20
Set TypingSpeed 35ms

Hide
Type "export PATH=/vhs/target/x86_64-unknown-linux-musl/release:$PATH"
Enter
Type "export PS1='$ '"
Enter
Type "clear"
Enter
Show

Type "mkd-freeport --first --human"
Enter
Sleep 1.2s

Type "mkd-freeport --bind 127.0.0.1 --range 49152-50000 --first --count 3 --pretty"
Enter
Sleep 1.5s

Screenshot docs/src/assets/ranges-and-first.png
Sleep 1s
