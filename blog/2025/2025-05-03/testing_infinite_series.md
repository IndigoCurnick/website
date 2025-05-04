It is necessary when confronted with an infinite series to ask whether it converges or diverges. We've already worked with some convergent series in [a previous article](/blog/2025-04-12/infinite-series-and-the-zeno-paradox). Specifically geometric series where \\(\vert r \vert < 1\\). If \\(\vert r \vert > 1\\) then the series has no defined sum as it grows forever. 

Strange things happen if you operate on a divergent series, as they have no definite sum. For example 

\\[S = 1 + 2 + 4 + 8 + 16 + \cdots\\]
\\[2S = 2 + 4 + 8 + 16 + \cdots = S - 1\\]
\\[S = -1\\]

Obviously, this is false! This is essentially like those proofs where you can prove any number is equal to any other number by a sneaky division by 0. 

Below I list a number of different tests for convergence or divergence.

## Test by Inspection

The first and easiest test is simple inspection. You can **never** prove that a series is convergent by inspection, but it is an easy way to see that a series is *divergent*.

Consider the series 

\\[1 + 2 + 4 + 8 + 16 + \cdots\\]

This series is clearly divergent. Why? Because the terms do not tend to 0. So the test is a series diverges if

\\[\lim_{n \rightarrow \infty} a_n \neq 0\\]

otherwise if 

\\[\lim_{n \rightarrow \infty} a_n = 0\\]

we need to test further to see if it is convergent (there are many series where \\(\lim_{n \rightarrow \infty} a_n = 0\\) are divergent!).

We can prove this easily by noticing that

\\[a_n = S_n - S_{n-1}\\]
\\[\lim_{n \rightarrow \infty} a_n = \lim_{n \rightarrow \infty} S_n - \lim_{n \rightarrow \infty} S_{n-1}\\]

If we assume that the series converges, then \\(\lim_{n \rightarrow \infty} S_n = \lim_{n \rightarrow \infty} S_{n-1} = L\\), so

\\[\lim_{n \rightarrow \infty} a_n = L - L = 0\\]

Thus if \\(\lim_{n \rightarrow \infty} a_n \neq 0\\) then the series can not converge.

Let's look at some examples 

1. \\(\frac{1}{2} - \frac{4}{5} + \frac{9}{10} - \frac{16}{17} + \frac{25}{26} - \frac{36}{37} + \cdots\\)

In this series we can see that \\(\lim_{n \rightarrow \infty} a_n = 1\\), so this series is divergent. While the alternating plus and minus might throw you, just focus on the *values*

2. \\(\sqrt{2} + \frac{\sqrt{3}}{2} + \frac{\sqrt{4}}{3} + \frac{\sqrt{5}}{4} + \frac{\sqrt{6}}{5} + \cdots\\)

In this series, the values are continually getting smaller and approaching 0. Therefore, we would need to test this series more to determine if it converges or diverges. Again, remember, just because \\(\lim_{n \rightarrow \infty} a_n = 0\\) does not mean a series is definitely converging!

3. \\(\sum^\infty_{n = 1} \frac{n + 3}{n^2 + 10n}\\)

Clearly, as \\(n \rightarrow \infty\\) \\(n^2 + 10n\\) grows faster than \\(n + 3\\), therefore \\(\lim_{n \rightarrow \infty} a_n = 0\\), so we need to test this series further to determine whether it converges or diverges.

4. \\(\sum^\infty_{n=1} \frac{(-1)^n n^2}{(n+1)^2}\\)

First, let's ignore the \\((-1)^n\\). All that could do is flip between a positive and negative value, but we don't care here. We just want to see which way it tends. Another way to write that fraction would be 

\\[\frac{n^2}{n^2 + 2n + 1}\\]

Let's factor in a very artificial way

\\[\frac{n^2}{n^2(1 + \frac{2}{n} + \frac{1}{n^2})} = \frac{1}{1 + \frac{2}{n} + \frac{1}{n^2}}\\]

So the limit is

\\[\lim_{n \rightarrow \infty} = \frac{1}{1 + \frac{2}{n} + \frac{1}{n^2}} = \frac{1}{1} = 1\\]

Because \\(\lim_{n \rightarrow \infty} \frac{2}{n} = \lim_{n \rightarrow \infty} \frac{1}{n^2} = 0\\)

So this series is divergent.

## Tests for Positive Series

