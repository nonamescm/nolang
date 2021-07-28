# nolang

## A simple tree-walk interpreter written in rust

<p align=center>
	<a href="#why">Why?</a> • 
	<a href="#syntax">syntax</a> • 
	<a href="#todo">todo</a>
</p>

### Why

<p align=center>Just to play with programming languages design</p>

### Syntax

<p align=center>all existing structures for now</p>

```js
@ variable
let x = 10;

@ Power function
let pow_by => (n, by)
	if by == 1;
		n;
	else n*pow_by(n, by-1);
end

writeln pow_by(2, 2);

do
	let nome = "nolang";
	writeln nome + "!!";
done;
```

### TODO

- [ ] Bytecode virtual machine
- [ ] Types
- [ ] Modules
