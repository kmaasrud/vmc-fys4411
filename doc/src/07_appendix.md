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

## Calculations

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



### Gradient of interacting trial wave function {#sec:trial_wf_gradient}

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

### Laplacian of interacting trial wave function {#sec:trial_wf_laplacian}

The Laplacian of the wacefunction needs to be evaluated in order to calculate

$$
\frac{1}{\Psi_T(\mathbf {r})} \nabla_k \nabla_k \Psi_T(\mathbf{r})
$$

The last part, $\nabla_k \Psi_T(\mathbf{r})$ is calculated in the section above / equation (**Reference here**). Next step is then to calculate

<!-- Check this!  -->
\begin{align*}
\nabla_k \nabla_k \Psi_T(\mathbf{r}) = &\nabla_k \bigg( \nabla_k \phi(\mathbf r_k) \left [\prod_{i\ne k} \phi(\mathbf r_i) \right] \exp{\left (\sum_{j<m}u(r_{jm})\right)} \\
 &+ \prod_i \phi (\mathbf r_i) \exp{\left(\sum_{j<m} u(r_{jm}) \right)}  \sum_{l \ne k} \nabla_k  u(\mathbf r_{kl}) \bigg) \\
\nabla_k \nabla_k \Psi_T(\mathbf r) = &\prod_{i\ne k} \left[ \nabla_k^2 \phi(\mathbf r_k) \exp{\left(\sum_{j<m} u(r_{jm})\right)} + \nabla_k \phi(\mathbf r_k) \cdot  \nabla_k \exp{\left( \sum_{j < m} u(r_{jm})\right)} \right] \\ &+ \nabla_k \prod_i \phi (\mathbf r_i) \exp{\left (\sum _{j<m} u(r_{jm})\right)}\sum_{l \ne k} \nabla_k u(r_{kl}) \\ &+
\nabla_k \exp{\left(\sum_{j < m} u(r_{jm})\right)}\prod_i \phi(\mathbf{r}_i) \sum_{l \ne k} \nabla_k u(r_{kl}) \\ &+ \nabla_k \sum_{l \ne k} \nabla_k u (r_{kl}) \prod_i \phi (\mathbf{r}_i) \exp{\sum_{j < m} u (r_{jm})}
\end{align*}

In order to avoid writing long calculations, the three main gradients are calculated below. The last of the three following expressions/equations is a bit more of a hazard to calculate. First the product rule is used. Then a rule for the gradient is applied where the gradient of a unit vector is 2 divided by its magnitude. $u'$ is parallel to the unit vector, hence their product becomes a scalar, the second derivate of $u$.

