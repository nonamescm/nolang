<h1>nolang</h1>

<h2>A simple tree-walk interpreter written in rust</h2>

* <a href="#why">Why?</a>
* <a href="#syntax">syntax</a>
* <a href="#todo">todo</a>

<h3>Why</h3>

<p>Just to play with programming languages design</p>

<h3>Syntax</h3>

<p>all existing structures for now</p>

```rust
@ variable
let x = 10


@ Power function

defn(n, by) pow_by =
    if by <= 1 then
        n
    else n*pow_by(n, by-1)

writeln(pow_by(2, 2))

do
    let nome = "nolang"
    writeln(nome + "!!")
end
```

<h3>TODO</h3>

- [ ] Bytecode virtual machine
- [ ] Types
- [ ] Modules
