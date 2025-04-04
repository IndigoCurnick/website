Something we need to do all the time in mathematics is solve polynomials. There's many methods for doing this thankfully! In this blog we'll go over methods of solving polynomials by hand, and leave numerical methods for another day.

## Factorising by Inspection

If we have a quadratic formula, we can usually factorise by inspection. This is basically a fancy way to say "we can look at it and guess the answer". Consider 

\\[x^2 + 2x - 8\\]

We can ask "what numbers add to 2 and times to -8?", and in this case a solution presents itself quite simply as

\\[(x+4)(x-2)\\]

Thus the roots are

\\[x = -4, x = 2\\]

Sometimes we can try several combinations, since expanding is so quick. Consider

\\[2x^2 + x - 2\\]

We know that when we factorise this, one of the factors will have a \\(2x\\). We also know that we either want a \\(+2, -1\\) or a \\(-2, +1\\). We might try

\\[(2x - 1)(x + 2)\\]

but if you expand this, it won't quite work. A little trial and error will lead you to

$$(2x+2)(x-1)$$

Try and solve the following using inspection

1. \\(x^2 - 3x - 10\\)
2. \\(x^2 -2x +1\\)
3. \\(2x^2 + 5x -3\\)
4. \\(6x^2 - 10x - 4\\)

## Quadratic Formula

Sometimes factorising isn't so simple. Consider the polynomial 

\\[x^2 + 5x +1 =0\\]

Which has solutions \\(x \approx -0.208712, x \approx -4.79129\\). You aren't going to be able to solve this easily at all by hand. Thankfully, there's a helpful formula (which you should memorise!)

For 

\\[ax^2 + bx + c = 0\\]
\\[x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}\\]

So from our example

\\[a = 1, b = 5, c = 1\\]
\\[x = \frac{-5 \pm \sqrt{5^2 - 4}}{2}\\]

Try the following using the quadratic formula

1. \\(2x^2 + 6x + 1 = 0\\)
2. \\(4x^2 + x - 7 = 0\\)
3. \\(x^2 - 2x - 9 = 0\\)
4. \\(7x^2 -25x + 2 = 0\\)

## Completing the Square

Another method of solving quadratics is completing the square. In this method, we arrange the quadratic into a perfect square form, rather than the more common two brackets form. (You might also notice that this is how the quadratic formula can be derived, though we won't explicitly give that proof).

The goal of completing the square is to get the quadratic in the form of 

\\[ax^2 + bx + c = 0\\]

into the form

\\[a(x+d)^2 + e = 0\\]

Where

\\[d = \frac{b}{2a}, e = c - \frac{b^2}{4a}\\]

So if we take an example of \\(x^2 + 8x + 6 = 0\\) we can see that \\(d = \frac{8}{2} = 4\\) and \\(e = 6 - \frac{64}{4} = - 10\\). Therefore, our answer is

\\[(x+4)^2 - 10\\]

Try the following out

1. \\(x^2 + 12x - 1 = 0\\)
2. \\(x^2 + 6x + 25 = 0\\)
3. \\(3x^2 - 12x + 4 = 0\\)
4. \\(3x^2 + 18x - 1 = 0\\)

## Comparing Coefficients

So far, we've been solving quadratics only. That's okay, but eventually you will need to solve higher order polynomials. There *is* a cubic formula, and a fourth order formula, but they're way too tedious to memorise. Also there is no fifth order or higher formula (which we won't prove here). A much better method is to factorise by comparing coefficients. 

Now the first trick with this kind of factorisation is you need to know one factor. There's unfortunately no special method for this or anything - you'll just need to use trial and error to find it. Sometimes looking at the polynomial might give you a hint to find a factor. 

Let's take the example of \\(x^3 + 6x^2 + 11x + 6 = 0\\). By trial and error, you'd find that \\(x = -1\\) satisfies that equation. Now the fun begins. If you think about what a factor means, we can actually write this

\\[x^3 + 6x^2 + 11x + 6 = (x+1)(ax^2 + bx + c)\\]

If we expand the RHS out we get

\\[x^3 + 6x^2 + 11x + 6 = ax^3 + (a+b)x^2 + (b+c)x^2 +c\\]

Now we can compare the coefficients

\\[a = 1\\]
\\[a + b = 6, b = 5\\]
\\[b + c = 11, c = 6\\]

So 

\\[x^3 + 6x^2 + 11x + 6 = (x+1)(x^2 + 5x + 6)\\]

At this point, since you have a quadratic, you can easily solve that with one of the methods listed so far (for your convenience, it is \\((x+2)(x+3)\\))

Try the following problems (Hint: all these polynomials have at least one solution which is an integer under 10 😉)

1. \\(x^3 -7x + 6\\)
2. \\(2x^3 -9x^2 +x + 12\\)
3. \\(x^4 - x^3 -7x^2 +x + 6\\)
4. \\(6x^5 - 17x^4 - 10x^3 + 20x^2 +4x - 3\\)


You can find the solutions [here](/blog/2025-04-02/solving-polynomials-by-hand-solutions)

