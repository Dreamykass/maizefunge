# Threading

## Commands
```
COMMAND          | RESULT
---------------- | ----------
f (fork)         | pops <delta>, pops <identifier>
                 | creates a new counter with <identifier>
                 | which has the same state as the current one,
                 | except for position, which is advanced forward <delta> times 
```