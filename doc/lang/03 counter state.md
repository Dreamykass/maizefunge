# Counter State

## State
```
N                | STATE
---------------- | ----------
0                | identifier
1                | x coord
2                | y coord
3                | direction (0 is up, 1 is right, 2 is down, 3 is left)
4                | stringmode
5                | 
6                | 
7                | 
8                | 
9                | 
A (10)           | free-use register
B (11)           | free-use register
C (12)           | free-use register
D (13)           | free-use register
E (14)           | free-use register
F (15)           | free-use register
```


## Commands
```
COMMAND          | RESULT
---------------- | ----------
r (read state)   | pops <n>, reads state at <n> into <v>, pushes <v>
                 |
s (set state)    | pops <v>, pops <n>, writes <v> to state at <n> 
                 | 
```