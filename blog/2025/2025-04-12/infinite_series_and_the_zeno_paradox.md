## Zeno of Elea and the Race Course

Zeno of Elea was a pre-Socratic philosopher (whom I [wrote about in a much much earlier blog](/blog/2022-02-27/pre-socratics-part3)) of which little is known. Nevertheless, his paradoxes have lived on in history. Zeno was a student of Parmenides, and they held that all experiences in reality are mere illusions. The basic idea was that there is no possibility of change, and to demonstrate this Zeno came up with a bunch of paradoxes of motion. The purpose of these was to create a set of logical traps for people who maintained that they could trust their senses. 

The most famous of these paradoxes is the story of Achilles and the tortoise. In this paradox Achilles and a tortoise have a race, but the tortoise is given a head-start. When Achilles starts running, he catches up to where the tortoise was. But, in that time, the tortoise has moved forward. So Achilles catches up again, but the tortoise has already moved forward, and so on. In this way, Achilles can never take the lead.

While that's the most famous one, we'll actually be taking a look at the dichotomy paradox in this blog and why it is wrong. In this blog, a runner needs to run a race course. First, they have to run half the race course, but first half that distance, but first half *that* distance, but first half _**that**_ distance, but first... 

<hr>

## Infinite Series

There are many examples of infinite series. Consider 

\\[1 + 2 + 3 + 4 + \cdots\\]

\\[1^2 + 2^2 + 3^2 + 4^2 + \cdots\\]

\\[\frac{1}{2} + \frac{2}{2^2} + \frac{3}{2^3} + \frac{4}{2^4} + \cdots\\]

\\[x - \frac{x^2}{2} + \frac{x^3}{3} - \frac{x^4}{4} + \cdots\\]

In all of these the \\(+ \cdots\\) indicates that the series continues forever. We want to give at least enough terms for the pattern to become obvious. 

### Geometric Series

One particularly interesting infinite series is the geometric series. Consider the following sequences

\\[2,4,8,16,32,\dots\\]

\\[1, \frac{2}{3}, \frac{4}{9}, \frac{8}{27}, \frac{16}{81}, \dots\\]

\\[a, ar, ar^2, ar^3, \dots\\]

