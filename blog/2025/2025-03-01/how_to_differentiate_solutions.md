## Differentiating Polynomials Solutions

1. \\(\frac{dy}{dx} = 4x\\)
2. \\(\frac{dy}{dx} = 9x^2 + 8x -1\\)
3. \\(\frac{dy}{dx} = -8x^3 -9x^2 - 7\\)
4. \\(\frac{dy}{dx} = 25x^4 + 4x^3\\)
5. \\(\frac{dy}{dx} - 0\\)

## Differentiation From First Principles Solutions

1)

\\[y = 3x\\]
\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} \frac{3(x + \Delta x) - 3x}{\Delta x}\\]
\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} \frac{3x + 3\Delta x - 3x}{\Delta x}\\]
\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} \frac{3 \Delta x}{\Delta x}\\]
\\[\frac{dy}{dx} = 3\\]

2)

\\[y = 2x^2 + 5x\\]
\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} \frac{2(x + \Delta x)^2 + 5(x + \Delta x) - 2x^2 - 5x}{\Delta x}\\]
\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} \frac{2(x^2 + 2x \Delta x + (\Delta x)^2) + 5x + 5\Delta x - 2x^2 - 5x}{\Delta x}\\]
\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} \frac{2x^2 + 4x \Delta x + 2(\Delta x)^2 + 5x + 5\Delta x - 2x^2 - 5x}{\Delta x}\\]
\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} \frac{4x \Delta x + 2(\Delta x)^2 + 5\Delta x}{\Delta x}\\]

Notice how we can divide through by \\(\Delta x\\)

\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} 4x + 2\Delta x + 5\\]
\\[\frac{dy}{dx} = 4x + 5\\]

3)

\\[y = x^3 - x\\]
\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} \frac{(x + \Delta x)^3 - x - \Delta x - x^3 + x}{\Delta x}\\]
\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} \frac{x^3 + 2x^2 \Delta x + x(\Delta x)^2 + x^2 \Delta x + 2x(\Delta x)^2 + (\Delta x)^3  - \Delta x - x^3}{\Delta x}\\]
\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} \frac{2x^2 \Delta x + x(\Delta x)^2 + x^2 \Delta x + 2x(\Delta x)^2 + (\Delta x)^3  - \Delta x}{\Delta x}\\]
\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} (2x^2  + x\Delta x + x^2  + 2x\Delta x + (\Delta x)^2  - 1)\\]

\\[\frac{dy}{dx} = 3x^2 - 1\\]

4)

\\[y = c\\]
\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} \frac{c - c}{\Delta}\\]
\\[\frac{dy}{dx} = 0\\]

## Finding Stationary Points with Differentiation Solutions

1. Minimum at \\(x= -1/4, y = -9/8\\)
2. Inflection at \\(x=0, y = -2\\)
3. Minimum at \\(x \approx -3.22366, y \approx -19.0593\\), maximum at \\(x \approx -0.961699, y \approx 12.0192\\), minimum at \\(x \approx 1.93536, y \approx -48.1278\\)

## Differentiation Product Rule Solutions

1. \\(\frac{dy}{dx} = x^2 (3 \cos(x) - x \sin(x))\\)
2. \\(\frac{dy}{dx} = (4x - 1) \sin(x) + x (2x - 1) \cos(x)\\)
3. \\(\frac{dy}{dx} = 5 e^x x^2 (x+3)\\)
4. \\(\frac{dy}{dx} = 6x^2 (3 \ln(x) +1)\\)
5. \\(\frac{dy}{dx} = e^x (\tan(x) + \sec^2 (x))\\)

6)

\\[y = f(x) g(x)\\]
\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} \frac{f(x + \Delta x) g(x + \Delta x) - f(x) g(x)}{\Delta x}\\]

The clever trick here is to add 0 in the form of \\(-f(x)g(x + \Delta x) + f(x) g(x + \Delta x)\\)

\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} \frac{f(x + \Delta x) g(x + \Delta x) -f(x)g(x + \Delta x) + f(x)g(x + \Delta x) - f(x) g(x)}{\Delta x}\\]
\\[\frac{dy}{dx} = \lim_{\Delta x \rightarrow 0} \left( \frac{f(x + \Delta x) - f(x)}{\Delta x} \cdot g(x + \Delta x) + f(x) \cdot \frac{g(x + \Delta x) - g(x)}{\Delta x} \right)\\]

