pseudo-befunge of how I'd implement function calls:
```cpp
push a, push b, // coords to go back to
add arg // arg for function
push x, push y, goto x y // now we're at function coord
pop arg, push result, // function did its stuff
// now our stack looks like: a, b, result
// we do some command(s) to make it so stack looks like: result, a, b
goto a b
 ```


rubenwardyToday at 23:19  
what about standard libaries and other files?

DreamykassToday at 23:21  
I thought about allowing to jump into another program space  
so yeah, you could push a string with the file name to load as a program space and jump there (and then back)

rubenwardyToday at 23:22  
yeah, that works unless you have op overloading  
I guess op overloading isn't needed  

DreamykassToday at 23:23  
hmm  
since the counter already needs to track some state (direction, position), why not let it modify that state  
and if it tracks direction and position, why not let it have some registers (?)  
so let's say 0 is direction, 1 and 2 are position, 3 is stringmode, 4 to 8 are free-use  
or something like that  

