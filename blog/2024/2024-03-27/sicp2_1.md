## Exercise 2.15

In Exercise 14 we are asked to investigate that calculating resistances with the interval arithmatic package we wrote gives different results. We try 

<p>\[\frac{R_1 R_2}{R_1 + R_2}\]</p>

and

<p>\[ \frac{1}{1/R_1 + 1/R_2} \]</p>

Although these are mathematically the same equation, they produce quite different results with interval arithmatic. Here was the output of my program

```
Working with r1: Interval { upper_bound: 121.2, lower_bound: 118.8 }, r2: Interval { upper_bound: 255.0, lower_bound: 245.0 }
Using method one we get: Interval { upper_bound: 84.95327102803738, lower_bound: 77.36842105263158 }
Using method two we get: Interval { upper_bound: 82.1531100478469, lower_bound: 80.00549752611325 }
```

The question is whether one of these is "better" than the other. Like all answers in computer science the answer really depeneds on which tradeoffs you're willing to take.

## Exercise 2.16

Now we're asked why do we observe these strange effects. This is due to the [dependency problem](https://en.wikipedia.org/wiki/Interval_arithmetic#Dependency_problem). In general, it is not possible to make a totally generic interval package which is capable of being completely accurate. However, it is possible to make specific ones for specific problems.

## Exercise 2.22

In this question Louis Reasoner is struggling with writing a `square_list` function. We're asked to spot his mistakes 

```
function square_list(items) {
	function iter(things , answer) {
		return is_null(things)
			? answer
			: iter(tail(things),
				pair(square(head(things)),
					answer)); // The error is here
					// The head of the list is appended 
					// to the front of the answer
					// So, the list is in reverse
	}
	return iter(items, null);
}
```

And another attempt 

```
function square_list(items) {
	function iter(things , answer) {
		return is_null(things)
			? answer
			: iter(tail(things),
				pair(answer ,
					square(head(things)))); // Here
					// He's just appending a single value when
					// he should be appending a list
	}
	return iter(items, null);
}
```

## Exercise 2.24

In this problem we're asked to list the result of `list(1, list(2, list(3,4))`, as well as the box and pointer structure, as well as the tree diagram.

The output will just be `(1, (2, (3, 4)))`

The box and pointer diagram is 

<pre>
[*][*]--->[*][/]
    |         |
    v         v
   [1]       [*][*]--->[*][/]
              |         |
              v         v
             [2]       [*][*]--->[*][/]
                        |         |
                        v         v
                       [3]       [4]
</pre>

and the tree diagram is

<pre>
      (list 1 (list 2 (list 3 4)))
           /               \
          1          (list 2 (list 3 4)) 
                        /           \ 
                       2          (list 3 4)
                                   /      \
                                  3        4
</pre>


## Exercise 2.26

Suppose we define these two lists

```
const x = list(1, 2, 3);
const y = list(4, 5, 6);
```

What will be the output of the following?

```
append(x, y);
(1,2,3,4,5,6);
```

```
pair(x, y);
{car: x, cdr: y};
```

```
list(x,y);
((1,2,3), (4,5,6));
```


## Exercise 2.53

What is the interpreter output of the following statements?

```
list("a", "b", "c");
("a", "b", "c");
```

```
list(list("george"));
(("george"));
```

```
tail(list(list("x1", "x2"), list("y1", "y2")));
("y1", "y2");
```

```
tail(head(list(list("x1", "x2"), list("y1", "y2"))));
"x2"
```

```
member("red", list(list("red", "shoes"), list("blue", "socks")));
null;
```

```
member("red", list("red", "shoes", "blue", "socks"));
("red", "shoes", "blue", "socks");
```


## Exercise 2.55

This problem deals with ambiguous characters. We're asked what the output of the following program is

```
'"' === ""
```

The output is `false`. We are comparing the string `"` (that's a string with a single character - the speech mark) to the empty string


## Exercise 2.63

In this exercise we are given the following two functions

```
function tree_to_list_1(tree) {
	return is_null(tree)
		? null
		: append(tree_to_list_1(left_branch(tree)),
			pair(entry(tree),
				tree_to_list_1(right_branch(tree))));
}
```

and

```
function tree_to_list_2(tree) {
	function copy_to_list(tree, result_list) {
		return is_null(tree)
			? result_list
			: copy_to_list(
				left_branch(tree),
				pair(entry(tree),
					copy_to_list(right_branch(tree),
								result_list)));
	}
	return copy_to_list(tree, null);
}
```

We're asked what output they produce on some trees, and the answer is always the same. However, `tree_to_list_1` is O(nlog(n)) and `tree_to_list_2` is O(n).

## Exercise 2.64

In this problem we are given the following functions and asked how it works and what its time complexity is

```
function list_to_tree(elements) {
	return head(partial_tree(elements ,length(elements)));
}

function partial_tree(elts, n) {
	if (n === 0) {
		return pair(null,elts);
	} else {
		const left_size = math_floor((n - 1) / 2);
		const left_result = partial_tree(elts, left_size);
		const left_tree = head(left_result);
		const non_left_elts = tail(left_result);
		const right_size = n - (left_size + 1);
		const this_entry = head(non_left_elts);
		const right_result = partial_tree(tail(non_left_elts),
											right_size);
		const right_tree = head(right_result);
		const remaining_elts = tail(right_result);
		return pair(make_tree(this_entry ,
					left_tree ,
					right_tree),
				remaining_elts);
	}
}
```

This is a recursive function which splits a list in half, and finds the subtree for each half list down to the base case. It has time complexity O(n)

## Exercise 2.71

<p>In this question we have a Huffman tree for an alphabet of _n_ symbols. Where the relative frequency of the symbols is \(1,2,4,...,2^{n-1}\).</p>

In general, it takes 1 bit to encode the most frequent symbol and _n-1_ bits to encode the least. Here's the Huffman tree for _n=5_

<pre>
                  {A B C D E} 31
                   /       \
                  /         \
             {A B C D} 15   {E} 16
              /      \   
             /        \
         {A B C} 7   {D} 8
          /     \  
         /       \
     {A B} 3    {C} 4
    /       \
 {A} 1      {B} 2     
</pre>

## Exercise 2.72

In this question we're asked to consider the function we designed in 2.68 ([see the repo](https://github.com/IndigoCurnick/structure_interpretation_computer_programs)). Consider the special case of the frequencies described in 2.71 - what is the order of growth? 

<p>The order is \(O(n^2)\)</p>

## Exercise 2.76

In this exercise we are asked to consider three different styles - generic operations with explicit dispatch, data-directed and message-passing. We're asked to consider what changes need to be made to a system to add new types or operations, and which systems are most adaptable to new types or new operations.

For generic operations with explicit dispatch, when adding new types we need to add a new branch in each operation on the generic data. When adding new operations we need to create a new procedure containing the dispatch for all the types.

For data-directed style, when adding new types we need to add the corresponding operations into the registry table. When adding new operation we need to add the operation to each type package and register them in the table.

For message passing style, when adding new types we just... do. Nothing need be done. When adding new operations we need to add the operation to the implementation of each type. 

Message-passing style is most suitable for systems that frequently need new types. Explicit dispatch is best for systems that frequently need new operations.

## Exercise 2.81

Louis Reasoner becomes concerned about coercing types into themselves in the algebraic package. He proposes the following functions 

```
function javascript_number_to_javascript_number(n) {
	return n;
}

function complex_to_complex(n) {
	return n;
}
```

However, Louis is mistaken. Adding these will actually cause the coercion functions to fall into infinite loops! And, he doesn't need to be concerned - the functions as written up to this point work just fine without them

