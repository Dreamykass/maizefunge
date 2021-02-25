# Threading

## General
A counter can create a new counter. Counters are advanced each generation, in 
order of creation.

## Commands
```
COMMAND          | RESULT
---------------- | ----------
f (fork)         | pops <delta>, pops <identifier>
                 | creates a new counter with <identifier>
                 | which has an empty stack, and the same state as the current one,
                 | except for position, which is advanced forward <delta> times 
                 |  
z (sleep)        | pops <n>, moves forward, sleeps for the next <n> generations  
                 | if <n> is 0, it has same effect as space/null
```