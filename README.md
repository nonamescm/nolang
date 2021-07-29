<h1>nolang</h1>

<h2>A simple tree-walk interpreter written in rust</h2>

* <a href="#why">Why?</a>
* <a href="#syntax">syntax</a>
* <a href="#todo">todo</a>

<h3>Why</h3>

<p>Just to play with programming languages design</p>

<h3>Syntax</h3>

<p>all existing structures for now</p>

```js
@ variable
let x = 10;


@ Power function

let pow_by => (n, by)
    if by <= 1;
        n;
    else n*pow_by(n, by-1);
end

writeln(pow_by(2, 2));

do
    let nome = "nolang";
    writeln(nome + "!!");
done;
```

<h3>TODO</h3>

- [ ] Bytecode virtual machine
- [ ] Types
- [ ] Modules
