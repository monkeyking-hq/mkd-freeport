Output docs/src/assets/output-formats.gif
Output docs/src/assets/output-formats.webm

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

Type "mkd-freeport --pretty"
Enter
Sleep 1.2s

Type "mkd-freeport --human"
Enter
Sleep 1s

Type "PORT=$(mkd-freeport --plain) && echo \"Allocated port: $PORT\""
Enter
Sleep 1.2s

Screenshot docs/src/assets/output-formats.png
Sleep 1s