We'll cover here five tests which can be used to aid in determining whether a series converges or diverges. Note that all of these tests are useful for when all of the terms of a series are positive. If any of the terms are negative, we can construct a related, alternative series where all of the terms are the absolute value i.e. they are all positive. If this alternative series converges, then the original series is *absolutely convergent*. 

It can be proven (though we will not do it in this article) that any series that is absolutely convergent is also convergent, when putting back the original minus signs. The actual value of the sum will be different though. 

### Test by Comparison

This test happens in two parts. 

Part 1

Let 

\\[m_1 + m_2 + m_3 + m_4 + \cdots\\]

be a series of positive terms which you already know converges. Then if we are testing some series

\\[a_1 + a_2 + a_3 + a_4 + \cdots\\]

is absolutely convergent if \\(\vert a_n \vert \leq m_n\\) for all \\(n\\) after some point in the series (could be the first term, could be the millionth term, or any other).

Part 2

Let 

\\[d_1 + d_2 + d_3 + d_4 + \cdots\\]

be a series of positive terms which you know diverges. Then the series 

\\[a_1 + a_2 + a_3 + a_4 + \cdots\\]

diverges if \\(\vert a_n \vert \geq d_n\\) for all \\(n\\) from some point on (again, could be the first term or the millionth or any other).

However, the corollaries of these, i.e. \\(\vert a_n \vert \geq m_n\\) and \\(\vert a_n \vert \leq d_n\\) do not tell us anything at all. For example if \\(\vert a_n \vert \geq m_n\\) then it could be convergent or divergent, we'd need to do more testing.

Let see this test in action. Let's consider the series

\\[S_n = \sum^{\infty}_{n = 1} \frac{1}{n}\\]

We can actually compare this with the series

\\[P_n = 1 + \frac{1}{2} + \frac{1}{2} + \frac{1}{2} + \cdots\\]

Which is clearly divergent, by cleverly rewriting it as

\\[P_n = 1 + \frac{1}{2} + \left( \frac{1}{4} + \frac{1}{4} \right) + \left( \frac{1}{8} + \frac{1}{8} + \frac{1}{8} + \frac{1}{8} \right) + \cdots\\]

Now 

\\[S_n = \sum^{\infty}_{n = 1} \frac{1}{n} = 1 + \frac{1}{2} + \frac{1}{3} + \frac{1}{4} + \cdots\\]

If we find the sum of the first four terms of \\(S_n\\) we can see that

\\[S_4 = S_{s^2} = 1 + \frac{1}{2} + \left( \frac{1}{3} + \frac{1}{4} \right)\\]

But this is bigger than \\(P_4\\) since

\\[1 + \frac{1}{2} + \left( \frac{1}{3} + \frac{1}{4} \right) > 1 + \frac{1}{2} + \left( \frac{1}{4} + \frac{1}{4} \right)\\]

So

\\[S_{2^2} > 1 + \frac{1}{2} + \frac{1}{2} = 1 + \frac{2}{2}\\]

If we do the first eight terms then

\\[S_8 = 1 + \frac{1}{2} + \left( \frac{1}{3} + \frac{1}{4} \right) + \left( \frac{1}{5} + \frac{1}{6} + \frac{1}{7} + \frac{1}{8} \right)\\]

\\[S_{2^3} > 1 + \frac{1}{2} + \left( \frac{1}{4} + \frac{1}{4} \right) + \left( \frac{1}{8} + \frac{1}{8} + \frac{1}{8} + \frac{1}{8} \right)\\]

\\[S_{2^3} > 1 + \frac{3}{2}\\]

Indeed, if you do this for more and more terms, you would find that 

\\[S_{2^n} > 1 + \frac{n}{2}\\]

Therefore, every term in \\(S_n > P_n\\), so since \\(P_n\\) diverges, then so does \\(S_n\\).

### Test by Integral 

We can use this test when the terms of the series are positive, but not increasing, in other words, when \\(a_{n + 1} \leq a_n\\). Remember, we can always ignore the first \\(N\\) terms, where \\(N\\) is finite. In other words, this test can still be used even if this condition does not hold till the millionth term. 

The test is that if \\(0 < a_{n+1} \leq a_n\\) for \\(n > N\\) then \\(\sum^\infty a_n\\) converges if \\(\int^\infty a_n dn\\) is finite, and diverges if the integral is infinite. 

We can see this in action with the harmonic series. Again

\\[S = 1 + \frac{1}{2} + \frac{1}{3} + \frac{1}{4} + \cdots\\]

we evaluate 

