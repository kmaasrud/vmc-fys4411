# Results

## Finding the optimal $\alpha$

Using the brute force Metropolis algorithm, we calculated the expected value of the local energy at different values of $\alpha$. This was also done at different dimensions and number of particles. The simulation over all these variables were done once for each core of the processor running them. In our case, this resulted in 8 runs. The mean over all runs are seen in figure \ref{fig:optimal_alpha}.

\begin{figure}[ht]%
  \centerfloat
  \captionsetup[subfigure]{labelformat=empty}
  \subfloat[]{\includegraphics[scale=.5]{assets/dim_and_n/EnergyAlpha_1.0N.png}}
  \subfloat[]{\includegraphics[scale=.5]{assets/dim_and_n/EnergyAlpha_10.0N.png}}\\
  \subfloat[]{\includegraphics[scale=.5]{assets/dim_and_n/EnergyAlpha_100.0N.png}}
  \caption{Expected local energy (in units of $\hbar\omega_\text{ho}$) per particle, found at $N=1,10,100$ and for $\dim= 1,2,3$. The results are the means over simulations run on 8 CPU cores simultaneously. The black dashed line shows the mean minimum over all three dimensions.}
  \label{fig:optimal_alpha}
\end{figure}

In figure \ref{fig:optimal_alpha}, we see that the optimal value of $\alpha$ seems to be consistently on the value $0.5$, as expected. However, for $N = 100$ the mean deviates a bit from our expectation. A more telling picture appears when we plot the standard deviation over the CPU cores as a function of $\alpha$ instead of the expected local energy. This is shown in figure \ref{fig:optimal_alpha_std}. From this its much more clear that we're reaching the actual desired value of $\alpha$ at $0.5$, regardless of how many number of particles we're simulating for.

\begin{figure}[ht]%
  \centerfloat
  \captionsetup[subfigure]{labelformat=empty}
  \subfloat[]{\includegraphics[scale=.5]{assets/dim_and_n/std_1.0N.png}}
  \subfloat[]{\includegraphics[scale=.5]{assets/dim_and_n/std_10.0N.png}}\\
  \subfloat[]{\includegraphics[scale=.5]{assets/dim_and_n/std_100.0N.png}}
  \caption{Expected local energy (in units of $\hbar\omega_\text{ho}$) per particle, found at $N=1,10,100$ and for $\dim= 1,2,3$. The results are the means over simulations run on 8 CPU cores simultaneously. The black dashed line shows the mean minimum over all three dimensions.}
  \label{fig:optimal_alpha_std}
\end{figure}