Suppose the first example was describing the population of a bacteria at different time steps. In this scheme the population will clearly continue to increase forever and ever (mathematically, at least, we'll ignore biology for now!). Consider on the other hand the second sequence: what if it represented the height a ball bounced. The ball might fall \\(1m\\), then rise \\(2/3m\\), then \\(4/9m\\) and so on, each time bouncing \\(2/3\\) the height it fell from (due to energy lost from inelastic collisions and friction and so on). 

A natural question for this ball is how far it travels in total distance. We can find that by the following expression

\\[1 + 2 \cdot \frac{2}{3} + 2 \cdot \frac{4}{9} + 2 \cdot \frac{8}{27} + \cdots = 1 + 2 \left( \frac{2}{3} + \frac{4}{9} + \frac{8}{27} + \cdots \right)\\]

Now we have the infinite series, namely

\\[\frac{2}{3} + \frac{4}{9} + \frac{8}{27} + \cdots\\]

And we want to find its sum.

The sum of a geometric series can be written by

\\[S_n = a + ar + ar^2 + \cdots + ar^{n-1}\\]

Which we can arbitrarily multiply through by \\(r\\)

\\[S_n r = ar + ar^2 + \cdots + ar^n\\]

Subtracting then following some simple algebra 

\\[S_n - S_nr = a - ar^n\\]
\\[S_n (1-r) = a(1 - r^n)\\]
\\[S_n = \frac{a(1-r^n)}{1-r}\\]

Which gives us our first formula for sums of geometric series. Clearly though, if \\(\vert r \vert > 1\\) then the sum will tend to infinity as each term will grow larger and larger. However, if \\(\vert r \vert < 1\\) then \\(r^n\\) approaches 0 as \\(n\\) becomes very large. In such a scheme, we can actually derive another simpler sum 

\\[S_n = a + ar + ar^2 + \cdots + ar^{n-1} + \cdots\\]
\\[S_n r = ar + ar^2 + \cdots + ar^n + \cdots\\]
\\[S_n - S_n r = a\\]
\\[S_n = \frac{a}{1-r}\\]

So if we return to our ball, we can see that \\(r = 2/3\\), so we can use the infinite sum and it evaluates to

\\[S_n = \frac{2/3}{1 - 2/3} = 2\\]

So the total distance is \\(1 + 2 \cdot 2 = 5\\). Your physical intuition might be that in such a scheme, the ball would have a smaller number of bounces, which is certainly true. However, the contribution is dominated by the first few terms.

The formula for a geometric series is again \\(ar^n\\), so let's make a table to look at the height of each rebound

<table>
    <tr>
        <td>\(n\)</td>
        <td>\(ar^n\)</td>
    </tr>
    <tr>
        <td>0</td>
        <td>\(0.6666\dots\)</td>
    </tr>
    <tr>
        <td>1</td>
        <td>\(0.444\dots\)</td>
    </tr>
    <tr>
        <td>2</td>
        <td>\(\approx 0.296296\)</td>
    </tr>
    <tr>
        <td>5</td>
        <td>\(\approx 0.08779\)</td>
    </tr>
    <tr>
        <td>10</td>
        <td>\(\approx 0.01156\)</td>
    </tr>
    <tr>
        <td>15</td>
        <td>\(\approx 0.00152\)</td>
    </tr>
    <tr>
        <td>20</td>
        <td>\(\approx 0.0002\)</td>
    </tr>
    <tr>
        <td>100</td>
        <td>\(\approx 1.6398 \times 10^{-18}\)</td>
    </tr>
</table>

In other words, while a physical system won't make an infinite number of bounces, this is still a pretty decent approximation due to the fact that after just a few bounces the contributions are basically none. Ironically, for such a bouncing ball system, it's actually easier to approximate it as an infinite series than a finite series.

Let's look at another example. Suppose you have a water purification process which can remove one-*n*th of the impurity removed in the preceding stage.

If \\(n=2\\), the first stage will remove \\(1/2\\), the second stage remove \\(1/4\\), the third stage \\(1/8\\) and so on. So we remove 

\\[\frac{1}{2}, \frac{1}{4}, \frac{1}{8}, \dots\\]

So if we use the infinite series formula

\\[S = \frac{1/2}{1 - 1/2} = 1\\]

But remember, these are *fractions removed*, so in other words, this is \\(100\%\\) of the impurity.

Consider now if \\(n=3\\). Then we remove

\\[\frac{1}{3}, \frac{1}{9}, \frac{1}{27}, \dots\\]

So 

\\[S = \frac{1/3}{1 - 1/3} = \frac{1}{2}\\]

So we can remove, at most, \\(50\%\\) of the impurity!



## Zeno's "Paradox"

So, let's return to the paradox Zeno presents. Our runner first runs \\(1/2\\) the track, then runs \\(1/4\\) of the track, then runs \\(1/8\\) of the track...

We've already seen this sequence before! Indeed, the water purification problem from earlier had the exact same sequence for us. Just as a reminder

\\[S = \frac{1/2}{1 - 1/2} = 1\\]

And so the whole track can be covered.

So, what's the trick? Zeno's mistake (or rhetorical trick) was to equate the idea of "infinitely many steps" with the idea of "impossible to complete". Our intuition is that an infinite number of steps for some process *will* never complete, but as we've seen in quite a number of examples, that's not true. 

The next place to go from here to even further deepen this intuition that an infinite number of something can sum to a finite quantity is calculus, which is a bit beyond the scope of this blog.

## References

Riley, K., F. (1974). *Mathematical Methods for the Physical Sciences: An Informal Treatment for Students of Physics and Engineering*. Cambridge University Press

Boas, M., L. (2005). *Mathematical Methods in the Physical Sciences (3rd ed.)*. Wiley