\\[\int^\infty \frac{1}{n} dn = [\ln(n)]^\infty = \infty\\]

Therefore, the series diverges.

This also demonstrates very clearly why the preliminary test can only tell us if a series diverges, but never if it converges. The harmonic series diverges but passes the preliminary test!

### Test by Ratio

The integral test can be very tricky to perform. Taking integrals of arbitrary functions is not always easy! This test can often be practically easier to perform.

In the geometric series, each term can be obtained by multiplying the one before by \\(r\\), so \\(a_{n+1} / a_n = r\\). In general, though, this factor is not some constant but dependent upon \\(n\\). We call that \\(\rho_n\\). We define this by

\\[\rho_n = \left\lvert \frac{a_{n+1}}{a_n} \right\rvert\\]
And then define 

\\[\rho = \lim_{n \rightarrow \infty} \rho_n\\]

Then the cases are

- \\(\rho < 1\\), the series converges
- \\(\rho > 1\\), the series diverges
- \\(\rho = 1\\), tells us nothing; try another test

If we try the harmonic series again

\\[1 + \frac{1}{2} + \frac{1}{3} + \cdots + \frac{1}{n} + \cdots\\]

then we can see

\\[\rho_n = \left\lvert \frac{1}{n + 1} \div \frac{1}{n} \right\rvert = \frac{n}{n+1}\\]

\\[\rho = \lim_{n \rightarrow \infty} \frac{n}{n+1} = \lim_{n \rightarrow \infty} \frac{1}{1 + \frac{1}{n}} = 1\\]

Which unfortunately tells us nothing! Thankfully, we've already seen two other tests which can tell us about this series. This is an important demonstration: these are all tests which might not always work. When assessing a series you have to be prepared to be pretty holistic.

### Test by Roots

In the roots test, we can test with

\\[r = \lim_{n \rightarrow \infty} (a_n)^{\frac{1}{n}}\\]

The cases are

- \\(r < 1\\), the series converges
- \\(r > 1\\), the series diverges
- \\(r = 1\\), tells us nothing; try another test

If we test the series 

\\[\sum^\infty_{n=1} \frac{n^3}{3^n}\\]

then \\(a_n = \frac{n^3}{3^n}\\), so

\\[\sqrt[n]{a_n} = \sqrt[n]{n^3} \frac{1}{3}\\]

\\[\lim_{n \rightarrow \infty} \sqrt[n]{n^3} \frac{1}{3} = \frac{1}{3} < 1\\]

So this series converges.

### Special Test

This test also has two parts. We can do a convergence test and a divergence test

1. If \\(\sum^\infty_{n=1} b_n\\) is a convergent series of positive terms, and \\(a_n \geq 0\\) and \\(a_n/b_n\\) tends to a finite limit, then \\(\sum^\infty_{n=1} a_n\\) converges
2. If \\(\sum^\infty_{n=1} d_n\\) is a divergent series of positive terms, and \\(a_n \geq 0\\) and \\(a_n / d_n\\) tends to a limit greater than 0, then \\(\sum^\infty_{n=1} a_n\\) diverges

Let's try this example

\\[\sum^\infty_{n=2} \frac{3^n - n^3}{n^5 - 5n^2}\\]

For this, we want to get a comparison series. Let's consider the numerator. Does \\(3^n\\) or \\(n^3\\) grow faster? We can actually compare logarithms for this: \\(\ln(3^n) = n \ln(3)\\) and \\(\ln(n^3) = 3 \ln(n)\\). So clearly, \\(3^n\\) grows faster. Now consider the denominator, which is at very large \\(n\\) essentially \\(n^5\\).

So our comparison series is 

\\[\sum^\infty_{n=2} \frac{3^n}{n^5}\\]

You can go ahead and prove this diverges by the ratio test, but I won't include the steps for that here. We can use the second part of the special test now

\\[\lim_{n \rightarrow \infty} \left( \frac{3^n - n^3}{n^5 - 2n^2} \div \frac{3^n}{n^5} \right) = \lim_{n \rightarrow \infty} \frac{1 - \frac{n^3}{3^n}}{1 - \frac{5}{n^3}} = 1\\]

which is indeed greater than 0, so this series diverges.

## References

Riley, K., F. (1974). *Mathematical Methods for the Physical Sciences: An Informal Treatment for Students of Physics and Engineering*. Cambridge University Press

Boas, M., L. (2005). *Mathematical Methods in the Physical Sciences (3rd ed.)*. Wiley