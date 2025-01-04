## Exercise 8

In this question we are asked to explore the consequences of assignment, and to write a function `f` such that evaluating `f(0) + f(1)` from 0 if evaluated from left to right and 1 if evaluated from right to left.

```
function createF() {
    let value = 0;
    return function(x) {
        let old_value = value;
        value = x;
        return old_value;
    };
}

const f = createF();

f(0) + f(1)
```

## Exercise 13

In this question we are working with the following function

```
function make_cycle(x) {
	set_tail(last_pair(x), x);
	return x;
}
```

We then define 

```
const z = make_cycle(list("a", "b", "c"));
```

What happens if we compute `last_pair(z)`? In that instance the `last_pair` function will iterate forever and never terminate, since `z` is a cycle.

## Exercise 17

In this exercise we are asked to devise a function `count_pairs` which counts the number of pairs in a given structure. 

```
function countPairs(x) {
    let visited = new Set();
    
    function count(x) {
        if (!Array.isArray(x) || x.length !== 2) {
            return 0;
        } else if (visited.has(x)) {
            return 0;
        } else {
            visited.add(x);
            return count(x[0]) + count(x[1]) + 1;
        }
    }
    
    return count(x);
}
```

Here I use a JavaScript `Set` to keep track of unique pairs

## Exercise 18

In this exercise we are asked to write a function which checks whether a list contains any kinds of loops

```
function hasCycle(x) {
    function hasVisited(n, visited) {
        if (visited.length === 0) {
            return false;
        } else if (n === visited[0]) {
            return true;
        } else {
            return hasVisited(n, visited.slice(1));
        }
    }

    function iter(x, visited) {
        if (!x) {
            return false;
        } else if (hasVisited(x, visited)) {
            return true;
        } else {
            return iter(x.slice(1), [x].concat(visited));
        }
    }

    return iter(x, []);
}
```

## Exercise 19

