This is a part 2 of a blog series, you can read the first part [here](/blog/2025-03-22/introduction-sentential-logic-1).

In this blog we're going to work more with variables, and find ways to manipulate logical expressions in pure mathematics.

## Advanced Variables

In the last blog we introduced some variables very briefly. We saw something like

- P1: All \\(M\\) are \\(A\\)
- P2: All \\(B\\) are \\(M\\)
- C: All \\(B\\) are \\(A\\)

This is okay as we started to see the overall structure of these arguments, but we'd really like to have them be symbols only. Now in this blog we won't quite yet get up to everything we need to express syllogisms in pure mathematics, we will start to express variables in a more advanced way.

We used letters to stand for concepts. For example maybe \\(d\\) for "dog", or even \\(R\\) for "it is raining". What happens when we want to reuse a concept, though? 

Consider the statements "3 is a prime number" and "5 is a prime number". We could say \\(A\\) means "3 is a prime number" and \\(B\\) means "5 is a prime number", but that gets very tedious very quickly. What if we could reuse the concept of "is a prime number"? We do that by writing variables like this: \\(P(X)\\). This says "\\(X\\) is a prime number". So we could use \\(P(3)\\) for "3 is a prime number" and \\(P(5)\\) for "5 is a prime number".

## More Logical Symbols

Now that we have variables, we can start to combine them together. Mathematicians use the following symbols.

<table>
    <tr>
        <td>Symbol</td>
        <td>Meaning</td>
    </tr>
    <tr>
        <td>\(\vee\)</td>
        <td>or</td>
    </tr>
    <tr>
        <td>\(\wedge\)</td>
        <td>and</td>
    </tr>
    <tr>
        <td>\(\neg\)</td>
        <td>not</td>
    </tr>
</table>

It should be noted that in mathematics and logic the convention is "or" (\\(\vee\\)) is always an *inclusive* or. This would make it equivalent to the legal term "and/or". There is another term called the \\(XOR\\) (which unfortunately has no standard symbol in common use), which is an "exclusive or" - this means "either x or y, but not x and y". You can always re-express or in terms of xor and xor in terms of or, if necessary, though. 

Also, you will sometimes see the \\(\sim\\) symbol used (called "tilde", pronounced "tilda") instead of \\(\neg\\) for not. I usually see philosophers use that notation, but I think by this point \\(\neg\\) is considered basically universal for not.

We can then combine variables together. For example, \\(A \vee B\\) for "\\(A \text{ or } B\\)" or \\(A \wedge \neg B\\) for "\\(A \text{ and not } B\\)".

## Truth Tables

The truth table is a useful introductory tool in the field of sentinel logic. It allows us to compute the status of a logical statement by simply considering each of the statements that comprise the logical statement. It is obvious how "and" and "not" impact statements. Let's explore this while constructing our first and most trivial truth tables

<table>
    <tr>
        <td>\(P\)</td>
        <td>\(\neg P\)</td>
    </tr>
    <tr>
        <td>True</td>
        <td>False</td>
    </tr>
    <tr>
        <td>False</td>
        <td>True</td>
    </tr>
</table>

In other words \\(\neg\\) flips a value. Now let's try "and". The way "and" works is by saying something is true only if both inputs are also true


<table>
    <tr>
        <td>\(P\)</td>
        <td>\(Q\)</td>
        <td>\(P \wedge Q\)</td>
    </tr>
    <tr>
        <td>True</td>
        <td>True</td>
        <td>True</td>
    </tr>
    <tr>
        <td>True</td>
        <td>False</td>
        <td>False</td>
    </tr>
    <tr>
        <td>False</td>
        <td>True</td>
        <td>False</td>
    </tr>
    <tr>
        <td>False</td>
        <td>False</td>
        <td>False</td>
    </tr>
</table>

Looking at these two tables we see they have the constituent statements on the left, and the logical statement itself on the right. We fill in the values true and false for all of the statements so that all combinations are present. It's very important all possible combinations are made. To check you have done it right, the total number of combinations will be 2 to the power of the number of variables. So if there is one variable we have \\(2^1=2\\) combinations, with 2 variables we have \\(2^2=4\\) combinations, with three variables we have \\(2^3=8\\) combinations and so on.

I like to list out the true/false columns for all of these statements in a systematic pattern. The leftmost column is all true till the halfway mark, and then all false to the end. The rightmost column alternates true/false. All the columns in between alternative at different frequencies to ensure all combinations are covered. If that sounds a little vague, I'll show examples with three statements later which should make the method clear.

Now the trivial examples of "and" and "not" are covered, let's think about "or" statements. Consider the statement "P or Q". Obviously if both P and Q are false, then "P or Q" is false. And, if only one is true, then the point of "or" is that the statement will still be true. But what if both P and Q are true? As we've discussed, the standard mathematical definition of "or" means this is also true.

<table>
    <tr>
        <td>\(P\)</td>
        <td>\(Q\)</td>
        <td>\(P \vee Q\)</td>
    </tr>
    <tr>
        <td>True</td>
        <td>True</td>
        <td>True</td>
    </tr>
    <tr>
        <td>True</td>
        <td>False</td>
        <td>True</td>
    </tr>
    <tr>
        <td>False</td>
        <td>True</td>
        <td>True</td>
    </tr>
    <tr>
        <td>False</td>
        <td>False</td>
        <td>False</td>
    </tr>
