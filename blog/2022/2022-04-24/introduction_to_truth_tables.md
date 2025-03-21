The truth table is a useful introductory tool in the field of sentinel logic. It allows us to compute the status of a logical statement by simply considering each of the statements that comprise the logical statement. It is obvious how "and" and "not" impact statements. Let's explore this while constructing our first and most trivial truth tables

<figure class="wp-block-table">
    <table>
        <tbody>
            <tr>
                <td>P</td>
                <td>Not P</td>
            </tr>
            <tr>
                <td>True</td>
                <td>False</td>
            </tr>
            <tr>
                <td>False</td>
                <td>True</td>
            </tr>
        </tbody>
    </table>
</figure>

<figure class="wp-block-table">
    <table>
        <tbody>
            <tr>
                <td>P</td>
                <td>Q</td>
                <td>P and Q</td>
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
        </tbody>
    </table>
</figure>

Looking at these two tables we see they have the constituent statements on the left, and the logical statement itself on the right. We fill in the values true and false for all of the statements so that all combinations are present. It's very important all possible combinations are made. To check you have done it right, the total number of combinations will be 2 to the power of the number of statements. So if there is one statements we have \\(2^1=2\\) combinations, with 2 statements we have \\(2^2=4\\) combinations, with three statements we have \\(2^3=8\\) combinations and so on.

I like to list out the true/false columns for all of these statements in a systematic pattern. The leftmost column is all true till the halfway mark, and then all false to the end. The rightmost column alternates true/false. All the columns in between alternative at different frequencies to ensure all combinations are covered. If that sounds a little vague, I'll show examples with three statements later which should make the method clear.

Now the trivial examples of "and" and "not" are covered, let's think about "or" statements. Consider the statement "P or Q". Obviously if both P and Q are false, then "P or Q" is false. And, if only one is true, then the point of "or" is that the statement will still be true. But what if both P and Q are true? This is a little ambiguous, and leads us to our first convention as such. There is inclusive and exclusive or. Inclusive or means "P or Q, or both" while the exclusive or means "P or Q, but not both". In standard mathematics (and in computer programming languages) or always means the inclusive or, so the truth table is as follows.

<figure class="wp-block-table">
    <table>
        <tbody>
            <tr>
                <td>P</td>
                <td>Q</td>
                <td>P or Q</td>
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
        </tbody>
    </table>
</figure>

Now you might be a little disappointed that "or" is always inclusive. Can we make a formula from "not", "and" and "inclusive or" that is the same as "exclusive or"? Of course! This is why I say using "or" to be inclusive is a convention, as we can construct exclusive or (in an alternative universe, or is always exclusive and they construct inclusive or by the same method I will demonstrate). How would we know if we find an equivalent formula? If the truth table is identical! Since this truth table is a little more complicated, I will use multiple columns to show the workings.

<figure class="wp-block-table">
    <table>
        <tbody>
            <tr>
                <td>P</td>
                <td>Q</td>
                <td>P or Q</td>
                <td>not (P and Q)</td>
                <td>(P or Q) and not (P and Q)</td>
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
        </tbody>
    </table>
</figure>

And we can see that "(P or Q) and not (P and Q)" has the behaviour we want.

Truth tables also make spotting contradictions or tautologies easy. Contradictions are statements that are always false, tautologies are statements that are always true. Let's look at examples

<figure class="wp-block-table">
    <table>
        <tbody>
            <tr>
                <td>P</td>
                <td>Q</td>
                <td>(P or Q) and (not P and not Q)</td>
            </tr>
            <tr>
                <td>True</td>
                <td>True</td>
                <td>False</td>
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
        </tbody>
    </table>
</figure>

<figure class="wp-block-table">
    <table>
        <tbody>
            <tr>
                <td>P</td>
                <td></td>
                <td>Q</td>
                <td>R</td>
                <td>(P and (Q or not R)) or (not P and R)</td>
            </tr>
            <tr>
                <td>True</td>
                <td></td>
                <td>True</td>
                <td>True</td>
                <td>True</td>
            </tr>
            <tr>
                <td>True</td>
                <td></td>
                <td>True</td>
                <td>False</td>
                <td>True</td>
            </tr>
            <tr>
                <td>True</td>
                <td></td>
                <td>False</td>
                <td>True</td>
                <td>True</td>
            </tr>
            <tr>
                <td>True</td>
                <td></td>
                <td>False</td>
                <td>False</td>
                <td>True</td>
            </tr>
            <tr>
                <td>False</td>
                <td></td>
                <td>True</td>
                <td>True</td>
                <td>True</td>
            </tr>
            <tr>
                <td>False</td>
                <td></td>
                <td>True</td>
                <td>False</td>
                <td>True</td>
            </tr>
            <tr>
                <td>False</td>
                <td></td>
                <td>False</td>
                <td>True</td>
                <td>True</td>
            </tr>
            <tr>
                <td>False</td>
                <td></td>
                <td>False</td>
                <td>False</td>
                <td>True</td>
            </tr>
        </tbody>
    </table>
</figure>

The last table not only shows a tautology but the method of systematically writing out all combinations of three variables.

I hope this gives you a good introduction into the useful world of truth tables. Try making your own logical statements and fill in the truth tables for them.