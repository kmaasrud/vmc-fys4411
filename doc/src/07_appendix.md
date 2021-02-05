\clearpage
\appendix

# Appendix

## Source code

All source code for both the Rust VMC implementation and this document is found on the following GitHub Repository

<https://github.com/kmaasrud/vmc-fys4411>

## Notation and other explanations

### Index notation for sums and products

For products and sums, the following convention is used:

$$\sum_{i <j}^N = \sum_{i=1}^N \sum_{j=i+1}^N,\quad \text{or}\quad \prod_{i <j}^N = \prod_{i=1}^N \prod_{j=i+1}^N$$

## Intermediate calculations

### Second derivative of trial wave function

<!-- Anna doesn't understand the last step -->
$$
\begin{aligned}
\nabla_{i}^2 \Psi_{T}(\mathbf{r})
&= \nabla_{i} \cdot\left[\frac{\partial}{\partial x_{i}}, \frac{\partial}{\partial y_{i}},   
   \frac{\partial}{\partial z_{i}}\right] \Psi_{T}(\mathbf{r}) \\
&= \nabla_i \cdot \left[\frac{\partial}{\partial x_i} \exp{(-\alpha
   \mathbf{r}_i^2}),\frac{\partial}{\partial y_i} \exp{(-\alpha \mathbf{r}_i^2}), \frac{\partial}{\partial z_i} \exp{(-\alpha \mathbf{r}_i^2})\right] \\
&= \nabla_{i} \cdot \left[-2 \alpha x_{i} \exp{(-\alpha \mathbf{r}_{i}^{2}}), -2 \alpha
   y_{i}  
   \exp{(-\alpha \mathbf{r}^2_{i}}), -2 \alpha z_{i} \exp{(-\alpha \mathbf{r}_{i}^2})
   \right] \\
&= -2 \alpha \left[  \exp{(-\alpha \mathbf{r}^2_{i}})(1 - 2 \alpha x^2_{i}), \exp{(-\alpha
   \mathbf{r}^2_{i}})(1 - 2 \alpha y^2_{i}), \exp{(-\alpha \mathbf{r}^2_{i}})
   (1 - 2 \alpha z^2_{i}) \right] \\
&= -2\alpha \Psi_{T} \left[(1 - 2 \alpha x^2_{i}), (1 - 2 \alpha y^2_{i}),
   (1 - 2 \alpha  z^2_{i}) \right]\\
&= -2\alpha \Psi_{T}\sum_{d = x,y,z}1 -2\alpha d_{i}^2 \\
&= -2\alpha \Psi_{T}(\dim - 2 \alpha  \mathbf{r}^2_{i})
\end{aligned}
$$

### Local energy for Gaussian wave function

Starting with

$$
E_L(\mathbf{r}) =
    \frac{1}{\Psi_T(\mathbf{r})} \left[ \sum_i^N \left (\frac{-\hbar^2}{2m}
    \nabla_{i}^2 \Psi_T (\mathbf{r}) + V_\text{ext} ({\mathbf{r}}_i) \Psi_T(\mathbf{r}) \right)  
    \right],
$$


and using the result from {@sec:second-derivative-of-trial-wave-function}, this results in:

$$
\begin{aligned} E_L (\mathbf r) &=  \frac{1}{\Psi_T(\mathbf{r})}  \left[ \sum_i^N \left(  \frac{\hbar^2 \alpha}{m}  (\dim - 2
    \alpha  \mathbf{r}^2_{i} ) + \frac{1}{2} m \omega^2_\text{ho} \mathbf{r}^2_{i} \right) \Psi_T(\mathbf{r}) \right ]\\
&= \frac{\hbar^2 }{m} \alpha N \dim +  \left( \frac{1}{2} m \omega^2_\text{ho} - 2 \alpha^2\right)  \sum_i^N \mathbf{r}^2_{i}
\end{aligned}
$$