</table>

How can we construct an exclusive or? Exclusive or is essentially saying "P or Q, but not P and Q". As symbols that would be \\(P \vee Q \wedge \neg (P \wedge Q)\\). Let's also break it down into steps so we can see how to use truth tables to work out more complex formulas.

<table>
    <tr>
        <td>\(P\)</td>
        <td>\(Q\)</td>
        <td>\(P \vee Q\)</td>
        <td>\(\neg (P \wedge Q)\)</td>
        <td>\(P \vee Q \wedge \neg (P \wedge Q)\)</td>
    </tr>
    <tr>
        <td>True</td>
        <td>True</td>
        <td>True</td>
        <td>False</td>
        <td>False</td>
    </tr>
    <tr>
        <td>True</td>
        <td>False</td>
        <td>True</td>
        <td>True</td>
        <td>True</td>
    </tr>
    <tr>
        <td>False</td>
        <td>True</td>
        <td>True</td>
        <td>True</td>
        <td>True</td>
    </tr>
    <tr>
        <td>False</td>
        <td>False</td>
        <td>False</td>
        <td>True</td>
        <td>False</td>
    </tr>
</table>

Notice that by breaking down the complex formula you can reduce it into new variables? \\(P \vee Q\\) essentially just becomes its own variable to the next step.

## Logical Laws

Now we have symbols, we can start to construct some logical laws. Let's start with a basic law, which is so obvious you might feel it doesn't need a proof as such but nevertheless, we will do so with a truth table

This law is called the double negation law

\\[\neg \neg P \equiv P\\]

<table>
    <tr>
        <td>\(P\)</td>
        <td>\(\neg P\)</td>
        <td>\(\neg \neg P\)</td>
    </tr>
    <tr>
        <td>True</td>
        <td>False</td>
        <td>True</td>
    </tr>
    <tr>
        <td>False</td>
        <td>True</td>
        <td>False</td>
    </tr>
</table>

I won't do truth tables for all of the following logical laws, as like the double negation law they are rather straightforward

Commutative Laws:

\\[P \wedge Q \equiv Q \wedge P\\]

\\[P \vee Q \equiv Q \vee P\\]

Associative Laws:

\\[P \wedge (Q \wedge R) \equiv (P \wedge Q) \wedge P\\]

\\[P \vee (Q \vee R) \equiv (P \vee Q) \vee P\\]

Idempotent Laws:

\\[P \wedge P \equiv P\\]

\\[P \vee P \equiv P\\]

Distributive Laws:

\\[P \wedge (Q \vee R) \equiv (P \wedge Q) \vee (P \wedge R)\\]

\\[P \vee (Q \wedge R) \equiv (P \vee Q) \wedge (P \vee R)\\]

Absorbtion Laws:

\\[P \vee (P \wedge Q) \equiv P\\]

\\[P \wedge (P \vee Q) \equiv P\\]

All of these are reasonably self explanatory.

A much more interesting set of laws is the DeMorgan laws

\\[\neg (P \wedge Q) \equiv \neg P \vee \neg Q\\]

\\[\neg (P \vee Q) \equiv \neg P \wedge \neg Q\\]

Let's make a truth table for the first DeMorgan law


<table>
    <tr>
        <td>\(P\)</td>
        <td>\(Q\)</td>
        <td>\(\neg (P \wedge Q)\)</td>
        <td>\(\neg P \vee \neg Q\)</td>
    </tr>
    <tr>
        <td>True</td>
        <td>True</td>
        <td>False</td>
        <td>False</td>
    </tr>
    <tr>
        <td>True</td>
        <td>False</td>
        <td>True</td>
        <td>True</td>
    </tr>
    <tr>
        <td>False</td>
        <td>True</td>
        <td>True</td>
        <td>True</td>
    </tr>
    <tr>
        <td>False</td>
        <td>False</td>
        <td>True</td>
        <td>True</td>
    </tr>
</table>

Now that we have these laws we can transform logical statements into other forms. For example, let's try and simplify \\(\neg (P \vee \neg Q)\\).

By applying DeMorgan's law we get

\\[\neg P \wedge \neg \neg Q\\]

By applying the double negation law we get

\\[\neg P \wedge Q\\]

So

\\[\neg (P \vee \neg Q) \equiv \neg P \wedge Q\\]

## Exercises

1. Convert the following sentences from English into logical symbol form
	1. John and Barry will come to the party
	2. I will go to London or Bristol
	3. It will rain or snow tomorrow, but it can't rain and snow
	4. John and Barry will both come to the party, or neither will
	5. Neither John nor Barry are both tall and muscular
2. Make a truth table for the second DeMorgan law
3. Make a truth table for \\((P \vee Q) \wedge (\neg P \vee \neg Q)\\)
4. Find simpler expressions for the following logical statements
	1. \\(\neg (\neg P \wedge \neg Q)\\)
	2. \\((P \wedge Q) \vee (P \wedge \neg Q)\\)
	3. \\(\neg (\neg P \vee Q) \vee (P \wedge \neg R)\\)
	4. \\((P \wedge R) \vee (\neg R \wedge (P \vee Q))\\)

You can find solutions [here](/blog/2025-04-19/introduction-sentential-logic-2-solutions), but try all the problems first!