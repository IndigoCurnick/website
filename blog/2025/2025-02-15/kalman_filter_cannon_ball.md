So far, we have been using linear Kalman filters to track reasonably simple 
problems. However, more complex problems require more complex Kalman filters. 
One of the primary limitations of the Kalman filter is that it is not capable of 
handling non-linear situations very well. In this example, we will be tracking a 
cannon ball.

In this problem we will be firing a cannon ball, and tracking the projectile as 
it travels through the air. We will assume no air resistance. The measurements 
will be coming from a radar 30,480m downrange of the cannon. The radar will 
measure a distance \\(r\\) to the cannon ball, as well as an angle \\(\theta\\).
       
The relationship between \\(r\\), \\(\theta\\), the position of the radar and the 
position of the cannon ball are given by


\\[ \theta = \tan^{-1}(\frac{y\_T - y\_R}{x\_T - x\_R}) \\]

\\[ r = \sqrt{(x\_t - x\_r)^2 + (y\_T - y\_R)^2} \\]

Where \\(y\_T\\) is the vertical position of the cannon ball, \\(x\_T\\) is the 
horizontal position of the cannon ball, \\(y\_T\\) is the vertical position of 
the radar and \\(x\_R\\) is the horizontal position of the radar.


Therefore, we also have

\\[ x\_T = r \cos(\theta) + x\_R \\]

\\[ r \sin(\theta) + y\_R \\]

The state we will use will be

\\[
\mathbf{x} =
\begin{bmatrix}
x\_T \\\\
\dot{x}\_T \\\\
y\_T \\\\
\dot{y}\_T
\end{bmatrix}
\\]

And the measurements will be

\\[
\mathbf{z} =
\begin{bmatrix}
\theta^* \\\\
r^*
\end{bmatrix}
\\]

Let's go ahead and code up the system that will be simulating the cannon ball. 
Since we want the linear Kalman filter and the non-linear Kalman filter to be on 
equal footing, let's use the exact same function, and just add the functionality 
we need now. We'll start with a struct to hold the data we need.

<pre>
<code>
struct Data {
    pub time: Vec&lt;f64>,
    pub x: Vec&lt;f64>,
    pub vx: Vec&lt;f64>,
    pub y: Vec&lt;f64>,
    pub vy: Vec&lt;f64>,
    pub r_measurements: Vec&lt;f64>,
    pub theta_measurements: Vec&lt;f64>,
}
</code>
</pre>

The `Data` struct is pretty self explanatory - it contains everything we need as 
a result of the cannon ball simulation.

Now, we need to actually produce the data. The big idea here is we will be 
working out the cannon ball's position "objectively" using SUVAT and then 
converting into the measurements \(r\) and \(\theta\). We will assume the cannon 
ball's starting position is at \(x=y=0\). Remember that the radar station is 
downrange - in other words, the cannon ball is shot in the direction of the 
radar station. At some point, the cannon ball will be above the radar station.

We make the following `get_data()` function to perform the simulation.

<pre>
<code>
fn get_data() -> Data {
    let mut t = 0.0;
    let mut x = 0.0;
    let mut y = 0.0;
    let vx = INIT_VELOCITY * INIT_ANGLE.to_radians().sin();
    let mut vy = INIT_VELOCITY * INIT_ANGLE.to_radians().cos();

    let mut t_history = vec![];
    let mut x_history = vec![];
    let mut y_history = vec![];
    let mut vx_history = vec![];
    let mut vy_history = vec![];
    let mut theta_measurements = vec![];
    let mut r_measurements = vec![];

    let theta_normal = Normal::new(0.0, THETA_ERROR).unwrap();
    let r_normal = Normal::new(0.0, R_ERROR).unwrap();
    let mut rng = rand::thread_rng();

    while y >= 0.0 {
        t_history.push(t);
        x_history.push(x);
        y_history.push(y);
        vx_history.push(vx);
        vy_history.push(vy);

        theta_measurements.push(theta(x, y) + theta_normal.sample(&mut rng));
        r_measurements.push(r(x, y) + r_normal.sample(&mut rng));

        // y direction
        let tmp_vy = vy - G * TS;

        y = y + 0.5 * (vy + tmp_vy) * TS;

        vy = tmp_vy;

        // x direction
        x = x + vx * TS;

        // t
        t += TS;
    }

    return Data {
        time: t_history,
        x: x_history,
        vx: vx_history,
        y: y_history,
        vy: vy_history,
        r_measurements: r_measurements,
        theta_measurements: theta_measurements,
    };
}
</code>
</pre>

