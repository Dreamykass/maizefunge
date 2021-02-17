# The Program

#### Value
32-bits-long, can be interpreted as a number, char, represent a command, etc.

#### Space  
Mutable 2D "map" of values.

#### Counter
Moves through the program space, executing commands, 
manipulating its stack (of values) and state (direction, etc).
Has a value identifier.

#### Stream
In/out stream that the program can push into or pop from. 
What it does, depends on the context the program is ran in.
For example, can represent user input and output via the terminal.

#### Overview
```
counter = stack + state
program = space(s) + counter(s) + stream
```

