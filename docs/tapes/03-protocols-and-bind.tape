Output docs/src/assets/protocols-and-bind.gif
Output docs/src/assets/protocols-and-bind.webm

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

Type "mkd-freeport --protocol udp --bind 127.0.0.1 --count 2 --pretty"
Enter
Sleep 1.5s

Type "mkd-freeport --protocol tcp --bind 127.0.0.1 --human"
Enter
Sleep 1.2s

Screenshot docs/src/assets/protocols-and-bind.png
Sleep 1s
