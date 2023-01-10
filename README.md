# Crust
A C test framework written in Rust. 🛠

To compile the main.rs file run:
```
rustc main.rs
```

Run the program using two arguments: C file, test file:

Win:
```
main.exe file.c test.txt
```

Unix:
```
./main file.c test.txt
```
The executable compiles the C file, runs it and compares the content (ignoring whitespaces) of the two files.

If the test passes you should get:
```
Test passed ✅
```
Else:
```
Test failed ❌
```

Happy hacking! 🐱‍💻
