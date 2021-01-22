# Some Pandoc Markdown syntax

I'm sure you're both familiar with the common Markdown syntax, and this applies just as well here. Pandoc exports links to the compiled PDF, so I often use them liberally, like [this](https://github.com/kmaasrud/vmc-fys4411). If you want *inline math*, you just write a set of dollar signs, like this: $f(x) = \sin(x)$. Most commonly though, you'll probably want *display math*, which is just as simple to write. Be sure to have a couple of newlines as padding, and just use a double set of dollar signs, like this:

$$E(\alpha) = \frac{\langle \psi(\alpha)|\mathcal H | \psi(\alpha)\rangle}{\langle \psi(\alpha) | \psi(\alpha)\rangle}$$

By default, these are not numerated (which I think is sensible). However, if you want them numerated, you just use the following syntax to assign a label

$$E_L (X) = \frac{\mathcal H \psi(X, \alpha)}{\psi(X, \alpha)},$$ {#eq:some_label}

and then reference the equation by using the same label. See eq. @eq:some_label (beware that this will not show up in the Markdown Preview pane. You'll need to compile the document to see the result).

One thing I find quite useful is to use footnotes throughout the document to give short insights (especially in the theory part). This is simple with Pandoc; just use this syntax^[Welcome to the footer! How are you?]. In addition, we can site all references (defined in the `kodb.yaml`-file) with this syntax [see @AasrudRongveRaniseth2019].

# Theory