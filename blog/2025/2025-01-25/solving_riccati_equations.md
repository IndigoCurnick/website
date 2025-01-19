I've already published two blogs on practical applications of the Kalman filter.
In both of those blogs, I use the somewhat throw away line "simply 
solve the Riccati equations". If you are not familiar with the Riccati equations
then this line could be very confusing indeed. So I hope to clear up any 
confusion in this blog which will focus on the mathematical side of the equations
rather than on the practical implementations in Rust.

Just for reference, the two blogs I refer to are

- [Kalman Filtering Falling Objects](/blog/2025-01-04/kalman-filtering-falling-object)
- [Kalman Filtering Object with Constant Velocity](/blog/2025-01-04/kalman-filtering-constant-velocity)

So what we will do is re-solve both of them, with a bit more detail to be as 
explicit as possible. Let's start with the falling object. 

## Falling Object

As a refresher, the state is given by

\\[ \mathbf{x} = [x, \dot{x}, \ddot{x}]^T \\]

Where \\(x\\) is the position, \\(\dot{x}\\) is the velocity, and \\(\ddot{x}\\) 
is the acceleration. Here, there's the option to not track the acceleration 
since we know it to be a constant. For fun and demonstration purposes we will 
have the filter estimate the acceleration, too.

The measurements will be given by

\\[\mathbf{z} = [x^*]\\]

Where \\(x^*\\) is a position measurement.

I won't detail deriving the fundamental matricies, but as a result here they are

\\[
\Phi_k =
\begin{bmatrix}
1 & dt & \frac{1}{2} (dt)^2 \\\\
0 & 1 & dt \\\\
0 & 0 & 1
\end{bmatrix}    
\\]

\\[ \mathbf{H} = [1,0,0] \\]

\\[
\mathbf{Q}_k =
\Phi_s
\begin{bmatrix}
\frac{(dt)^5}{20} & \frac{(dt)^4}{8} & \frac{(dt)^3}{3} \\\\
\frac{(dt)^4}{8} & \frac{(dt)^3}{3} & \frac{(dt)^2}{2} \\\\
\frac{(dt)^3}{6} & \frac{(dt)^2}{2} & dt
\end{bmatrix}    
\\]

So, now we are ready to solve the Riccati equations. Start by recalling the
Kalman filter equation

\\[ \hat{x}_k = \Phi_k \hat{x}_{k-1} + G_k u_{k-1} + K_k [z_k - H\Phi_k \hat{x}_{k-1} - H G_k u_{k-1}] \\]

We do need to also derive

\\[ M_k = \Phi_k P_{k-1} \Phi^T_k + Q_k \\]

\\[ K_k = M_k H^T [H M_k H^T + R_k]^{-1} \\]

So we already have everything we need to derive \\(M_k\\) and \\(K_k\\), so we 
can do that easily enough.

Now, in this problem we have no \\(G_k\\) term as there is no control to this
process, so the Kalman filter equation just becomes

\\[ \hat{x}_k = \Phi_k \hat{x}_{k-1} + K_k [z_k - H\Phi_k \hat{x}_{k-1}] \\]

So this equation essentially comes in two parts. \\( \Phi_k \hat{x}_{k-1} \\) is
the prediction, and \\( K_k [z_k - H\Phi_k \hat{x}_{k-1}] \\) is the measurement.
So, we can solve these seperately. I use a bar notation (\\( \bar{x} \\)) to 
indicate a predicted value. So, the prediction part becomes these three equations

\\[ \bar{\ddot{x}} = \hat{\ddot{x}}\_{k-1} \\]
\\[ \bar{\dot{x}} = \hat{\dot{x}}\_{k-1} + dt \bar{\ddot{x}} \\]
\\[ \bar{x} = \hat{x}\_{k-1} + dt \bar{\dot{x}} + 0.5 (dt)^2 \bar{\ddot{x}} \\]

Now we want to deal with the measurement part. What's helpful is to define a 
residual - I use a tilde for this purpose (\\( \tilde{x} \\))

\\[ \tilde{x} = x^* - \bar{x} \\]

Now for this problem, because we only have one measurement, we also only have 
one residual.

Next, the gain matrix \\( K_k \\) is actually a column matrix with three 
rows. So we denote the value from each row with \\(K_1, K_2, K_3\\).

And so, to complete the Kalman filter equation (i.e. the Riccati equation) we
can now say

\\[ \hat{x}_k = \bar{x}_k + K_1 \tilde{x} \\]
\\[ \hat{\dot{x}}_k = \bar{\dot{x}}_k + K_2 \tilde{x} \\]
\\[ \hat{\ddot{x}}_k = \bar{\ddot{x}}_k + K_3 \tilde{x} \\]

Which gives us the new state. I use hats (\\( \hat{x} \\)) to indicate a 
filtered variable. Don't forget to update the covariance

\\[ P_k = (I - K_k H) M_k \\]

And we're done!

## Constant Velocity

We can use the same method as above, I'll go a bit quicker this time. The state 
is given by

\\[ \mathbf{x} = [x, \dot{x}]^T\ \\]

The measurements will again be given by

\\[\mathbf{z} = [x^*]\\]

The important matricies are

\\[
\Phi(t) =
\begin{bmatrix}
1 & t \\
0 & 1
\end{bmatrix}    
\\]

\\[ \mathbf{H} = [1,0] \\]

\\[
\mathbf{Q}_k = \Phi_s
\begin{bmatrix}
(dt)^3 / 3 & (dt)^2 / 2 \\\\
(dt)^2 / 2 & dt
\end{bmatrix}    
\\]

Once again we have a system without a control term so the equation we need to 
solve is just

\\[ \hat{x}\_k = \Phi_k \hat{x}\_{k-1} + K\_k [z\_k - H\Phi\_k \hat{x}\_{k-1}] \\]

Remember to use 

\\[ M_k = \Phi_k P_{k-1} \Phi^T_k + Q_k \\]

\\[K_k = M_k H^T [H M_k H^T + R_k]^{-1} \\]

So we start with the predictions

\\[ \bar{\dot{x}} = \hat{\dot{x}}_{k-1} \\]

\\[ \bar{x} = \hat{x}_{k-1} + \bar{\dot{x}} dt \\]

Now the residual

\\[ \tilde{x} = x^* - \bar{x} \\]

and finally the predictions

\\[ \hat{\dot{x}} = \bar{\dot{x}} + K_2 x^* \\]

\\[ \hat{x} = \bar{x} + K_1 x^* \\]

Don't forget to update the covariance!

## References 

Zarchan, P., Musoff, H. (2009) *Fundamentals of Kalman Filtering: A Practical Approach (3rd Ed.)*. 
American Institude of Aeronautics and Astronautics

Tziallas, G., Adam, M., Assimakis, N., Polyzos, A. (2021) *Position, Velocity and Acceleration Tracking Using Kalman Filter*.
B P International