There's a few constants in there, while you can change them as much as you like, 
I'll give the values that we will use for the rest of this article.

- `INIT_VELOCITY: f64 = 915.0` (m/s)
- `INIT_ANGLE: f64 = 45.0` (degrees)
- `THETA_ERROR: f64 = 0.01` (radians)
- `R_ERROR: f64 = 30_500.0` (m)
- `TS: f64 = 0.1` (s)

Let's start now solving the equations needed to derive the Kalman filter that we 
will be using. We start with the state-space equations.

\\[
\begin{bmatrix}
\dot{x}_T \\\\
\ddot{x}_T \\\\
\dot{y}_T \\\\
\ddot{y}_T
\end{bmatrix} =
\begin{bmatrix}
0 & 1 & 0 & 0 \\\\
0 & 0 & 0 & 0 \\\\
0 & 0 & 1 & 0 \\\\
0 & 0 & 0 & 0
\end{bmatrix}
\begin{bmatrix}
x_T \\\\
\dot{x}_T \\\\
y_T \\\\
\dot{y}_T
\end{bmatrix} +
\begin{bmatrix}
0 \\\\
0 \\\\
0 \\\\
-g
\end{bmatrix} +
\begin{bmatrix}
0 \\\\
u_s \\\\
0 \\\\
u_s
\end{bmatrix}
\\]

So we know the dynamics matrix


\\[
\mathbf{F} =
\begin{bmatrix}
0 & 1 & 0 & 0 \\\\
0 & 0 & 0 & 0 \\\\
0 & 0 & 1 & 0 \\\\
0 & 0 & 0 & 0
\end{bmatrix}
\\]

Under normal circumstances we would derive the fundamental matrix like so

\\[ \mathbf{\Phi}(t) = \mathcal{L} ((\mathbf{s} \mathbf{I} - \mathbf{F})^{-1}) \\]

However, in this case we're lucky since

\\[
\mathbf{F}^2 =
\begin{bmatrix}
0 & 1 & 0 & 0 \\\\
0 & 0 & 0 & 0 \\\\
0 & 0 & 1 & 0 \\\\
0 & 0 & 0 & 0
\end{bmatrix}
\begin{bmatrix}
0 & 1 & 0 & 0 \\\\
0 & 0 & 0 & 0 \\\\
0 & 0 & 1 & 0 \\\\
0 & 0 & 0 & 0
\end{bmatrix} =
\begin{bmatrix}
0 & 0 & 0 & 0 \\\\
0 & 0 & 0 & 0 \\\\
0 & 0 & 0 & 0 \\\\
0 & 0 & 0 & 0
\end{bmatrix}
\\]

So the approximation

\\[
\mathbf{\Phi}(t) =
\mathbf{I} + \mathbf{F} + \frac{\mathbf{F}^2 t^2}{2!} +
\frac{\mathbf{F}^3 t^3}{3!} + \dots
\\]

Therefore, we can get an exact solution by

