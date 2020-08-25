I recently came across this lesser known feature in Python
and started to play around with it. 

Being able to chain comparison operators is a rarity among most modern
programming languages, which is a shame considering how elegant and intuitive it turns
out to be. 

Here are some examples:

```Python
a, b, c = 5, 10, 15
print(a < b < c)
# True
```
The code above is equivalent to:

```Python
a, b, c = 5, 10, 15
print(a < b and b < c) 
# True
```
And something a bit more complex:

```Python
a, b, c, d, e, f, g = 5, 10, 30, 20, 15, 0, 0
flag = a <= b > f < c > d is not f is g
print(flag)
# True
```
That is all equivalent to:

```Python
a, b, c, d, e, f, g = 5, 10, 30, 20, 15, 0, 0
flag = a <= b and b > f and f < c and c > d and d is not f and f is g
print(flag)
# True
```

You can see how tedious things might get. Python continues to amaze me
with how beautiful and concise it can be.
