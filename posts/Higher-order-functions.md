The higher-order function is one of the many powerful and important concepts I have run into when dabbling in the world of functional programming.

Before diving into the details, it's worth mentioning that in certain languages like Python or Javascript, functions are treated as first-class citizens. What does this mean? It means that functions can be stored in variables, passed around to functions or returned from functions, just as you would be able to do with a primitive or non-primitive data type.

Knowing this, a higher-order function is a function that either takes in a function as input or returns a function as output.

Examples of higher-order functions that you may deal with on a daily basis but may be unaware of the underlying concept: `filter`, `map`, `reduce`, `sort`.

Now that definitions are taken care of, let's take a look at some examples:

### Filter

Problem: *given an arbitrary iterable, filter out elements whose value fail to meet a certain criteria.*

The `filter` function takes in a single-argument predicate function (a function that returns a boolean value) and an iterable, and returns a new iterable that consists of each element in the passed in iterable that when passed into to the predicate function evaluate to true.

```python
def filter(func, iterable):
  return [el for el in iterable if func(el)]

print(filter(lambda x: x & 1, [1, 2, 3, 4, 5])) # [1, 3 ,5]
```

### Map

Problem: *given an arbitrary iterable, apply some transformation to each element in that iterable.*

The `map` function takes in a single-argument function and an iterable applies that function to each item in the iterable, returning the resulting iterable.

```python
def map(func, iterable):
  return [func(x) for x in iterable]

print(map(lambda x: x // 2, [2, 4, 6, 8])) # [1, 2, 3, 4]
```

### Reduce

Problem: *given an arbitrary iterable, accumulate some transformation to each element in that iterable and return the resulting value.*

The `reduce` function takes in a two-argument function and an iterable and applies that function to elements in the iterable, accumulating the result along the way.

```python
def reduce(func, iterable, init=None):
  it  = iter(iterable)
  val = next(it) if init is None else init

  for el in it:
    val = func(val, el)

  return val

print(reduce(lambda x, y: x * y, [1, 2, 3])) # 6
```
As we can see above, we first deal with our initializer value, if none is present we use the
first element in the list.

We then run through the list, applying the passed in `func` with our current value and each subsequent element in the list.

### Composition of programs

Now that we've gone through some basic examples, let's see how this pattern might be useful in a practical
scenario.

Problem: *write a program that outputs the sum of naturals, squares and cubes from numbers 1 to N where N is some arbitrary input*

Seems easy enough? Without the use of higher order functions, we can simple write the program like so:

```python
def natural(N):
  return sum([x for x in range(1, N + 1)])

def square(N):
  return sum([x ** 2 for x in range(1, N + 1)])

def cube(N):
  return sum([x ** 3 for x in range(1, N + 1)])

def main(N):
  for func in [natural, square, cube]:
    print(func(N))

if __name__ == '__main__':
  main(int(input()))
```

Notice that the three different functions are actually very similar, the only difference is the *transformation
of data* we perform on each number in the sequence.

We can introduce a higher order function that *applies* this transformation to the data.

```python
def summation(func, N):
  return sum(list(map(func, range(1, N + 1))))

def main(N):
  for func in [lambda x: x, lambda x: x ** 2, lambda x: x ** 3]:
    print(summation(func, N))

if __name__ == '__main__':
  main(int(input()))
```

### Decorators

It's worth covering another type of higher-order function, one that *returns* a function. In Python, the idea of a *decorator* is simply a function that takes in another function and returns a new one with some modified behaviour.

Here's a simple example of a timer, a function that computes the time of a function call.

```python
import time

def timer(func):
  def wrap(*args, **kwargs):
    start = time.perf_counter()
    func(*args, **kwargs)
    end   = time.perf_counter()
    print(f'Function {func.__name__} executed in {end - start:.4f}s')
  return wrap

@timer
def waste(n):
  for _ in range(n):
    pass

def main():
  waste(2000000)

if __name__ == '__main__':
  main() # Function waste executed in 0.0396s
```

As we can see, the function `timer` takes in a function as input and returns a wrapper function that modifies the behaviour of the passed in function by performing operations before and after the function call. Now the new function will embody this new behaviour whenever it is called.

### Fin

Being able to treat functions as first-class citizens in your programming language is a powerful idea to wrap your head around and a worth while design choice when composing large scale and complex applications.

All of the functions written for this post can be found here ->
[https://gist.github.com/terror/4d86aaf49cc724d0bfe5af11d05da88e](https://gist.github.com/terror/4d86aaf49cc724d0bfe5af11d05da88e)
