# Counter State
A collection of values - registers - that can be freely manipulated.
All registers are start with the value of 0 (zero).

## State
```
REGISTER         | MEANING
---------------- | ----------
0                | counter identifier
1                | x coord (width: +right, -left)
2                | y coord (height: +down, -up)
3                | direction (0 is up, 1 is right, 2 is down, 3 is left)
4                | current space identifier
5                | stringmode
6                | charmode
7                | active stream identifier
8                | remaining sleep
9                | 
A (10)           | free-use register A
B (11)           | free-use register B
C (12)           | free-use register C
D (13)           | free-use register D
E (14)           | free-use register E
F (15)           | free-use register F
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