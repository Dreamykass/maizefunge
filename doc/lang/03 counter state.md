# Counter State
A collection of values - registers - that can be freely manipulated.
All registers are start with the value of 0 (zero).

## State
```
REGISTER         | MEANING
---------------- | ----------
0                | counter identifier
1                | x coord
2                | y coord
3                | direction (0 is up, 1 is right, 2 is down, 3 is left)
4                | stringmode
5                | charmode
6                | active stream identifier
7                | 
8                | 
9                | 
10               | free-use register
11               | free-use register
12               | free-use register
13               | free-use register
14               | free-use register
15               | free-use register
```

## Commands
```
COMMAND          | RESULT
---------------- | ----------
r (read state)   | pops <n>, reads state at <n> into <v>, pushes <v>
s (set state)    | pops <v>, pops <n>, writes <v> to state at <n> 
                 | 
a-f (set regst.) | pops <v>, sets the free-use register at state a-f with <v>
A-F (get regst.) | reads the free-use register at state a-f into <v>, pushes <v>
```