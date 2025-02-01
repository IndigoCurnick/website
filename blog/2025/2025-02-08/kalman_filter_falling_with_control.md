In [previous blogs](/blog/2025-01-04/kalman-filtering-constant-velocity), 
we looked at [objects falling under gravity](/blog/2025-01-04/kalman-filtering-falling-object). 
In those blogs we asked the Kalman filter to try and estimate the position, 
velocity and acceleration from the readings. However, we know in advance what 
gravity is doing. So, why don't we instead tell the Kalman filter that the 
object is accelerating? That way, we can get an even more accurate filter.

Note that many of the important results used in this blog can be found in the 
two above referenced blogs, so please refer to them for any missing derivations
in this blog.

We can actually use the exact same code to generate the data that we used in the  
falling blog.

Remember that the full state propagation equation is given by

\\[ \hat{x}\_k = \Phi\_k \hat{x}\_{k-1} + G\_k u\_{k-1} + K\_k [z\_k - H \Phi\_k \hat{x}\_{k-1} - H G\_k u\_{k-1}] \\]

So far, we've been ignoring \\(G\\), but this is our control term now.
So, let's construct the state-space equation for falling with a control term.

\\[
\begin{bmatrix}
\dot{x} \\\\
\ddot{x}
\end{bmatrix} =
\begin{bmatrix}
0 & 1 \\\\
0 & 0
\end{bmatrix}
\begin{bmatrix}
x \\\\
\dot{x}
\end{bmatrix}
+
\begin{bmatrix}
0 \\\\
-1
\end{bmatrix}
g
\\]

In effect, we are here saying that the acceleration is equal to \\(-g\\) in the 
\\(y\\) axis.

We've already calculated for this system before that

\\[\Phi_k =
\begin{bmatrix}
1 & t \\\\
0 & 1
\end{bmatrix}
\\]

Now, it's trivial to see that \\(u_{k-1}=g\\) and that

\\[
G = \begin{bmatrix}
0 \\\\
-1
\end{bmatrix}
\\]

But, we need \\(G_k\\), which is given by

\\[
G_k =
\int^t_0 \Phi(\tau) G d \tau =
\int^t_0
\begin{bmatrix}
1 & \tau \\\\
0 & 1
\end{bmatrix}
\begin{bmatrix}
0 \\\\
-1
\end{bmatrix}
d \tau =
\begin{bmatrix}
-0.5 t^2 \\\\
-t
\end{bmatrix}
\\]

If you think about that term, you might have been able to derive it from 
intuition alone - it is, after all, just SUVAT written out in a funny way.

At this point we can recall

\\[ M_k = \Phi_k P_{k-1} \Phi^T_k + Q_k \\]
\\[ K_k = M_k H^T [H M_k H^T + R_k]^{-1} \\]

Now, for this next part, refer back to the blog on 
[solving the Riccati equations](/blog/2025-01-25/solving-riccati-equations)
if you need a refresher. Once again recall that

\\[ \hat{x}\_k = \Phi\_k \hat{x}\_{k-1} + G\_k u\_{k-1} + K\_k [z_k - H\Phi\_k \hat{x}\_{k-1} - H G\_k u\_{k-1}] \\]

Since the control term is now present, we can't just ignore it. Again, we can think
of this in two stages. The \\(\Phi\_k \hat{x}\_{k-1} + G\_k u\_{k-1}\\) term is 
the prediction and the \\(K\_k [z_k - H\Phi\_k \hat{x}\_{k-1} - H G\_k u\_{k-1}]\\)
term is the update.

Let's begin with the prediction.

\\[
\begin{bmatrix}
\bar{x}\_k \\\\
\bar{\dot{x}}\_k
\end{bmatrix} = 
\begin{bmatrix}
1 & t \\\\
0 & 1
\end{bmatrix}
\begin{bmatrix}
\hat{x}\_{k-1} \\\\
\hat{\dot{x}}\_{k-1}
\end{bmatrix} +
\begin{bmatrix}
-0.5t^2 \\\\
-t
\end{bmatrix}
g
\\]

Which we can expand out into

\\[ \bar{x}\_k = \hat{x}\_{k-1} + \hat{\dot{x}}_{k-1} dt - 0.5g t^2 \\]
\\[ \bar{\dot{x}}_k = \hat{x}\_{k-1} - g t \\]

Now if we move onto the update step we can write 

\\[
\begin{bmatrix}
\bar{x}\_k^\prime \\\\
\bar{\dot{x}}\_k^\prime \\\\
\end{bmatrix} =
\begin{bmatrix}
{K\_1}\_k \\\\
{K\_2}\_k
\end{bmatrix}
\left[
x^*_k - \begin{bmatrix} 1 & 0 \end{bmatrix}
\begin{bmatrix}
1 & t \\\\
0 & 1
\end{bmatrix}
\begin{bmatrix}
\hat{x}\_{k-1} \\\\
\hat{\dot{x}}\_{k-1}
\end{bmatrix} -
\begin{bmatrix} 1 & 0 \end{bmatrix}
\begin{bmatrix}
-0.5t^2 \\\\
-t 
\end{bmatrix}
g
\right]
\\]

Expanding this will get us 

\\[
\bar{x}\_k^\prime = {K\_1}\_k [ x^*\_k - \hat{x}\_{k-1} - \hat{\dot{x}}\_{k-1} t + 0.5g t^2 ]
\\]

\\[
\bar{\dot{x}}\_k^\prime = {K\_2}\_k [ x^*\_k - \hat{x}\_{k-1} - \hat{\dot{x}}\_{k-1} t + 0.5g t^2 ]
\\]

The astute will notice that the residual term is the same in both cases, so here 
we will define 

\\[ \tilde{x} = x^*\_k - \hat{x}\_{k-1} - \hat{\dot{x}}\_{k-1} t + 0.5 gt^2 \\]

So putting this all together, we will find that 

\\[\hat{x}\_k = \bar{x}\_k + {K\_1}\_k \tilde{x} \\]

\\[ \hat{\dot{x}}\_k = \bar{\dot{x}}_k + {K\_2}_k \tilde{x} \\]

And this produces the following results - which look great!
Don't forget to look at the [companion repository](https://github.com/IndigoCurnick/kalman-filtering-rs)
for the full code!

<div id="position-plot" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="velocity-plot" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="position-residual" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="velocity-residual" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<script id="KalmanFilterScripts" src="/blog-assets/2025-02-08-kalman-filter-falling-control/plots.js"></script>


## References 

Zarchan, P., Musoff, H. (2009) *Fundamentals of Kalman Filtering: A Practical Approach (3rd Ed.)*. 
American Institude of Aeronautics and Astronautics

Curnick, I. (2025) *Kalman Filter: How to Solve the Riccati Equations*.
Available from [https://indigocurnick.xyz/blog/2025-01-25/solving-riccati-equations](https://indigocurnick.xyz/blog/2025-01-25/solving-riccati-equations)

Curnick, I. (2025) *Kalman Filtering Object with Constant Velocity*
Available from [https://indigocurnick.xyz/blog/2025-01-04/kalman-filtering-constant-velocity](https://indigocurnick.xyz/blog/2025-01-04/kalman-filtering-constant-velocity)

Curnick, I. (2025) *Kalman Filtering Falling Objects*
Available from [https://indigocurnick.xyz/blog/2025-01-04/kalman-filtering-falling-object](https://indigocurnick.xyz/blog/2025-01-04/kalman-filtering-falling-object)