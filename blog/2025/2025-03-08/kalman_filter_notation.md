**This article is a WIP - I'll add more notations as I find them**

Kalman filtering notation is terrible. Every author uses a different, confusing, 
notation. I am not able to standardise it, but at least I can provide some of 
the key references and explain what notation they use.

This article assumes some familiarity with the Kalman filter, as I won't detail
every single symbol.

## Welch and Bishop

Greg Welch and Gary Bishop published the very popular 
_An Introduction to the Kalman Filter_ that many people see as a first resource. 
The equations are

### Predict
\\[ \hat{x}^-\_k = A \hat{x}\_{k-1} + Bu\_{k-1} \\]
\\[ P^-\_k = AP\_{k-1}A^T + Q \\]

### Update
\\[ K_k = P^-\_k H^T (H P^-\_k H^T + R)^{-1} \\]
\\[ \hat{x}\_k = \hat{x}^-\_k + K\_k(z\_k - H \hat{x}^-\_k) \\]
\\[ P\_k = (I - K\_k H)P^-\_k \\]

## Wikipedia

Wikipedia uses a more complicated notation

### Predict 

\\[ \hat{\mathbf{x}}\_{k\mid k-1} = \mathbf{F}\_k \mathbf{x}\_{k-1\mid k-1} + \mathbf{B}\_k \mathbf{u}\_{k} \\]
\\[ \hat{\mathbf{P}}\_{k\mid k-1} = \mathbf{F}\_k \mathbf{P}\_{k-1 \mid k-1} \mathbf{F}\_k^\textsf{T} + \mathbf{Q}\_k \\]

### Update

\\[ \tilde{\mathbf{y}}\_k = \mathbf{z}\_k -\mathbf{H}\_k\hat{\mathbf{x}}\_{k\mid k-1} \\]
\\[ \mathbf{S}\_k = \mathbf{H}\_k \hat{\mathbf{P}}\_{k\mid k-1} \mathbf{H}\_k^\textsf{T} + \mathbf{R}\_k \\]
\\[ \mathbf{K}\_k = \hat{\mathbf{P}}_{k\mid k-1}\mathbf{H}_k^\textsf{T} \mathbf{S}_k^{-1} \\]
\\[ \mathbf{x}\_{k\mid k} = \hat{\mathbf{x}}\_{k\mid k-1} + \mathbf{K}\_k\tilde{\mathbf{y}}\_k \\]
\\[ \mathbf{P}\_{k|k} = \left(\mathbf{I} - \mathbf{K}\_k \mathbf{H}\_k\right) \hat{\mathbf{P}}\_{k|k-1} \\]
\\[ \tilde{\mathbf{y}}\_{k\mid k} = \mathbf{z}\_k - \mathbf{H}\_k\mathbf{x}\_{k\mid k} \\]

## Labbe

More recently, Roger Labbe's GitHub textbook
_Kalman and Bayesian Filters in Python_ has become one of the go to texts. 
He uses a very simple notation

### Predict

\\[ \mathbf{\bar{x}} = \mathbf{Fx} + \mathbf{Bu} \\]
\\[ \mathbf{\bar{P}} = \mathbf{FPF^T} + Q \\]

### Update

\\[ \mathbf{y} = \mathbf{z} - \mathbf{H \bar{x}} \\]
\\[ \mathbf{S} = \mathbf{H \bar{P}} \mathbf{H}^T + R \\]
\\[ \mathbf{K} = \mathbf{\bar{P}} \mathbf{H}^T \mathbf{S}^{-1} \\]
\\[ \mathbf{x} = \mathbf{\bar{x}} + \mathbf{Ky} \\]
\\[ \mathbf{P} = (\mathbf{I} - \mathbf{KH}) \mathbf{\bar{P}} \\]

## Zarchan and Musoff

Paul Zarchan and Howard Musoff in
_Fundamnetals of Kalman Filtering: A Practical Approach_ present an unusual 
notation. I have mixed feelings about this textbook, since it comes with great 
examples of actually using the filter and an exploration of how the filter 
behaves. But, I hate this notation because it doesn't make a clear, distinct 
separation between predictions and updates.

On the other hand, Zarchan is one of few authors who actually provides enough 
information to reproduce his Kalman filters. Not only that, but the methods he 
describes to create an extended Kalman filter seem to be the only reliable way 
to produce one.

\\[ \hat{x}\_k = \Phi\_k \hat{x}\_{k-1} + G_k u\_{k-1} + K\_k [z\_k - H \Phi\_k \hat{x}\_{k-1} - H G\_k u\_{k-1}] \\]
\\[ M\_k = \Phi\_k P\_{k-1} \Phi^T\_k + Q\_k \\]
\\[ K\_k = M\_k H^T [H M\_k H^T + R\_k]^{-1} \\]
\\[ P\_k = (I - K\_k H) M\_k \\]

## Brookner

Eli Brookner in his _Tracking and Kalman Filtering Made Easy_ presents an 
interesting notation. Again, no separation between predict and update.

