# Method

Rust is used to preform the numerical calculations.
<!-- Maybe explain why we chose Rust instead of traditional C++? -->

## Variational Monte Carlo

### Metropolis sampling {.unnumbered}

Since normalizing the wave function is computationally demanding, we need another way of drawing samples from it. This is where the Metropolis algorithm comes in.

For a given distribution $p(\theta)$, we know that:

$$p(\theta) \propto g(\theta),$$

where our goal is to sample from $p(\theta)$. The Metropolis algorithm proceeds as follows:

1. Select an initial value $\theta_0$. This is often chosen randomly.
2. To produce a new sample:
	1. Draw a candidate $\theta'$ from the proposal distribution $q(\theta'|\theta_{i-1})$
	2. Compute the ratio $r = \frac{g(\theta')q(\theta_{i-1}|\theta')}{g(\theta_{i-1})q(\theta'|\theta_{i-1})}$
	3. Decide:
		- If $r \ge 1$ or $r > u$ (where $u$ is a random sample from the uniform distribution), set $\theta_{i} = \theta'$.
		- Else, set $\theta_{i} = \theta_{i-1}$
3. Repeat as many times as needed.
		
Our distribution $p$ here is of course $\Psi_T$. The proposal distribution $q(\theta'|\theta_{i-1})$ is the distribution that suggests a new sample given the previous one. In the case which we'll call the *brute force Metropolis*, our choice here is the normal distribution centered around the previous sample to favor samples close to it. This makes the sequence into a random walk and our  $r$ becomes a bit simpler, namely $r = \frac{g(\theta')}{g(\theta_{i-1})}$. A flaw with this is that the sampler might not converge around the important parts[^important] and jump around a bit "willy-nilly". To combat this, we utilize the proposal distribution shown in eq. {@eq:proposal_disrt} to get more relevant samples quicker.

[^important]: Namely the greater values of the distribution, which actually contribute.

### Monte Carlo integration {.unnumbered}

To evaluate the required integrals and find the energy, we use Monte Carlo integration (see section 2.3.1 of our previous work [@AasrudRongveRaniseth2019]). Instead of sampling randomly, we use the Metropolis algorithm as explained above to get our new samples.

### Steepest gradient descent {.unnumbered}

Lastly, to reach the optimum value of $\alpha$, we wish to find the minimum of $E(\alpha)$, in tune with the variational principle as shown in {@sec:the-variational-principle}. This is achieved using a simple steepest gradient descent (or SDG) method. Briefly explained, it works by following the negative value of the gradient, which always points in the direction of greatest momentaneous descent. So it proceeds as follows:

$$ \alpha_{i+1} = \alpha_i - \eta \frac{\partial E}{\partial \alpha} ,$$

where $\eta$ is called the *learning rate* - a value which decides how big of a leap we want to do in the direction of the negative gradient.

## Natural length scale

As shown in {@sec:scaled_ham}, we use scaled length-units of $r \rightarrow r' = \frac{r}{a_0}$ and $E \rightarrow E' = \frac{E}{\hbar \omega_\text{ho}}$. This gives us the constants of $a_0 = \frac{a}{a_\text{ho}}$ and $\gamma = \frac{\omega_z}{\omega_\text{ho}}$. These scaled units are used throughout the program and are later reversed to get real values.

## Auto-vectorization

Auto-vectorization in Rust is almost as easy as in C++, and can be applied by setting `RUSTFLAGS = "-C opt-level=3 -C target-cpu=native"` in the `Cargo.toml` file, which basically inputs the parameters to the compiler at compiletime. The first flag tells the compiler to run all possible optimizations. Setting `opt-level=2` is the same as running the alias `-O` which only runs some optimizations [@rustc_book]. `target-cpu` tells the compiler which cpu to compile specific code for. By inserting `native`, the compiler will compile for the cpu the compiler is run at [@rustc_book].

However, simple loops like `for i in 0..n` will not be properly vectorized due to the fact that the compiler cannot guarantee that the length of the loop is within bounds of the slice iterated over. The easiest way to ensure that this does not happen is to use an interator. If this cannot be done, hinting to LLVM the length of the slice would also eliminate the bound checks. An example is to define the slice as
`let x = &x[0..n];`
