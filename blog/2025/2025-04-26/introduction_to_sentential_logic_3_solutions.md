These are the solutions for [this blog](/blog/2025-04-26/introduction-sentential-logic-3).
You should try all the problems yourself first!

1)

1. \\((S \vee \neg E) \rightarrow \neg H\\) where \\(S\\) stands for "the gas has an unpleasant smell", \\(E\\) stands for "the gas is explosive" and \\(H\\) stands for "the gas is hydrogen"
2. \\((F \wedge H) \rightarrow D\\) where \\(F\\) stands for "George has a fever", \\(H\\) stands for "George has a headache" and \\(D\\) stands for "George will go to the doctor"
3. \\((F \rightarrow D) \wedge (H \rightarrow D)\\) where the symbols have the same meaning as above
4. \\((x \neq 2) \rightarrow (P(x) \rightarrow O(x))\\) where \\(P(x)\\) stands for "\\(x\\) is prime" and \\(O(x)\\) stands for "\\(x\\) is odd"

2)

\\[P \leftrightarrow Q = P \rightarrow Q \wedge Q \rightarrow P\\]
\\[P \rightarrow Q \wedge Q \rightarrow P = (\neg P \vee Q) \wedge (\neg Q \vee P)\\]
For this next step we make use of the distributive laws!
\\[(\neg P \vee Q) \wedge (\neg Q \vee P) = (\neg P \wedge \neg Q) \vee (\neg P \wedge P) \vee (Q \wedge \neg Q) \vee (Q \wedge P)\\]
Finally we can cancel all of the contradictions
\\[(\neg P \wedge \neg Q) \vee (\neg P \wedge P) \vee (Q \wedge \neg Q) \vee (Q \wedge P) = (\neg P \wedge \neg Q) \vee (P \wedge Q)\\]

3)

\\[(P \rightarrow Q) \vee (P \rightarrow R) = (\neg P \vee Q) \vee (\neg P \vee R)\\]
\\[(\neg P \vee Q) \vee (\neg P \vee R) = (\neg P \vee \neg P) \vee (Q \vee R)\\]
\\[(\neg P \vee \neg P) \vee (Q \vee R) = \neg P \vee (Q \vee R)\\]
\\[\neg P \vee (Q \vee R) = P \rightarrow (Q \vee R)\\]

4)

1. Neither
2. Contradiction
3. Tautology
4. Tautology