\\[ X^\*\_{n+1,n} = \Phi X^\*\_{n,n} \\]
\\[ X^\*\_{n,n} = X^\*\_{n,n-1} + H\_n (Y\_n - MX^\*\_{n,n-1}) \\]
\\[ H\_n = S^\*\_{n,n-1} M^T [R\_n + MS^\*\_{n,n-1} M^T]^{-1} \\]
\\[ S^\*\_{n,n-1} = \Phi S^\*\_{n-1,n-1} \Phi^T + Q\_n \\]
\\[ S^\*\_{n-1,n-1} = (I - H\_{n-1} M) S^\*\_{n-1,n-2} \\]

## Gelb

Arthur Gelb et al in _Applied Optimal Estimation_ present another system 
for notation. Again, no separation between predict and update.

\\[ \hat{\underline{x}}\_k(-) = \Phi\_{k-1} \hat{\underline{x}}\_{k-1}(+) \\]
\\[ \hat{\underline{x}}\_k(+) = \hat{\underline{x}}\_k(-) + K_k[Z\_k -H\_k ] \hat{\underline{x}}\_k(-) \\]
\\[ K\_k = P\_k(-) H\_k^T [H\_k P\_k(-) H\_k^T + R\_k]^{-1} \\]
\\[ P\_k(+) = \Phi\_{k-1} P\_{k-1}(+) \Phi^T\_{k-1} + Q\_{k-1} \\]
\\[ P\_k(-) = (I - K\_k H\_k) P\_k(-) \\]

## Brown and Hwang

Robert Grover Brown and Patrick Y.C. Hwang in
_Introduction to Random Signals and Applied Kalman Filtering_ also don't make a 
distinction between predict and update.

\\[ \hat{\mathbf{x}}^-\_{k+1} = \phi\_k \hat{\mathbf{x}}\_k \\]
\\[ \hat{\mathbf{x}}\_k = \hat{\mathbf{x}}^-\_k + \mathbf{K}\_k [\mathbf{z}\_k - \mathbf{H}\_k \hat{\mathbf{x}}\_k] \\]
\\[ \mathbf{K}\_k = \mathbf{P}^-\_k \mathbf{H}^T\_k [\mathbf{H}\_k \mathbf{P}^-\_k \mathbf{H}\_k^T + \mathbf{R}\_k]^{-1} \\]
\\[ \mathbf{P}^-\_{k+1} = \phi \mathbf{P}\_k \phi^T\_k + \mathbf{Q}\_k \\]
\\[ \mathbf{P}\_k = (\mathbf{I} - \mathbf{K}\_k \mathbf{H}\_k) \mathbf{P}^-\_k \\]

## My Own Notation 

In the tradition of other Kalman filter authors, I also make my own notation. I
mostly use inspiration from Zarchan, but I introduce back in the concept of the 
distinct predict and update steps.

### Predict

\\[ \bar{x}\_k = \Phi\_k \hat{x}\_{k-1} + G_k u\_{k-1} \\]
\\[ M\_k = \Phi\_k P\_{k-1} \Phi^T\_k + Q\_k \\]

### Update

\\[ K\_k = M\_k H^T [H M\_k H^T + R\_k]^{-1} \\]
\\[ \tilde{x}\_k = z\_k - H \Phi\_k \hat{x}\_{k-1} - H G\_k u\_{k-1} \\]
\\[ \hat{x}\_k = \bar{x}\_k + K\_k \tilde{x}\_k \\]
\\[ P\_k = (I - K\_k H) M\_k \\]

## References 

Zarchan, P., Musoff, H. (2009) *Fundamentals of Kalman Filtering: A Practical Approach (3rd Ed.)*. 
American Institude of Aeronautics and Astronautics

Curnick, I. (2025) *Kalman Filter: How to Solve the Riccati Equations*.
Available from [https://indigocurnick.xyz/blog/2025-01-25/solving-riccati-equations](https://indigocurnick.xyz/blog/2025-01-25/solving-riccati-equations)

Welch, G., Bishop, G. (2006) *An Introduction to the Kalman Filter*. University of North Carolina at Chapel Hill

Wikipedia *Kalman Filter*. Available from [https://en.wikipedia.org/wiki/Kalman_filter](https://en.wikipedia.org/wiki/Kalman_filter)

Labb, R. *Kalman and Bayesian Filters in Python*. Available from [https://github.com/rlabbe/Kalman-and-Bayesian-Filters-in-Python](https://github.com/rlabbe/Kalman-and-Bayesian-Filters-in-Python)

Brookner, E. (1998) *Tracking and Kalman Filtering Made Easy*. John Wiley & Sons, Inc.

Gelb, A. (1974) *Applied Optimal Estimation*. MIT Press

Brown, R. G., Hwang, P. Y. C. (2012) *Introduction to Random Signals and Applied Kalman Filtering (4th Ed)*. John Wiley & Sons, Inc.