\\[
\mathbf{\Phi}(t) = \mathbf{I} + \mathbf{F} t =
\begin{bmatrix}
1 & 0 & 0 & 0 \\\\
0 & 1 & 0 & 0 \\\\
0 & 0 & 1 & 0 \\\\
0 & 0 & 0 & 1
\end{bmatrix} +
\begin{bmatrix}
0 & 1 & 0 & 0 \\\\
0 & 0 & 0 & 0 \\\\
0 & 0 & 1 & 0 \\\\
0 & 0 & 0 & 0
\end{bmatrix}
t =
\begin{bmatrix}
1 & t & 0 & 0 \\\\
0 & 1 & 0 & 0 \\\\
0 & 0 & 1 & t \\\\
0 & 0 & 0 & 1
\end{bmatrix}
\\]

The continuous process noise matrix is given by

\\[
\mathbf{Q}(t) =
\begin{bmatrix}
0 & 0 & 0 & 0 \\\\
0 & \Phi_s & 0 & 0 \\\\
0 & 0 & 0 & 0 \\\\
0 & 0 & 0 & \Phi_s
\end{bmatrix}
\\]

Where \\(\Phi_s\\) is the spectral density of white noise. As always we can 
derive the discrete process noise via

\\[\mathbf{Q}\_k = \int\_0^t \mathbf{\Phi}(\tau) \mathbf{Q} \mathbf{\Phi}^T(\tau) dt\\]

So we can simply solve like so

\\[
\mathbf{Q}\_k = \int^t\_0
\begin{bmatrix}
1 & \tau & 0 & 0 \\\\
0 & 1 & 0 & 0 \\\\
0 & 0 & 1 & \tau \\\\
0 & 0 & 0 & 1
\end{bmatrix}
\begin{bmatrix}
0 & 0 & 0 & 0 \\\\
0 & \Phi_s & 0 & 0 \\\\
0 & 0 & 0 & 0 \\\\
0 & 0 & 0 & \Phi_s
\end{bmatrix}
\begin{bmatrix}
1 & 0 & 0 & 0 \\\\
\tau & 1 & 0 & 0 \\\\
0 & 0 & 1 & 0 \\\\
0 & 0 & \tau & 1
\end{bmatrix}
d \tau
\\]

\\[
\mathbf{Q}\_k = \int^t\_0
\Phi_s
\begin{bmatrix}
\tau^2 & \tau & 0 & 0 \\\\
\tau * 1 & 0 & 0 \\\\
0 & 0 & \tau^2 & \tau \\\\
0 & 0 & \tau & 1
\end{bmatrix}
d \tau
\\]

\\[
\mathbf{Q}\_k = \Phi\_s
\begin{bmatrix}
(dt)^3 / 3 & (dt)^2 / 2 & 0 & 0 \\\\
(dt)^2 / 2 & dt & 0 & 0 \\\\
0 & 0 & (dt)^3 / 3 & (dt)^2 / 2 \\\\
0 & 0 & (dt)^2 / 2 & dt
\end{bmatrix}
\\]

Notice how \\(\mathbf{Q}\_k\\) is \\(\mathbf{Q}\_k\\) from the 
[constant velocity example](/blog/2025-01-04/kalman-filtering-constant-velocity), 
but it appears twice in the matrix? That's not a coincidence - here we have two 
independent constant velocity problems!

Now for the linearisation of the problem. Doing this is actually much easier 
than you might expect - we simply use a Jacobian matrix as \\(\mathbf{H}\\). 
The linearised measurement equation is given by

\\[
\mathbf{H} =
\begin{bmatrix}
\frac{\partial \theta}{\partial x_T} &
\frac{\partial \theta}{\partial \dot{x}_T} &
\frac{\partial \theta}{\partial y_T} &
\frac{\partial \theta}{\partial \dot{y}_T} \\\\
\frac{\partial r}{\partial x_T} &
\frac{\partial r}{\partial \dot{x}_T} &
\frac{\partial r}{\partial y_T} &
\frac{\partial r}{\partial \dot{y}_T}
\end{bmatrix}
\\]

Using the fact that

\\[ \theta = \tan^{-1} \left( \frac{y\_T - y\_R}{x\_T - x\_R} \right) \\]

We can derive

\\[ \frac{\partial \theta}{\partial \dot{x}\_T} = \frac{-(y\_T - y\_R)}{r^2} \\]

