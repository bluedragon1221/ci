# `ci-lisp`
A minimal lisp based on [Lambda Calculus](https://en.wikipedia.org/wiki/Lambda_calculus), focused on easy writing and easy parsing.

## Getting started
Make sure you have `git` and `cargo` installed, then clone the `ci` monorepo (its just `ci-lisp` here for now):
```
git clone https://github.com/bluedragon1221/ci
cd ci/ci-lisp
```

Now you can run it:
```
$ cargo run -- --help
Usage: ci-lisp [OPTIONS]

Options:
  -i <PRELOAD>      Name of library to preload
  -m                Treat every line as an infix {...}
      --math        Enable built-in math functions. eg. add, sub, inc, dec, etc
  -h, --help        Print help
  -V, --version     Print version
```

For the full ci-lisp experience, try this command
```
cargo run -- --math -i ../lib/ext_math.ci ../lib/ext_symbols.ci -m
```

## Overview
### Math
Functions can only take one argument.
For operations that need to take multiple arguments, we use currying or pairs

You can think of curried math operators as "do this to that".
For example, "add 2 to 3":
```lisp
〉((add 2) 3)
5
```
Or "subtract 4 from 7":
```lisp
〉((sub 4) 7)
3
```

A natural extension of this paradigm is that you don't have to say what "that" is:
```lisp
〉(add 3)
<native fn>
```
> (Better string formatting for functions coming soon!)

Say we define `(add 3)` as its own function:
```lisp
〉((def (add 3)) 'my_fn)
nil
```

Now we can call it just like any other function:
```lisp
〉(my_fn 3)
6
```

### Functions
You define a function in this form: `((def body) 'name)`.
Yes, it might seem a little backwards to put the body before the name, but it makes sense once we introduce infix syntax later.
This also introduces quote `'` syntax.
Quoting means that you are refering to the _literal symbol_.
If we were instead to just put `name` in our function declaration, it would immediately try to look up `name` in the environment, see that it does not exist, and error.

### Infix
At this point, the `((f b) a)` pattern is getting annoying.
In some cases, the order "a f b" might make more sense.
In these cases, infix syntax can help.

For example, in math:
```lisp
{3 add 2}
{7 sub 4}
```

The define function can also be written using infix syntax:
```lisp
{'my_fn def (add 3)}
```

The library `lib/ext_symbols.ci` defines symbolic equivalents for many common functions, such as add, sub, and def:
```lisp
(include "lib/ext_symbols.ci")

{3 + 2}
{7 - 4}
{'my_fn = (add 3)}
```

Launching the repl with `-m` enables infix-repl mode, where it treats every line as an infix.
That means you can type lines like this:
```
'g = 7
'b = {g + 2}
```

### Datatypes
All complex datatypes are [church-encoded](https://en.wikipedia.org/wiki/Church_encoding).
This means something like a pair is really just a special function that other functions know how to handle.
You can see this in the standard library, in the definition for `cons`, the function that makes a pair:
```lisp
{'cons = (fn 'b (fn 'a (fn 'u ((u a) b))))}
((cons "last") "first")
```

![Wait, it's all just functions!? Always has been](./fns_meme.jpg)

`ext_symbols` provides us with sugar for making pairs:
```
{'g = {3 : 2}}
```

#### Lists
Lists are constructed using cons-pairs, just like in lisp:
```lisp
{1 : {2 : {3 : {4 : {5 : nil}}}}}
```

This is annoying, so we have a special syntax for lists:
```lisp
{'l = [1 2 3 4 5]}
```
Much cleaner!

You can get the nth item of a list (indicies start at 0):
```lisp
((nth 3) l)
```

#### Fractions
Fractions are a first-class citizen of ci-lisp.

```lisp
〉'f = ((frac 2) 3)
nil
〉(fmt_frac f)
3/2
```

`ext_symbols` provides `/` for constructing a fraction:
```lisp
'f = {2 / 3}
```

You can add, multiply and simplify fractions:
```lisp
〉'f = {2 / 3}
nil
〉'g = {1 / 4}
nil
〉'h = {f fadd g}
nil
〉(fmt_frac h)
3/12
```

### Higher-Order Functions
You can compose functions using `compose`:
```lisp
〉'f = (add 2)
nil
〉'g = (mul 3)
nil
〉'h = ((compose g) f)
nil
〉(h 1)
5
```

Its symbolic equivelant is `.`, like in haskell:
```lisp
〉'f = (add 2)
nil
〉'g = (mul 3)
nil
〉'h = {f . g}
nil
〉(h 1)
5
```

Sometimes the opposite order makes sense, similar to a pipe from POSIX shell:
```lisp
〉'i = { f |> g }
nil
〉(i 3)
15
```

### `map` et al.
You can use `map` and `fold{r,l}`, just like in functional languages.

When calling `map`, pass the function, then the data, like so:
```lisp
〉'a = [1 2 3 4 5]
nil
〉'b = ((map (fn 'x {x + 2})) a)
nil
〉(fmt_list b)
"[ 3 4 5 6 7 ]"
```

Just like before, you don't have to pass the data right away:
```lisp
〉'bulk-add-two = (map (fn 'x {x + 2}))
nil
〉(fmt_list (bulk-add-two [1 2 3 4 5]))
"[ 3 4 5 6 7 ]"
```

As an example of using foldl and foldr, lets write a function that multiplies a list of numbers.
To display another feature though, let's say I forgot the order of the arguments in the `foldr` function.
````lisp
〉(help foldr)
**Description**:
> Applies the foldr function.
> Args:
> - `f`: Define how to merge one item with the accumulator from the right. ex. `(fn 'x (fn 'acc ...))`
> - `acc`: The default value to start with. ex. `[]`
> - `lst`: The list of elements you're applying the fold to

**Definition:**
```lisp
(fn 'f (fn 'acc (fn 'lst ((((if (not lst)) (fn '_ acc)) (fn '_ ((f (car lst)) (((foldr f) acc) (cdr lst))))) nil))))
```
nil
````
> (Most of the functions in `ext_math` have docs built in. Try it out!)

From that information, we figure out that a sum function might look like this (I've split the accumulator function `f` into its own definition to make things cleaner):
```lisp
〉'f = (fn 'x (fn 'acc {x + acc}))
nil
〉'sum-lst = ((foldr f) 0)
nil
〉(sum_lst [1 2 3 4 5])
15
```

For `foldl`, lets look at the library definition for `fmt_list` (slightly modified), which uses the `foldl` function:
```lisp
{'fmt_list_inner = ((foldl (fn 'acc (fn 'x {acc .. {" " .. x}}))) "")}
{'fmt_list = (fn 'lst {"[" .. {(fmt_list_inner lst) .. " ]"}})}
```

In general, `foldl` makes more intuitive sense.
However, if your output type is a list, consider using `foldr` instead.
This is because there is currently no way to append to the end of a list.
This is an explicit design choice, as `append` would have an `O(n)` time complexity, while `cons` is `O(1)`.
See [this Stack Overflow post](https://stackoverflow.com/questions/33304408/is-it-faster-traditional-to-use-append-vs-cons-and-reverse-in-scheme) for more information.

As an example of `foldr` with lists, lets look at the library definition of `map`, which is defined using foldr.
```lisp
{'map = (fn 'f
  ((foldr (fn 'x (fn 'acc {(f x) : acc}))) [])
)}
```
If you think about it, its kinda backwards.
Think about it as starting from the _right_, and moving left, building your list by placing items at the beginning.
Its quite elegant, isn't it?

---

That's all for now! As I add new features, I'll update this document accordingly.
If there's anyone out there crazy enough to actually use this thing, please report bugs as you find them!
I'm sure this thing isn't bug-free.
