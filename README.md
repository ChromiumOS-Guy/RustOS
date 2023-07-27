# RustOS
An Operating System in The Rust Programing Language

### Bugs:
1. currently Rust String lib (Rstr) converts numbers to strings in reverse which requires to reverse the string when its done, and all ways I could think of to reverse the string have critical memory leaks and all rust implemented ways (which I know of) have memory leaks this means its not possible for me to actually have numbers display correctly

### Info:
the actual code is in the RustOS Branch also when you compile this you need to be on linux if anyone thought differently

### Credits:
current bootloader from PonchoOS guide (cuz I got lazy + I ain't making a bootloader in Rust)
