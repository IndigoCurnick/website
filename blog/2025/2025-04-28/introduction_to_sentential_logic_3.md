This is part 3 of a series on sentential logic. You should read 
[part 1](/blog/2025-03-22/introduction-sentential-logic-1) and 
[part 2](/blog/2025-04-19/introduction-sentential-logic-2) first


## Tautologies and Contradictions

A tautology is a logical statement which is always true. A contradiction is a logical statement that is always false. Consider \\(P \vee \neg P\\). Let's make a truth table for it

<table>
    <tr>
        <td>\(P\)</td>
        <td>\(P \vee \neg P\)</td>
    </tr>
    <tr>
        <td>True</td>
        <td>True</td>
    </tr>
    <tr>
        <td>False</td>
        <td>True</td>
    </tr>
</table>


In other words, it does not matter what value \\(P\\) has, \\(P \vee \neg P\\) will always be true, and is thus a tautology.

Consider now \\(P \wedge \neg P\\).


<table>
    <tr>
        <td>\(P\)</td>
        <td>\(P \wedge \neg P\)</td>
    </tr>
    <tr>
        <td>True</td>
        <td>False</td>
    </tr>
    <tr>
        <td>False</td>
        <td>False</td>
    </tr>
</table>

No matter the value of \\(P\\), \\(P \wedge \neg P\\) will always be false. So, it is a contradiction.

The main value of tautologies and contradictions to us now is that they can be used to remove certain elements from formulas. The laws are as follows

Tautology laws

- \\(P \wedge \text{ (a tautology) } \equiv P\\)
- \\(P \vee \text{ (a tautology) is also a tautology}\\)
- \\(\neg \text{ (a tautology) is a contradiction}\\)

Contradiction laws

- \\(P \wedge \text{ (a contradiction) is a contradiction}\\)
- \\(P \vee \text{ (a contradiction) } \equiv P\\)
- \\(\neg \text{ (a contradiction) is a tautology}\\)

So if we have the formula \\(P \vee \neg (Q \vee \neg Q)\\) we can effectively just reduce this to \\(P\\).




## Conditional and Biconditional Arguments

One of the problems mathematicians and philosophers have (and autistic people) is that in normal every day speech people do not literally say what they mean. Think about that Pink Floyd lyric: "If you don't eat your meat, you can't have any pudding". If you think about what this literally saying, it is simply saying that if you don't eat meat then you will not get dessert. However, there is an *implication* that if you *do* eat the meat, you *will* get dessert. Despite the fact that this was not actually said: in a very literal sense there is nothing at all in the original statement to imply that eating meat leads to dessert (I wonder if any lawyers have ever tried pulling a trick based on this?).

So what's going on? In formal logic to get around these problems we introduced the concept of *iff* (that's not a typo - if with two fs). *If* in logic is like a "one way" if - "*if* you don't eat meat, *then* you can't have dessert" (with no implication whatsoever that if you *do* eat meat, that you will have dessert). *Iff* is a "two-way" if. "*Iff* you don't eat your meat, then you can't have dessert" (while explicitly saying if you *do* eat meat, you can have dessert). 

In other words, "*iff* \\(A\\) then \\(B\\)" can also mean "if \\(A\\) then \\(B\\) and if \\(B\\) then \\(A\\)". In logic we represent these with a very logical symbol

- *if* is represented by \\(\rightarrow\\)
- *iff* is represented by \\(\leftrightarrow\\) 

### The Conditional

So why do we need this? If you remember back to part 1 of this series, we started largely by analysing syllogisms, largely of the form like:

- If it is raining, then the ground will be wet
- It is raining
- The ground will be wet

So converting this into a more logical representation we have

- \\(R \rightarrow W\\)
- \\(R\\)
- \\(\therefore W\\) 

Where \\(R\\) means "it is raining" and \\(W\\) means "the ground is wet". \\(\therefore\\) is a logic symbol which means "therefore" - it indicates the conclusion of an argument. 

