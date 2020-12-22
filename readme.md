# Semantics Interpreter

This interpreter executes programs, which are represented as expressions.

Supported expression types:
* integer constant
* boolean constant
* arithmetic operations
* equality comparison
* if statements
* variable reads and writes
* function declarations and calls

Within main, several example programs are already defined. Running interpreter.rs as is will demonstrate all of the above expression types by executing the following program:

    function f(top,bot):
      if (bot == 0) then 0 else top/bot

    let bot = 3 in
      (let bot = 2 in bot)
      +
      (f(400+74,bot) + f(470+4,0))

A printout of each expression is provided as it is executed, along with the program counter.