$$
\nabla_k
\exp{\left(\sum_{j <m}{u(r_{jm}}\right)} = \exp{\left(\sum_{j <m}{u(r_{jm}}\right)} \sum_{l \ne k}{\nabla_k u(r_{kl})}
$$

$$
\nabla_k \prod_i \phi(\mathbf{r}_i) = \prod _{i \ne k} \phi(\mathbf{r}_i) \nabla_k \phi(\mathbf {r}_k)
$$

\begin{align*}
\nabla_k \sum_{l \ne k}{\nabla_k u(r_{kl})} &= \sum_{l \ne k}{\nabla_k \left(\frac{\mathbf{r}_l - \mathbf {r}_k}{\mathbf{r} _{lk}} u'(r _{lk})\right)} \\ &= \sum _{l\ne k}\left(\nabla_k \frac{\mathbf{r}_l - \mathbf {r}_k}{\mathbf{r} _{lk}} u'(r _{lk}) + \frac{\mathbf{r}_l - \mathbf {r}_k}{\mathbf{r} _{lk}} \nabla_k u'(r _{lk}) \right) \\ &= \sum _{l\ne k} \frac{2}{r _{lk}} + u''(r _{lk})
\end{align*}


Finally the Laplacian can be calculated, by reintroducing the fraction $\frac{1}{\Psi_T(\mathbf{r})}$

\begin{align*}
\frac{1}{\Psi_T(\mathbf{r})} \nabla_k^2 \Psi_T(\mathbf{r}) = &\frac{\prod_{i \ne k} \phi(\mathbf{r}_i)}{\prod _{i} \phi(\mathbf{r}_i)} \left(\nabla^2_k \phi(\mathbf{r}_k) + \nabla_k \phi(\mathbf{r}_k \sum _{l\ne k}\nabla_k u(r _{kl})\right)  + \left( \frac{\nabla_k \phi(\mathbf{r}_i)}{\phi(\mathbf{r}_i)} \sum _{l \ne k} \nabla_k u(r _{kl})\right) \\ &+ \sum _{l \ne k} \nabla_k u(r _{kl})  + \sum _{j \ne k} \nabla_k u(r _{kj}) + \nabla_k  \sum _{l \ne k} \nabla_k u(r _{kl})
\end{align*}

The second and third terms are the same. Two of the terms are shown in the calculations above and $\nabla_k u(r_{kl})$ is the unit vector multiplied with the derivate of a scalar. Then we have the final expression


\begin{align*}
\frac{1}{\Psi_T(\mathbf{r})} \nabla_k^2 \Psi_T(\mathbf{r}) = &\frac{\nabla_k \phi(\mathbf{r}_k)}{\phi(\mathbf{r}_k)} + 2 \frac{\nabla_k \phi(\mathbf{r}_k)}{\phi(\mathbf{r}_k)}\sum _{j\ne k}
\frac{\mathbf{r}_j - \mathbf {r}_k}{\mathbf{r} _{jk}}u'(r _{lk}) \\ &+ \sum _{j\ne k}\sum _{l\ne k}
\frac{\mathbf{r}_j - \mathbf {r}_k}{\mathbf{r} _{jk}} u'(r _{lk}) + \sum _{j\ne k}\sum _{l\ne k}
\frac{\mathbf{r}_j - \mathbf {r}_k}{\mathbf{r} _{jk}} \frac{\mathbf{r}_l - \mathbf {r}_k}{\mathbf{r} _{lk}}  u'(r _{jk})  u'(r _{lk}) \\ &+ \sum _{l\ne k} \frac{2}{r _{lk}} u'(r _{lk}) +  u''(r _{lk})
\end{align*}

### Scaling of repulsion Hamiltonian {#sec:scaled_ham}

We have the initial expression for the Hamiltonian, {@eq:hamiltonian}. Inserting {@eq:external-potential}, we get:

$$ H = \frac{1}{2}\sum_i^N \left(-\frac{\hbar^2}{m}\nabla^2_i + m\left(\omega_\text{ho}^2 (r_{x, i}^2 + r_{y, i}^2) + \omega_z^2 r_{z, i}^2\right)\right) + \sum_{i<j}^N V_\text{int}(|\mathbf r_i - \mathbf r_j|) .$$

We now introduce the scaled length unit $r' = \frac{r}{a_\text{ho}}$ which in turn leads to $\nabla^{\prime 2}_i = a_\text{ho}^2\nabla_i^2$.

$$ H = \frac{1}{2}\sum_i^N \left(-\frac{\hbar^2}{ma_\text{ho}^2}\nabla^{\prime 2}_i  + ma_\text{ho}^2\left(\omega_\text{ho}^2(r_{x, i}^{\prime 2} + r_{y, i}^{\prime 2}) + \omega_z^2 r_{z, i}^{\prime 2}\right)\right) + \sum_{i<j}^N V_\text{int}(|\mathbf r_i - \mathbf r_j|)$$

Inserting the definition of $a_\text{ho} = \frac{\hbar}{m\omega_\text{ho}}$, we get

$$ H = \frac{1}{2}\sum_i^N \left(-\hbar\omega_\text{ho}\nabla^{\prime 2}_i  + \hbar\omega_\text{ho}\left((r_{x, i}^{\prime 2} + r_{y, i}^{\prime 2}) + \frac{\omega_z^2}{\omega_\text{ho}^2} r_{z, i}^{\prime 2}\right)\right) + \sum_{i<j}^N V_\text{int}(|\mathbf r_i - \mathbf r_j|), $$

$$ H = \frac{\hbar\omega_\text{ho}}{2}\sum_i^N \left(-\nabla^{\prime 2}_i  + (r_{x, i}^{\prime 2} + r_{y, i}^{\prime 2}) + \gamma^2 r_{z, i}^{\prime 2})\right) + \sum_{i<j}^N V_\text{int}(|\mathbf r_i - \mathbf r_j|), $$

where $\gamma = \frac{\omega_z}{\omega_\text{ho}}$. We lastly reorganize the above to obtain a scaled Hamiltonian $H' = \frac{H}{\hbar\omega_\text{ho}}$ and also make sure to scale the function $V_\text{int}\rightarrow V'_\text{int}$ by transitioning from $a\rightarrow a' = \frac{a}{a_\text{ho}}$.

$$ H' = \frac{1}{2}\sum_i^N \left(-\nabla_i^{\prime 2} + r_{x, i}^{\prime 2} + r_{y, i}^{\prime 2} + \gamma^2 r_{z, i}^{\prime 2}\right) + \sum_{i<j}^N V'_\text{int}(|\mathbf r'_i - \mathbf r'_j|) .$$ {#eq:scaled_ham_appendix}

By ensuring that we used scaled length units of $r' = \frac{r}{a_\text{ho}}$ and scaled energy units of $E' = \frac{E}{\hbar\omega_\text{ho}}$, equation {@eq:scaled_ham_appendix} holds. For simplification, we will not use the primed notation outside this derivation.
