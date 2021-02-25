# Basic Commands

## Stack manipulation
```
COMMAND          | RESULT
---------------- | ----------
0-9 (digits)     | pushes <0-9> 
                 | (0 pushes 0, 5 pushes 5, 8 pushes 8, etc)
+ (add)          | pops <b>, pops <a>, pushes <a+b>
- (subtract)     | pops <b>, pops <a>, pushes <a-b>
* (multiply)     | ..., pushes <a*b>
/ (divide)       | ..., pushes <a/b>
% (modulo)       | ..., pushes <a%b>
` (greater)      | ..., pushes <1 if a is greater than b, otherwise 0>
! (not)          | pops <a>, pushes <0 if a is non-zero, otherwise 1>
~ (sign)         | pops <a>, pushes <a *-1>
                 |
\ (swap)         | pops <b>, pops <a>, pushes <b>, pushes <a>
$ (pop)          | pops <v>
: (duplicate)    | pops <v>, pushes <v>, pushes <v>
; (peek under)   | pops <d>, peeks at <x> that is <d> elements
                 | under the top element of the stack, pushes <x>
                 | (if <d> is 0, ; acts same as :)
```

## Counter Movement
```
COMMAND          | RESULT
---------------- | ----------
  (space/null)   | moves forward in the current direction
< (left)         | rotates to absolute west/left and moves forward 
> (right)        | rotates to absolute east/right and moves forward 
^ (up)           | rotates to absolute north/up and moves forward 
v (down)         | rotates to absolute south/down and moves forward 
[ (ccw)          | rotates counter-clockwise and moves forward
] (cw)           | rotates clockwise and moves forward
? (random)       | rotates to absolute random direction and moves forward
# (bridge)       | moves forward twice (skipping a command) 
                 |
_ (horizontal if)| pops <val>, acts like < if <val> is truthy, otherwise >
| (vertical if)  | pops <val>, acts like ^ if <val> is truthy, otherwise v
```

## Special
```
COMMAND          | RESULT
---------------- | ----------
@ (end)          | terminates the program, including all counters
                 |
" (stringmode)   | toggles stringmode state
                 | while active, pushes unicode values of commands
' (charmode)     | toggles charmode state, if active, 
                 | pushes one unicode value of command, and deactivates
                 |
. (stream pop)   | pops <v> from the currently active in/out stream, pushes <v> 
, (stream push)  | pops <v>, pushes <v> into the currently active in/out stream
                 |
g (get)          | pops <y>, pops <x>,
                 | reads <v> at (x,y) coord, pushes <v>
p (put)          | pops <y>, pops <x>, pops <v>, 
                 | writes <v> at (x,y) coord of current space
```


