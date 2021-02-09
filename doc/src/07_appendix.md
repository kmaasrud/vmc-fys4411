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


<!-- For APPENDIX -->
#### Gradient and Laplacian for trail wave function general case
##### Gradient
Rewriting the wave function to

$$
\Psi_T(\mathbf{r})=\left[
    \prod_i^N \phi(\mathbf{r}_i)
\right]
\exp{\left(\sum_{i<j}u(r_{ij})\right)}
$$

where $r_{ij} = |r_i - r_j|$ and we set $u(r_{ij}) = \ln f(r_{ij})$. Lastly $g(\alpha, \beta,\mathbf{r}_i)$ is redefined to the following function

$$
\phi(\mathbf{r}_i) = \exp [-\alpha(x_i^2 + y_i^2 + \beta z_i^2)] = g(\alpha, \beta,\mathbf{r}_i).
$$

For convenience

$$ \Psi_1 (\mathbf{r}_{i})= \prod_i^N \phi(\mathbf{r}_i)$$

and  

$$\Psi_2 (\mathbf{r}_ {ij}) = \exp{\left(\sum_{i<j}u(r_{ij})\right)}$$

where $\Psi_1$  and $\Psi_2$ is the one-body and correlated part of the wave function, respectively. Both parts have simple dependency of the k'th particle. $\Psi_1$ is a product of one-body wave functions with only one factor dependent of $\mathbf{r}_k$ and $\Psi_2$ is $\mathbf{r}_k$ - dependent for the pairs $\sum _{i\ne k} u(\mathbf{r} _{ik})$.  Hence the first derivatives becomes

$$
\nabla_k \Psi_1(\mathbf{r}) = \left[\prod_ {i\ne k}^N \phi(\mathbf{r}_i) \right] \nabla_k \phi(\mathbf{r}_k)
$$


$$
\nabla_k \Psi_2(\mathbf{r}_ {ij}) = \exp {\left (\sum_ {i<j}u(r_{ij})\right)}  \sum_ {i \ne k} \nabla_k  u(\mathbf{r}_{ik})
$$

Giving the first derivate of the trail wave function

$$
\nabla_k \Psi_T(\mathbf{r}) = \nabla_k \phi(\mathbf{r}_ k) \left [\prod_ {i\ne k}^N \phi(\mathbf{r}_ i) \right] \exp {\left (\sum_ {i<j}u(r_{ij})\right)} \\ +
 \prod_i^N \phi (\mathbf{r}_ i) \exp{ \left( \sum_{i<j} u(r_{ij}) \right)}  \sum_ {i \ne k} \nabla_k  u(\mathbf{r}_{ik})
$$

##### Laplacian
The Laplacian of the wacefunction needs to be evaluated in order to calculate

$$
\frac{1}{\Psi_T(\mathbf {r})} \nabla_k \nabla_k \Psi_T(\mathbf{r})
$$

The last part, $\nabla_k \Psi_T(\mathbf{r})$ is calculated in the section above / equation (**REFERANCE**). Next step is then to calculate

<!-- Check this!  -->
$$
\nabla_k \nabla_k \Psi_T(\mathbf{r}) = \nabla_k \left( \nabla_k \phi(\mathbf{r}_ k) \left [\prod_ {i\ne k}^N \phi(\mathbf{r}_ i) \right] \exp {\left (\sum_ {j<m}u(r_{jm})\right)} \\
 +\prod_i^N \phi (\mathbf{r}_ i) \exp{ \left( \sum_{j<m} u(r_{jm}) \right)}  \sum_ {l \ne k} \nabla_k  u(\mathbf{r}_{kl}) \right) \\
= \prod_ {i\ne k}^N \left[ \nabla_k^2 \phi(\mathbf{r}_k) \exp{\left(\sum _{j<m} u(r _{jm})\right)} + \nabla_k \phi(\mathbf{r_k}) \cdot  \nabla_k \exp{\left( \sum _{j < m} u(r _{jm})\right)} \right] \\ + \nabla_k \prod_i \phi (\mathbf{r}_i) \exp{\left (\sum _{j<m} u(r _{jm})\right)}\sum _{l \ne k} \nabla_k u(r _{kl}) \\+
\nabla_k \exp{\left(\sum _{j < m} u(r _{jm})\right)}\prod_i \phi(\mathbf{r}_i) \sum _{l \ne k} \nabla_k u(r _{kl}) + \nabla_k \sum _{l \ne k} \nabla_k u (r _{kl}) \prod_i \phi (\mathbf{r}_i) \exp{\sum _{j < m} u (r _{jm})}
$$