Notice the following things

\\[\lim_{\Delta x \rightarrow 0} \frac{f(x + \Delta x) - f(x)}{\Delta x} = f^\prime(x)\\]
\\[\lim_{\Delta x \rightarrow 0} g(x + \Delta x) = g(x)\\]
\\[\lim_{\Delta x \rightarrow 0} f(x) = f(x)\\]
\\[\lim_{\Delta x \rightarrow 0} \frac{g(x + \Delta x) - g(x)}{\Delta x} = g^\prime(x)\\]

\\[\frac{dy}{dx} = f^\prime (x) g(x) + g^\prime (x) f(x)\\]

## Differentiation Quotient Rule Solutions

1. \\(\frac{dy}{dx} = \frac{x \cos(x) - \sin(x)}{x^2}\\)
2. \\(\frac{dy}{dx} = \frac{\sec^2(x) - \tan(x)}{e^x}\\)
3. \\(\frac{dy}{dx} = \frac{-2x^2 + 4x + 1}{x^2 (2x + 1)^2}\\)
4. \\(\frac{dy}{dx} = \frac{e^x (5x^2 -14x -2)}{x^3 (5x + 1)^2}\\)

5)

Define

\\[h(x) = \frac{f(x)}{g(x)}\\]

\\[\frac{d}{dx} h(x) = \lim_{\Delta x \rightarrow 0} \frac{\frac{f(x + \Delta x)}{g(x + \Delta x)} - \frac{f(x)}{g(x)}}{\Delta x}\\]

\\[\frac{d}{dx} h(x) = \lim_{\Delta x \rightarrow 0} \frac{\frac{f(x + \Delta x) g(x) - f(x)g(x + \Delta x)}{g(x + \Delta x)g(x)}}{\Delta x}\\]
\\[\frac{d}{dx} h(x) = \lim_{\Delta x \rightarrow 0} \frac{f(x + \Delta x) g(x) - f(x)g(x + \Delta x)}{g(x + \Delta x)g(x) \Delta x}\\]

\\[\frac{d}{dx} h(x) = \lim_{\Delta x \rightarrow 0} \left( \frac{g(x)(f(x + \Delta x) - f(x))}{\Delta x g(x + \Delta x)g(x)} - \frac{f(x)(g(x + \Delta x) - g(x))}{\Delta x g(x + \Delta x)g(x)} \right)\\]
\\[\frac{d}{dx} h(x) = \frac{g(x)f^\prime(x)}{(g(x))^2} - \frac{f(x)g^\prime(x)}{(g(x))^2}\\]
\\[\frac{d}{dx} h(x) = \lim_{\Delta x \rightarrow 0} \frac{f^\prime(x)g(x) - f(x)g^\prime(x)}{(g(x))^2}\\]

## Differentiation Chain Rule Solutions

1. \\(\frac{dy}{dx} = 10 \sec^2 (10x + 4)\\)
2. \\(\frac{dy}{dx} = 2 e^{x^2} x\\)
3. \\(\frac{dy}{dx} = \sin(x) e^{1 - \cos(x)}\\)
4. \\(\frac{dy}{dx} = \sin(x)(-\cos(2 + \cos(x)))\\)

## Assorted Differentiation Problems Solutions


### Stationary Point Problems Solutions

1. minimum at \\(x = \frac{1}{\sqrt{2}}, y = \frac{1}{2} (1 + \ln(2))\\)
2. minimum at \\(x \approx -1.83941, y \approx -34.8514\\)
3. maximum at \\(x = -8, y = 450\\), minimum at \\(x=2, y=-50\\)


### Gradient Problems Solutions

1. approx 13.54499...
2. approx -19.0279...
3. 195
4. 94743
5. \\((2+e)\cos(1+e) \approx -3.9552...\\)


### Position of an Object Solutions

1. \\(\dot{x} = 4t(3t^2 - 9t + 20)\\)
2. \\(\ddot{x} = 36t^2 - 72t + 80\\)
3. No, it just starts from rest at \\(t=0\\)

### Population of Bacteria Solutions

1. \\(e^{10} \approx 22026.4657...\\)
2. \\(e^{15} + 12 \approx 3269029.37247...\\)

### Competing Bacteria Solution

1. \\(b\\) has the first peak
2. \\(b\\) dies out first, \\(a\\) dies out last
3. \\(a\\) starts with 2, \\(b\\) starts with 3