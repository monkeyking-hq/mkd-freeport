Output docs/src/assets/overview.gif
Output docs/src/assets/overview.webm

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

Type "mkd-freeport"
Enter
Sleep 1s

Type "mkd-freeport --human"
Enter
Sleep 1s

Type "mkd-freeport --plain"
Enter
Sleep 1s

Type "mkd-freeport --protocol udp --count 2 --pretty"
Enter
Sleep 1.2s

Type "mkd-freeport --reserve --bind 127.0.0.1 --timeout 2 --human"
Enter
Sleep 2.5s

Screenshot docs/src/assets/overview.png
Sleep 1s