Let's try and analyse \\(A \rightarrow B\\) a bit more to figure out an equivalent formula for it in terms of symbols you already know.

Let's start by thinking about what it means. "If \\(A\\) then \\(B\\)". So if \\(A\\) is true, and \\(B\\) is true then \\(A \rightarrow B\\) is itself true. If \\(A\\) is true, but \\(B\\) is false, then \\(A \rightarrow B\\) is itself false (because it represents an untrue statement. \\(A \rightarrow B\\) is saying "if \\(A\\) happens, then \\(B\\) will certainly happen". So if \\(A\\) happens without \\(B\\) happening, then \\(A \rightarrow B\\) is false). 

So what about the other two combinations? Well if \\(A\\) is false and \\(B\\) is false, what should \\(A \rightarrow B\\) be? Think about our example: if it isn't raining and the ground isn't wet, then it might feel like the statement hasn't been tested so on first glance it might feel like the truth value here is indeterminate. However, the statement *has* been tested: it didn't rain, and the ground didn't become wet. So the statement is true here, even if it feels a little strange.

Finally, what if \\(A\\) is false, but \\(B\\) is true. Again, turn to our example. If it *isn't* raining but the ground is wet, that could be because of a bunch of different reasons, maybe someone used a hose. Remember - this is a "one way" if. Just because it is raining means the ground will be wet does not imply at all that the ground being wet means it's raining! So in this case, we can say that the statement is true.

Therefore, our truth table looks like

<table>
    <tr>
        <td>\(A\)</td>
        <td>\(B\)</td>
        <td>\(A \rightarrow B\)</td>
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
        <td>True</td>
    </tr>
    <tr>
        <td>False</td>
        <td>False</td>
        <td>True</td>
    </tr>
</table>

Curiously, the truth table of \\(\neg A \vee B\\) looks like


<table>
    <tr>
        <td>\(A\)</td>
        <td>\(B\)</td>
        <td>\(\neg A \vee B\)</td>
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
        <td>True</td>
    </tr>
    <tr>
        <td>False</td>
        <td>False</td>
        <td>True</td>
    </tr>
</table>

While this might not mesh very well with your every day intuition of what "if \\(A\\) then \\(B\\)" means, it is an alternative formula. It would seem weird to say "it won't rain or the ground will be wet" but it is entirely consistent with "if it rains, then the ground will be wet".

Of course, we can also use De Morgan's law to show that \\(\neg A \vee B \equiv \neg (A \wedge \neg B)\\). It would be super super unnatural to say "it is not the case that both it will rain and the ground will not be wet" but again, clearly it's all the same thing. So in summary

- \\(A \rightarrow B \equiv \neg A \vee B\\)
- \\(A \rightarrow B \equiv \neg (A \wedge \neg B)\\)

The next two ideas are sort of related: the *converse* and the *contrapositive*. And it is **very important that you do not confuse them**. 

The converse of \\(A \rightarrow B\\) is \\(B \rightarrow A\\) - essentially "running the argument in reverse" and it is **not** the same thing as \\(A \rightarrow B\\). If you think about the example again "if the ground is wet then it has rained" is a totally and utterly different argument to "if it rains, then the ground will be wet". This is despite the fact that, again, in every day conversation people do tend to speak and act like these are the same thing. 

The contrapositive ought not be confused with the converse because the contrapositive *is* equivalent to the original argument. The contrapositive of \\(A \rightarrow B\\) is \\(\neg B \rightarrow \neg A\\). So in our example this would be "if the ground isn't wet, then it hasn't rained" which is exactly equivalent to "if it rains, then the ground is wet".

Just to summarise this in a truth table