\\[ \frac{\partial \theta}{\partial \dot{x}\_T} = 0 \\]

\\[ \frac{\partial \theta}{\partial y\_T} = \frac{x\_T - x\_R}{r^2} \\]

\\[ \frac{\partial \theta}{\partial \dot{y}\_T} = 0 \\]

And using the fact that

\\[ r = \sqrt{(x\_t - x\_r)^2 + (y\_T - y\_R)^2} \\]

We can derive

\\[ \frac{\partial r}{\partial x\_T} = \frac{x\_T - x\_R}{r} \\]

\\[ \frac{\partial r}{\partial \dot{x}\_T} = 0 \\]

\\[ \frac{\partial r}{\partial y\_T} = \frac{y\_T - y\_R}{r} \\]

\\[ \frac{\partial r}{\partial \dot{y}\_T} = 0 \\]

So, \\(\mathbf{H}\\) becomes

\\[
\mathbf{H} =
\begin{bmatrix}
\frac{y_T - y\_R}{r^2} & 0 & \frac{x\_T - x\_R}{r^2} & 0 \\\\
\frac{x\_T - x\_R}{r} & 0 & \frac{y\_T - y\_R}{r} & 0
\end{bmatrix}
\\]

So, we now have everything to actually perform
the filtering by solving the Riccati equations.

\\[ \bar{\dot{x}} = \hat{\dot{x}}\_{k-1} \\]
\\[ \bar{x} = \hat{x}\_{k-1} + dt \hat{\dot{x}} \\]
\\[ \bar{\dot{y}} = \hat{\dot{y}}\_{k-1} - g dt \\]
\\[ \bar{y} = \hat{y}\_{k-1} + dt \hat{\dot{y}}\_{k-1} - 0.5 g (dt)^2 \\]

Recall that

\\[ M_k = \Phi\_k P\_{k-1} \Phi^T\_k + \mathbf{Q}\_k \\]
\\[ K\_k = M\_k H^T [H M\_k H^T + R\_k]^{-1} \\]

We also define

\\[ \tilde{x} = x^* - \bar{x} \\]
\\[ \tilde{y} = y^* - \bar{y} \\]

And so

\\[ \hat{x}\_k = \bar{x} + K\_{11} \tilde{x} + K\_{12} \tilde{y} \\]

\\[ \hat{\dot{x}}\_k = \bar{\dot{x}} + K\_{21} \tilde{x} + K\_{22} \tilde{y} \\]

\\[ \hat{y} = \bar{y} + K\_{31} \tilde{x} + K\_{32} \tilde{y} \\]

\\[ \hat{\dot{y}} = \bar{\dot{y}} + K\_{41} \tilde{x} + K\_{42} \tilde{y} \\]

Now that we have all the necessary components, we can look at some results.


<div id="full-plot-ekf" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="r-residual-ekf" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="theta-residual-ekf" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="x-residual-ekf" class="plotly-graph-div" style="height:100%; width:100%;"></div>

<div id="y-residual-ekf" class="plotly-graph-div" style="height:100%; width:100%;"></div>

We can see that the Kalman filter handles the situation extremely well - 
the errors are very impressive. What's more impressive is the residuals are 
tiny in \\(x\\) and \\(y\\) - in some instances down from \\(600m\\) to \\(10m\\)!

<script id="KalmanFilterScripts" src="/blog-assets/2025-02-15-kalman-filter-cannon-ball/plots.js"></script>

## References 

Zarchan, P., Musoff, H. (2009) *Fundamentals of Kalman Filtering: A Practical Approach (3rd Ed.)*. 
American Institude of Aeronautics and Astronautics

Curnick, I. (2025) *Kalman Filtering Object with Constant Velocity*
Available from [https://indigocurnick.xyz/blog/2025-01-04/kalman-filtering-constant-velocity](https://indigocurnick.xyz/blog/2025-01-04/kalman-filtering-constant-velocity)