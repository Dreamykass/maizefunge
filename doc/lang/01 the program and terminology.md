# The Program

## Value
32 bits long. Can be interpreted as a number, char, represent a command, etc.
Practically everything is a value, stores values, or manipulates values.

## Space  
Mutable 2D "map" of values (potentially commands) that the program counter
manipulates and navigates around.

## Counter
Moves through the program space, executing commands, 
manipulating its stack and state, as well as the program space itself.

### Stack
Dynamic collection of values, in the form of a stack
(last-in first-out data structure). Unique to a counter.

### State
Fixed collection of values, unique to a counter.
Contains the identifier of the counter, its direction, position, etc.

## Stream
In/out stream that the program can push values into or pop from.
Has a value identifier. 
What it does, depends on the context the program is ran in.
For example, can represent user input and output via the terminal.


## Overview
```
counter = stack + state
program = space(s) + counter(s) + stream
```

