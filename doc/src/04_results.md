# Results

## Analytic vs. numerical calulcations

## Finding the optimal $\alpha$

Using the brute force Metropolis algorithm, we calculated the expected value of the local energy at different values of $\alpha$. This was also done at different dimensions and number of particles. The simulation over all these variables were done once for each core of the processor running them. In our case, this resulted in 8 runs. The mean over all runs are seen in figure \ref{fig:optimal_alpha}.

\begin{figure}[ht]%
  \centerfloat
  \captionsetup[subfigure]{labelformat=empty}
  \subfloat[]{\includegraphics[scale=.5]{assets/plots/dim_and_n/EnergyAlpha_1.0N.png}}
  \subfloat[]{\includegraphics[scale=.5]{assets/plots/dim_and_n/EnergyAlpha_10.0N.png}}\\
  \subfloat[]{\includegraphics[scale=.5]{assets/plots/dim_and_n/EnergyAlpha_100.0N.png}}
  \caption{Expected local energy (in units of $\hbar\omega_\text{ho}$) per particle, found at $N=1,10,100$ and for $\dim= 1,2,3$. The results are the means over simulations run on 8 CPU cores simultaneously. The black dashed line shows the mean minimum over all three dimensions. The system is non-interacting and the values are calculated with the brute force method.}
  \label{fig:optimal_alpha}
\end{figure}

In figure \ref{fig:optimal_alpha}, we see that the optimal value of $\alpha$ seems to be consistently on the value $0.5$, as expected. However, for $N = 100$ the mean deviates a bit from our expectation. A more telling picture appears when we plot the standard deviation over the CPU cores as a function of $\alpha$ instead of the expected local energy. This is shown in figure \ref{fig:optimal_alpha_std}. From this its much more clear that we're reaching the actual desired^[Desired because having a low standard deviation suggests that we've approached the true value.] value of $\alpha$ at $0.5$, regardless of how many number of particles we're simulating for.

\begin{figure}[ht]%
  \centerfloat
  \captionsetup[subfigure]{labelformat=empty}
  \subfloat[]{\includegraphics[scale=.5]{assets/plots/dim_and_n/std_1.0N.png}}
  \subfloat[]{\includegraphics[scale=.5]{assets/plots/dim_and_n/std_10.0N.png}}\\
  \subfloat[]{\includegraphics[scale=.5]{assets/plots/dim_and_n/std_100.0N.png}}
  \caption{Standard deviation over energy values generated on 8 cores simulatneously, for $N=1,10,100$ and $\dim=1,2,3$. The results show a staggeringly low standard deviation at the value of $\alpha = 0.5$. The system is non-interacting and the values are calculated with the brute force method.}
  \label{fig:optimal_alpha_std}
\end{figure}
\FloatBarrier


## Steepest Gradient Descent

Only the non-interacting case with 10 particles in 3 dimensions was tested with 20 thousand Monte Carlo cycles. The first test was to see what learning rate yielded sufficiently fast convergence to the correct energy. The test was done with start $alpha = 0.2$. The result can be seen in \ref{fig:sgd-learning-rates}

\begin{figure}[ht]%
  \centerfloat
  \captionsetup[subfigure]{labelformat=empty}
  \subfloat[]{\includegraphics[scale=.5]{assets/plots/SGD_learning-rate.png}}
  \caption{Steepest gradient descent of start $\alpha = 0.2$ with different learning rates, $\eta$.}
  \label{fig:sgd-learning-rates}
\end{figure}
\FloatBarrier

This shows that a learning rate of $0.0004$ is on the safe side of stability, while still beeing fast enough. We then used this learning rate for testing the convergence of our SGD to the correct $\alpha$ by starting at eight different alpha values: $\alpha = \{0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9\}$. From figure \ref{fig:sgd-alphas} below we see that the steepest gradient descent method is executed beautifully as all starting values approaches the correct $\alpha$ value of $0.5$.

\begin{figure}[ht]%
  \centerfloat
  \captionsetup[subfigure]{labelformat=empty}
  \subfloat[]{\includegraphics[scale=.5]{assets/plots/SGD_alphas.png}}
  \caption{Steepest gradient descent of different start $\alpha$.}
  \label{fig:sgd-alphas}
\end{figure}

\FloatBarrier


## Introducing importance sampling

After introducing a new Metropolis algorithm based on importance sampling, we compared the performance of it against the brute force algorithm by tracking each Monte Carlo cycle and it's corresponding calculated energy. Using blocking (as explained under Blocking in {@sec:statistical-analysis}), we calculated the standard deviation at different amounts of Monte Carlo cycles. The results are shown in figure \ref{fig:std-at-cycles}.

\begin{figure}[h!]
  \centerfloat
  \includegraphics[scale=.45]{assets/plots/std_at_cycles.png}
  \caption{Standard deviation for brute force Metropolis and Metropolis with importance sampling, plotted against the number of Monte Carlo cycles performed. The values are calcuated with blocking.}
  \label{fig:std-at-cycles}
\end{figure}
\FloatBarrier

As one can see, both methods behave very similarly, almost overlapping.


## An interacting system

Following these tests for a non-interacting system, we put our solver to the task of finding the energy of a system of $10$ particles in an elliptical harmonic potential ($\beta = \gamma = 2.82843$), at different values of $\alpha$ when the particles interact with eachother. The results are shown in figure \ref{fig:interacting-elliptical}.

\begin{figure}[htb]
  \centerfloat
  \includegraphics[scale=.5]{assets/plots/interacting_elliptical.png}
  \caption{The energy of a three-dimensional system of $10$ interacting particles, situated in an elliptical harmonic oscillator potential well. The energy is evaluated against different values of $\alpha$ and using the two Metropolis algorithms listed.}
  \label{fig:interacting-elliptical}
\end{figure}

They are quite ambigious, especially in the case of importance sampling. After this, we ran our SGD solver with the same interacting system. This yielded promising results under both Metropolis algorithms, both converging on a value just below $\alpha = 0.5$. However, as seen in figure \ref{fig:sgd-interacting}, the brute force Metropolis algorithm converges more quickly.

\begin{figure}[htb]
  \centerfloat
  \includegraphics[scale=.5]{assets/plots/sgd_interacting.png}
  \caption{Convergence of $\alpha$ for the abovementioned system, solved using the two Metropolis algorithms listed. An acceptable convergence was aquired after $150$ steps, and so the SGD was stopped there for both algorithms.}
  \label{fig:sgd-interacting}
\end{figure}
\FloatBarrier
