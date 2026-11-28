Output docs/src/assets/foreground-reserve.gif
Output docs/src/assets/foreground-reserve.webm

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

Type "mkd-freeport --reserve --bind 127.0.0.1 --count 2 --timeout 3 --human"
Enter
Sleep 3.5s

Type "mkd-freeport --reserve --bind 127.0.0.1 --human"
Enter
Sleep 1.5s
Ctrl+C
Sleep 1s

Screenshot docs/src/assets/foreground-reserve.png
Sleep 1s
