You can find the relevant blog [here](/blog/2025-04-19/introduction-sentential-logic-2)

1)

1. \\(P(J) \wedge P(B)\\) where \\(P(X)\\) means "\\(X\\) will come to the party", \\(J\\) means "John" and \\(B\\) means "Barry"
2. \\(G(L) \vee G(B)\\) where \\(G(X)\\) means "I will go to \\(X\\)", \\(L\\) means "London" and \\(B\\) means "Bristol"
3. \\(T(R) \vee T(S) \wedge \neg (T(R) \wedge T(S))\\), where \\(T(X)\\) means "it will \\(X\\) tomorrow", \\(R\\) means "rains" and \\(S\\) means "snows" (we could alternatively represent this as \\(T(R) \text{ xor } T(S)\\) if we allow for an \\(\text{xor}\\) operator)
4. \\(P(J) \wedge P(B) \vee (\neg P(J) \wedge \neg P(B))\\) but we can also use DeMorgan's law to make this \\(P(J) \wedge P(B) \vee \neg (P(J) \vee P(B))\\). \\(P(X)\\) means "\\(X\\) will come to the party", \\(J\\) means "John" and \\(B\\) means "Barry" 
5. \\(\neg ((T(J) \wedge M(J)) \wedge (T(B) \wedge M(B)))\\) but we can also use deMorgan's law to turn this into \\(\neg (T(J) \wedge M(J)) \vee \neg (T(B) \wedge M(B))\\). \\(T(X)\\) means "\\(X\\) is tall", \\(M(X)\\) means "\\(X\\) is muscular", \\(J\\) means "John" and \\(B\\) means "Barry"

2)


<table>
    <tr>
        <td>\(P\)</td>
        <td>\(Q\)</td>
        <td>\(\neg (P \vee Q)\)</td>
        <td>\(\neg P \wedge \neg Q\)</td>
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
        <td>False</td>
        <td>False</td>
    </tr>
    <tr>
        <td>False</td>
        <td>True</td>
        <td>False</td>
        <td>False</td>
    </tr>
    <tr>
        <td>False</td>
        <td>False</td>
        <td>True</td>
        <td>True</td>
    </tr>
</table>

3)


<table>
    <tr>
        <td>\(P\)</td>
        <td>\(Q\)</td>
        <td>\(P \vee Q\)</td>
        <td>\(\neg P \vee \neg Q\)</td>
        <td>\((P \vee Q) \wedge (\neg P \vee \neg Q)\)</td>
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

4)

1. \\(P \vee Q\\)
2. \\(P\\) (if this one stumps you, try making a truth table for \\(Q \vee \neg Q\\))
3. \\(P \wedge \neg (Q \wedge R)\\)
4. \\(P \vee (\neg R \wedge Q)\\)
