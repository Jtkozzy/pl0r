# pl0r
PL/0 implementation in Rust 

- Uses project file structure similar to my earlier Free Pascal version
- Scanner is a rustified, adapted copy of munificent's "Crafting Interpreters" book Java scanner (read the book ! It's excellent !)
- Parser is quite direct conversion of Pascal version, uses lots of match :)
- Parser does not try to recover/resync but quits on first error a'la old Turbo Pascal (Pascal version used heavily Pascal's sets and I couldn't figure out a simple way to do this in Rust. Well, PL/0 programs tend to be short so this version should be adequate)
- Parser does not use nested procedures like Pascal version, so needs some added function parameters for nesting level and variable table index etc
- Interpreter is a direct conversion from Pascal version
- This shows how bad I am as a Rust programmer :) .. clone(), clone() everywhere.
