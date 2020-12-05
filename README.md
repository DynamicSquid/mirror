# mirror

A neat little language. Transpiles down to C++, statically typed, and strong. It isn't much, but this is the first time I used Rust.

## Getting Started

To build it from source, you'll need a Rust compiler, a C++ compiler, and some way to run a Bash script. Or you could just use [repl.it](https://repl.it), which is what I used. Just import this GitHub project into repl.it and you're all set! Just make sure to create a new file called `source.mir` where you'll write your actual code.

If you have a repl.it account, fork [this repl](https://repl.it/@DynamicSquid/Mirror#source.mir) and you can start coding right away!

## Syntax

Mirror has a built in `print` function that acts as standard output.

```
print("Hello Mirror!")
```

Variables are created using the `set` keyword.

```
set name = "squid"
```

Conditionals are as expected.

```
if "squid" == "smart" {
  print("squid is smart")
}
else if "squid" == "octopus" {
  print("no")
}
else {
  print("squid")
}
```

Functions are defined using the `def` keyword.

```
def become_squid(string name) {
  print(name)
  print(" is now a squid")
}

become_squid("octopus")
```

Ranged based for loops are also supported.

```
set name = "squid"
for ch in name {
  print(ch)
  print(' ')
}
```

And arrays.

```
set arr = [ 5, 6, 7 ]
for num in arr {
  print(num)
}
```
