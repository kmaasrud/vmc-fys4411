# Method

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
		
Our distribution $p$ here is of course $\Psi_T$. The proposal distribution $q(\theta'|\theta_{i-1})$ is the distribution that suggests a new sample given the previous one. In the case which we'll call the *brute force Metropolis*, our choice here is the normal distribution centered around the previous sample to favor samples close to it. This makes the sequence into a random walk and our  $r$ becomes a bit simpler, namely $r = \frac{g(\theta')}{g(\theta_{i-1})}$. A flaw with this is that the sampler might not converge around the important parts[^important] and jump around a bit "willy-nilly". To combat this, we utilize the proposal distribution shown in eq. {@eq:proposal_distr} to get more relevant samples quicker. We will use the term *importance sampling Metropolis* to refer to this method.

[^important]: Namely the greater values of the distribution, which actually contribute.

### Monte Carlo integration {-}

To evaluate the required integrals and find the energy, we use Monte Carlo integration (see section 2.3.1 of our previous work [@AasrudRongveRaniseth2019]). Instead of sampling randomly, we use the Metropolis algorithm as explained above to get our new samples.

### Steepest gradient descent {-}

Lastly, to reach the optimum value of $\alpha$, we wish to find the minimum of $E(\alpha)$, in tune with the variational principle as shown in {@sec:the-variational-principle}. This is achieved using a simple steepest gradient descent (or SDG) method. Briefly explained, it works by following the negative value of the gradient, which always points in the direction of greatest momentaneous descent. So it proceeds as follows:

$$ \alpha_{i+1} = \alpha_i - \eta \dot E_\alpha,$$

where $\dot E_\alpha$ is the gradient of the energy with regards to $\alpha$ as defined in {@eq:energy-deriv} and $\eta$ is the so-called *learning rate* - a value which decides how big of a leap we want to do in the direction of the negative gradient.

## Statistical analysis

### Blocking

All of these computer simulations can be considered "computational experiments", and can thus be statistically analyzed in the same way as real-life experiments. There is one catch, however: All our samples are correlated with the previous one, making a "correlation chain" of sorts. [Nilsen @Nilsen2008] presents that in the case of correlated samples, the standard deviation of a sampled quantity (in our case $E$) is

$$ \sigma_E = \sqrt{\frac{1 + 2\tau /\Delta t}{n - 1}\left(\langle E^2\rangle - \langle E\rangle^2\right)}, $$

where $\Delta t$ is the time between each sample and $\tau$ is the time between one sample and the next uncorrelated sample - called the *correlation time*. To combat this correlation effect, we need to split our samples into blocks, each containing $N_\text{blocking}$ samples. Assuming the blocks are big enough that they are uncorrelated, we can calculate the variance normally based on their mean.

A natural value for $N_\text{blocking}$ would be $\tau$, but we don't know it's value. A computationally efficient way of finding it is to plot the standard deviation against different values of $N_\text{blocking}$. The error will initially increase, but eventually plateau, by which we've reached uncorrelated samples and subsequently our desired value for $N_\text{blocking}$ [@Nilsen2008]. Using this, we can confidently compute the variance of our Monte Carlo integration.

## Natural length scale

As shown in {@sec:scaled_ham}, we use scaled length-units of $r \rightarrow r' = \frac{r}{a_0}$ and $E \rightarrow E' = \frac{E}{\hbar \omega_\text{ho}}$. This gives us the constants of $a_0 = \frac{a}{a_\text{ho}}$ and $\gamma = \frac{\omega_z}{\omega_\text{ho}}$. These scaled units are used throughout the program and are later reversed to get real values.

## Choice of programming language

The Variational Monte Carlo solver is implemented in Rust. The reasons for choosing this language are two-fold:

- Rust is known for being on par with C/C++ in regards to efficiency. This makes it a great fit for heavy numerical computation, which is needed in a task like this.
- In contrast with C/C++, Rust has guaranteed memory safety. This does of course not solve any logical errors in our code, but it alleviates a lot of the memory struggles often met when dealing with such a low-level language - especially with regards to parallellization.

In addition to these, we were all very intrigued by this modern language that is currently taking the computer science world by storm.

### Auto-vectorization

Auto-vectorization in Rust is almost as easy as in C++, and can be applied by setting `RUSTFLAGS = "-C opt-level=3 -C target-cpu=native"` in the `Cargo.toml` file, which basically inputs the parameters to the compiler at compiletime. The first flag tells the compiler to run all possible optimizations. Setting `opt-level=2` is the same as running the alias `-O` which only runs some optimizations [@rustc_book]. `target-cpu` tells the compiler which cpu to compile specific code for. By inserting `native`, the compiler will compile for the cpu the compiler is run at [@rustc_book].

However, simple loops like `for i in 0..n` will not be properly vectorized due to the fact that the compiler cannot guarantee that the length of the loop is within bounds of the slice iterated over. The easiest way to ensure that this does not happen is to use an iterator. If this cannot be done, hinting to LLVM the length of the slice would also eliminate the bound checks. An example is to define the slice as
`let x = &x[0..n];`.
