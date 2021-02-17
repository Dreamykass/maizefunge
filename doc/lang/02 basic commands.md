# Basic Commands

## Stack manipulation
```
COMMAND          | RESULT
---------------- | ----------
0-F (digits)     | pushes <0-15> 
                 | (0 pushes 0, 5 pushes 5, A pushes 10, etc)
+ (add)          | pops <a>, pops <b>, pushes <b+a>
- (subtract)     | pops <a>, pops <b>, pushes <b-a>
* (multiply)     | ..., pushes <b*a>
/ (divide)       | ..., pushes <b/a>
% (modulo)       | ..., pushes <b%a>
` (greater)      | ..., pushes <1 if b is greater than a, otherwise 0>
! (not)          | pops <a>, pushes <0 if a is non-zero, otherwise 1>
                 |
\ (swap)         | pops <a>, pops <b>, pushes <a>, pushes <b>
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
  (space/null)   | counter moves forward in the current direction
< (left)         | counter rotates to absolute west/left and moves forward 
> (right)        | counter rotates to absolute east/right and moves forward 
^ (up)           | counter rotates to absolute north/up and moves forward 
v (down)         | counter rotates to absolute south/down and moves forward 
[ (ccw)          | counter rotates counter-clockwise and moves forward
] (cw)           | counter rotates clockwise and moves forward
? (random)       | counter rotates to absolute random direction and moves forward
# (bridge)       | counter moves forward twice (skipping a command) 
                 |
_ (horizontal if)| pops <val>, acts like < if <val> is truthy, otherwise >
| (vertical if)  | pops <val>, acts like ^ if <val> is truthy, otherwise v
```

## Special
```
COMMAND          | RESULT
---------------- | ----------
@ (end)          | terminates the counter, if all counters are terminated,
                 | program is considered finished
" (stringmode)   | toggles stringmode state
                 | while active, pushes unicode values of commands
                 |
. (stream pop)   | pops <v> from the in/out stream, pushes <v> 
, (stream push)  | pops <v>, pushes <v> into the in/out stream
                 |
g (get)          | pops <y>, pops <x>,
                 | reads <v> at (x,y) coord, pushes <v>
p (put)          | pops <y>, pops <x>, pops <v>, 
                 | writes <v> at (x,y) coord of current space
```