In this exercise, we are asked to solve Exercise 17 again, but using a constant amount of space. For this, we can use the [Floyd's tortoise and hare algorithm](https://en.wikipedia.org/wiki/Cycle_detection#Tortoise_and_hare).

```
function hasCycle(x) {
    function run(tortoise, hare) {
        if (!tortoise || !hare || !hare.next || !hare.next.next) {
            return false;
        } else if (tortoise.data === hare.data) {
            return true;
        } else {
            return run(tortoise.next, hare.next.next);
        }
    }

    return run(x, x.next);
}

```

## Exercise 21

In this question, we are dealing with printing out a queue structure that was defined just before in the text. Specifically, it seems to be printing out some elements twice, for instance

```
const q1 = make_queue();

insert_queue(q1, "a");
[["a", null], ["a", null]]

insert_queue(q1, "b");
[["a", ["b", null]], ["b", null]]

delete_queue(q1);
[["b", null], ["b", null]]

delete_queue(q1);
[null, ["b", null]]
```

So, what's happening here? The issue really comes down to the interpreter not understanding the internals of the queue representation. The reason the last queue element appears twice is that the `rear_ptr` is also pointing into the same list the `front_ptr` points into. We can solve this by making our own print function which just prints the `front_ptr`

```
function printQueue(q) {
    console.log(front_queue(q));
}
```

## Exercise 31

In this question we are asked why when `accept_action_function` defined in `make_wire` specifies that when a new action is added to a wire, that function is immediately run. This is because we need to execute an action in order to store it in the agenda. If we didn't do this the simulation would fail!

## Exercise 32

This question asks why the agenda is a FIFO queue rather than  LIFO queue. This is because we need each action run to incorporate all previous changes. In other words, the order of execution matters!

## Exercise 34

Louis Reasoner proposes a squerer device

```
function squarer(a, b) {
	return multiplier(a, a, b);
}
```

The flaw in this idea is that the `multiplier` as already defined does not know that two of its inputs are the same. Thus, with only one input it can not solve the multiplication equation.

## Exercise 35

We are asked to complete the implementation of `squarer`

```
function squarer(a, b) {
    function processNewValue() {
        if (b.hasValue()) {
            if (b.getValue() < 0) {
                throw new Error("square less than 0 -- SQUARER " + b.getValue());
            } else {
                a.setValue(Math.sqrt(b.getValue()), me);
            }
        } else {
            b.setValue(Math.pow(a.getValue(), 2), me);
        }
    }

    function processForgetValue() {
        a.forgetValue(me);
        b.forgetValue(me);
    }

    function me(request) {
        if (request === 'I-have-a-value') {
            processNewValue();
        } else if (request === 'I-lost-my-value') {
            processForgetValue();
        } else {
            throw new Error("Unknown request -- SQUARER " + request);
        }
    }

    connect(a, me);
    connect(b, me);
    return me;
}
```

## Exercise 37

In this exercise we are asked to define "constraint" version of arithmetic operations - `cminus`, `cmul`, `cdiv`, `cv` (constant value) that allow us to define a simpler `celsius_fahrenheit_converter`

```
function celsiusFahrenheitConverter(x) {
    return cAdd(
        cMul(
            cDiv(
                cv(9),
                cv(5)
            ),
            x
        ),
        cv(32)
    );
}

function cAdd(x, y) {
    let z = makeConnector();
    adder(x, y, z);
    return z;
}

function cSub(x, y) {
    let z = makeConnector();
    adder(y, z, x);
    return z;
}

function cMul(x, y) {
    let z = makeConnector();
    multiplier(x, y, z);
    return z;
}

function cDiv(x, y) {
    let z = makeConnector();
    multiplier(y, z, x);
    return z;
}

function cv(x) {
    let y = makeConnector();
    constant(x, y);
    return y;
}
```

## Exercise 38

In this question Peter, Paul and Mary share a joint bank account that initially contains $100. Concurrently, Peter deposits $10, Paul withdraws $20 and Mary withdraws half of the money in the account. In other words, they execute the following commands

- Peter: `balance = balance + 10`
- Paul: `balance = balance - 20`
- Mary: `balance = balance - (balance / 2)`

We are asked to list the possible different values depending on the execution order we might end up with

- Peter Paul Mary        (100 + 10 - 20) / 2 = 45
- Peter Mary Paul        (100 + 10) / 2 - 20 = 35
- Paul Peter Mary        (100 - 20 + 10) / 2 = 45
- Paul Mary Peter        (100 - 20) / 2 + 10 = 50
- Mary Peter Paul        100 / 2 + 10 - 20   = 40
- Mary Paul Peter        100 / 2 - 20 + 10   = 40

This shows how important getting the order right is!!

## Exercise 3.39

Above this question a serialiser is defined, which I won't write out. We redefine the serialiser as

```
let x = 10;

const s = make_serializer();

concurrent_execute(	() => { x = s( () => x*x)();},
	s( () => { x = x * x * x;}) );
```

which of the five original possible values remain?

- 121: P2 increments `x` to 11 and then P1 sets `x` to `x * x`
- 100: P1 accesses `x` twice, then P2 sets `x` to 11, then P1 sets `x` to 100
- 101: P1 sets `x` to 100 and then P2 increments `x` to 101

## Exercise 40

Give all possible values of `x` that can result from executing

```
let x = 10;

concurrent_execute( () => { x = x * x; },
	() => { x = x * x * x; } );
```

Which of the possibilities remain if we increase use the serialised form

```
let x = 10;

const s = make_serializer();

concurrent_execute( s( () => {x = x * x; }),
	s( () => { x = x * x * x; }) );
```


Without a serialiser we can get

- 100: P1 reads two `x` of 10, P2 sets `x` to 1000, P1 sets `x` to 100
- 1000: P2 reads three `x` of 10, P1 sets `x` to 100, P2 sets `x` to 1000
- 10,000: P2 reads two `x` of 10, P1 sets `x` to 100, P2 reads one `x` of 10, P2 sets x to 10,000
- 100,000: P2 reads one `x` of 10, P1 sets `x` to 100, P2 gets two `x` of 100, P2 sets `x` to 100,000
- 1,000,000: P1 sets `x` to 100 then P2 sets `x` to 1,000,000

With a serialiser the only option left is 1,000,000

## Exercise 41

Ben Bitdiddle rewrites the bank account code as follows

```
function make_account(balance) {
	function withdraw(amount) {
		if (balance > amount) {
			balance = balance - amount;
			return balance;
		} else {
			return "Insufficient funds";
		}
	}
	function deposit(amount) {
		balance = balance + amount;
		return balance;
	}
	const protect = make_serializer();
	function dispatch(m) {
		return m === "withdraw"
				? protect(withdraw)
				: m === "deposit"
				? protect(deposit)
				: m === "balance"
				? protect( () => balance)(undefined)
				: error(m, "unknown request -- make_account");
	}
	return dispatch;
}
```

Does Ben Bitdiddle need to worry about this? It depends whether the reading and writing operations are atomic!

If they are atomic, then no additional protections are needed. This is because reading the balance has no impact on the value of the balance.

If they are not atomic, then this protection is needed, as then a read might happen half way through a write, and an invalid read operation occurs.

## Exercise 42

Ben Bitdiddle then wants to look at performance and rewrites the account code as follows

```
function make_account(balance) {
	function withdraw(amount) {
		if (balance > amount) {
			balance = balance - amount;
			return balance;
		} else {
			return "Insufficient funds";
		}
	}
	
	function deposit(amount) {
		balance = balance + amount;
		return balance;
	}
	
	const protect = make_serializer();
	const protect_widthdraw = protect(withdraw);
	const protect_deposit = protect(deposit);
	
	function dispatch(m) {
		return m === "withdraw"
				? protect_withdraw
				: m === "deposit"
				? protect_deposit
				: m === "balance"
				? balance
				: error(m, "unknown request -- make_account");
	}
	
	return dispatch;
}
```

Is it safe for Ben Bitdiddle to do this? Yes! This version makes no concurrent difference, although it saves time remaking the `protect` functions every time

## Exercise 44

Ben Bitdiddle is back on the scene - he is not claiming that if multiple people are transferring money between multiple accounts then we only need the following

```
function transfer(from_account, to_account, amount) {
	from_account("withdraw")(amount);
	to_account("deposit")(amount);
}
```

Louis Reasoner think there's a problem here and we need something more sophisticated. However, Louis is not right. The difference between the transfer problem and the exchange problem is that in the transfer problem no new variable (`difference` in exchange) is introduced. It is this internal variable which gives rise to concurrency issues!

## Exercise 45

Louis Reasoner is back again: this time he thinks the whole system is too complex. He proposes the following system

```
function make_account_and_serlializer(balance) {
	function withdraw(amount) {
		if (balance > amount) {
			balance = balance - amount;
			return balance;
		} else {
			return "Insufficient funds";
		}
	}
	
	function deposit(amount) {
		balance = balance + amount;
		return balance;
	}
	
	const balance_serializer = make_serializer();
	
	return m => m === "withdraw"
		? balance_serializer(withdraw)
		: m === "deposit"
		? balance_serializer(deposit)
		: m === "balance"
		? balance
		: m === "serializer"
		? balance_serialier
		: error(m, "unknown request -- make_account");
}
```

Then deposits are handled as with the original `make_account`:

```
function deposit(account, amount) {
	account("deposit")(amount);
}
```

The problem with Louis Reasoner's idea is everything shares the same serializer - thus, this system easily leads to deadlock as soon as anything concurrent actually tries to happen

## Exercise 47

In this exercise we are asked to define semaphores in terms of mutexes and in terms of atomic `test_and_set` operations


In terms of mutex:

```
function makeSemaphore(n) {
    let mutex = makeMutex();
    let count = 0;

    function acquire() {
        mutex.acquire();
        if (count === n) {
            mutex.release();
            acquire();
        } else {
            count++;
            mutex.release();
        }
    }

    function release() {
        mutex.acquire();
        count--;
        mutex.release();
    }

    function theSemaphore(m) {
        if (m === 'acquire') {
            acquire();
        } else if (m === 'release') {
            release();
        }
    }

    return theSemaphore;
}
```

In terms of `test_and_set`:

```
function makeSemaphore(n) {
    let cell = [false];
    let count = 0;

    function acquire() {
        acquireCell();
        if (count === n) {
            clearCell();
            acquire();
        } else {
            count++;
            clearCell();
        }
    }

    function release() {
        acquireCell();
        count--;
        clearCell();
    }

    function acquireCell() {
        if (testAndSet(cell)) {
            acquireCell();
        }
    }

    function clearCell() {
        cell[0] = false;
    }

    function theSemaphore(m) {
        if (m === 'acquire') {
            acquire();
        } else if (m === 'release') {
            release();
        }
    }

    return theSemaphore;
}
```

## Exercise 48

The deadlock-avoidance method proposed just before this question is asked avoids deadlock since a process wanting to acquire a2 must acquire a1 first, but two processes can not acquire a1 at the same time, which avoids deadlock

```
function makeAccount(number, balance) {
    function withdraw(amount) {
        if (balance >= amount) {
            balance -= amount;
            return balance;
        } else {
            return "Insufficient funds";
        }
    }

    function deposit(amount) {
        balance += amount;
        return balance;
    }

    let protected = makeSerializer();

    function dispatch(m) {
        if (m === 'withdraw') return protected(withdraw);
        if (m === 'deposit') return protected(deposit);
        if (m === 'serializer') return protected;
        if (m === 'balance') return balance;
        if (m === 'number') return number;
        throw new Error("Unknown request -- MAKE-ACCOUNT " + m);
    }

    return dispatch;
}

function serializedExchange(account1, account2) {
    let serializer1 = account1('serializer');
    let serializer2 = account2('serializer');
    let number1 = account1('number');
    let number2 = account2('number');

    if (number1 < number2) {
        return serializer2(serializer1(exchange))(account1, account2);
    } else {
        return serializer1(serializer2(exchange))(account1, account2);
    }
}

```

## Exercise 51

What does the interpreter print in response to the following

```
let x = stream_map(display, stream_enumerate_interval(0, 10));

stream_ref(x, 5);

stream_ref(x, 7);
```

It will print

```
1
2
3
4
5
6
7
```

## Exercise 52

Consider the following statements

```
let sum = 0;

function accum(x) {
	sum = x + sum;
	return sum;
}

const seq = stream_map(accum, stream_enumerate_interval(1, 20));

const y = stream_filter(is_even, seq);

const z = stream_filter(x => x % 5 === 0, seq);

stream_ref(y, 7);

display_stream(z);
```

We are then asked for the changing value of `sum`, which is 

```
1
6
10
136
210
```

And the displayed values are

```
10
15
45
55
105
120
190
210
```

## Exercise 53

We're asked to describe the elements of the stream

```
const s = pair(1, () => add_streams(s, s));
```

Which would be 

```
1, 2, 4, 8, 16, 32...
```

## Exercise 56

In this problem we want to efficiently list all integers with no prime factors other than 2, 3 or 5. We define a function `merge`

```
function merge(s1, s2) {
	if (is_null(s1)) {
		return s2;
	} else if (is_null(s2)) {
		return s1;
	} else {
		const s1head = head(s1);
		const s2head = head(s2);
		return s1head < s2head
			? pair(s1head, () => merge(stream_tail(s1), s2))
			: s1head > s2head
			? pair(s2head, () => merge(s1, stream_tail(s2)))
			: pair(s1head, () => merge(stream_tail(s1), stream_tail(s2)));
	}
}
```

Then we can construct the stream with

```
const S = pair(1, () => merge( scale_stream(S, 2), merge(scale_stream(S, 3), scale_stream(S, 5) ));
```

## Exercise 57

This function asks us how many additions are needed to calculate the nth Fibonacci number for the unoptimised and optimised methods discussed in the book

For the unoptimised method it is 

```
f(n) = n + 1
```

And for the unoptimised it is

```
f(n) = f(n - 1) + f(n - 2) + 1
```

So, quite a big difference!

## Exercise 58

Give an interpretation of the following function

```
function expand(num, den, radix) {
	return pair(math_trunc((num * radix) / den),
		() => expand((num * radix) % den, den, radix));
}
```

It returns a stream composed of digits of a fraction. For example,

```
expand(1, 7, 10);

1, 4, 2, 8, 5, 7, ...

expand(3, 8, 10);

3, 7, 5, 0, 0, 0...
```

## Exercise 63

Louis Reasoner is back and not happy with the performance of the stream produced by `sqrt_stream` function and tries to optimise it

```
function sqrt_stream_optimised(x) {
	return pair(1, memo(() => stream_map(guess => 
		sqrt_improve(guess, x),
		sqrt_stream(x))));
}
```

Alyssa P. Hacker proposes 

```
function sqrt_stream_optimised_2(x) {
	const guesses = pair(1, memo(() => stream_map(guess => 
		sqrt_improve(guess, x),
		guesses)));
		
	return guesses;
}
```

Alyssa claims hers is much more efficient - she is correct, every time Louis' program is called it produces a new stream, so the memoization is not being properly made use of

## Exercise 68

Louis Reasoner once again thinks an implementation is unnecessarily complicated and proposes this solution to pairs

```
function pairs(s, t) {
	return interleave(stream_map(x => list(head(s), x),
		t),
		pair(stream_tail(s), stream_tail(t)));
}
```

If we call `pairs(integers, integers)` with this implementation we get an infinite recursion, so it doesn't work!

## Exercise 77

We have the following implementation of `integral` 

```
function integral(integrand, initial_value, dt) {
	return pair(initial_value
		is_null(integrand)
		? null
		: integral(stream_tail(integrand),
			dt * head(integrand) + initial_value,
			dt));
}
```

Which we want to improve so that it expects `integrand` as a delayed argument, like so

```
function integral(delayedIntegrand, initialValue, dt) {
    let integrand = force(delayedIntegrand);
    let nextValue = initialValue;

    if (streamIsNull(integrand)) {
        return theEmptyStream;
    } else {
        nextValue = nextValue + dt * streamCar(integrand);
        return consStream(initialValue, () => integral(delay(delayedIntegrand), nextValue, dt));
    }
}
```