<table>
    <tr>
        <td>\(A\)</td>
        <td>\(B\)</td>
        <td>\(A \rightarrow B\)</td>
        <td>\(B \rightarrow A\)</td>
        <td>\(\neg B \rightarrow \neg A\)</td>
    </tr>
    <tr>
        <td>True</td>
        <td>True</td>
        <td>True</td>
        <td>True</td>
        <td>True</td>
    </tr>
    <tr>
        <td>True</td>
        <td>False</td>
        <td>False</td>
        <td>True</td>
        <td>False</td>
    </tr>
    <tr>
        <td>False</td>
        <td>True</td>
        <td>True</td>
        <td>False</td>
        <td>True</td>
    </tr>
    <tr>
        <td>False</td>
        <td>False</td>
        <td>True</td>
        <td>True</td>
        <td>True</td>
    </tr>
</table>

### The Biconditional

We've already seen that \\(P \rightarrow Q\\) and \\(Q \rightarrow P\\) are not the same thing? But what if we wanted both \\(P \rightarrow Q\\) and \\(Q \rightarrow P\\) to be true? That is where the biconditional comes in. Very straightforward here

\\[P \leftrightarrow Q = P \rightarrow Q \wedge Q \rightarrow P\\]

Since we know the truth table for \\(P \rightarrow Q\\) then this is very simple

<table>
    <tr>
        <td>\(P\)</td>
        <td>\(Q\)</td>
        <td>\(P \leftrightarrow Q\)</td>
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
        <td>True</td>
    </tr>
</table>

If we also use the contrapositive law which we've already seen then clearly \\(P \leftrightarrow Q = P \rightarrow Q \wedge \neg P \rightarrow \neg Q\\).

So if we look at an example of this in context, the phrase "the game will be cancelled iff it is either raining or snowing" can become \\(C \leftrightarrow (R \vee S)\\).

### Necessary and Sufficient

We can relate the ideas we've explored so far to the English terms "necessary" and "sufficient". If we look at the conditional statements first.

Consider \\(P \rightarrow Q\\). This maps cleanly to the following two ideas

- \\(P\\) is sufficient for \\(Q\\)
	- i.e. if \\(P\\) happens, that's enough to guarantee \\(Q\\)
- \\(Q\\) is necessary for \\(P\\)
	- i.e. for \\(P\\) to happen, \\(Q\\) *must* happen

\((P \leftrightarrow Q\\) is the combination of both. Now \\(P\\) is necessary and sufficient for \\(Q\\), and also \\(Q\\) is necessary and sufficient for \\(P\\)

## Exercises

Change these!

1. Convert the following sentences from English into logical symbol form
	1. If this gas either has an unpleasant smell or is not explosive, then it isn't hydrogen
	2. Having both a fever and a headache is a sufficient condition for George to go to the doctor
	3. Both having a fever and having a headache are sufficient conditions for George to go to the doctor
	4. If \\(x \neq 2\\), then a necessary condition for \\(x\\) to be prime is that \\(x\\) be odd
2. Show that \\(P \leftrightarrow Q\\) is the same as \\((P \wedge Q) \vee \neg (P \wedge \neg Q)\\)
3. Show that \\(P \rightarrow Q \vee P \rightarrow R\\) is equivalent to \\(P \rightarrow (Q \vee R)\\)
4. Which of these statements tautologies, contradictions or neither? (You might like to use truth tables, or convert them into other logical forms)
	1. \\((P \vee Q) \wedge (\neg P \vee \neg Q)\\)
	2. \\((P \vee Q) \wedge (\neg P \wedge \neg Q)\\)
	3. \\((P \vee Q) \vee (\neg P \vee \neg Q)\\)
	4. \\((P \wedge (Q \vee \neg R)) \vee (\neg P \vee R)\\)

You can find the solutions [here](/blog/2025-04-28/introduction-sentential-logic-3-solutions), 
but try all the problems first!

## References

Velleman, D. J. (2019) _How to Prove It: A Structured Approach (3rd ed)_. Cambridge University Press

Stoll, R., R. (1968) _Set Theory and Logic_ Dover Publications
