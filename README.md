# rbf-interp

This crate contains an implementation of Radial Basis Function multidimensional interpolation.
For an excellent introduction to the topic, see the [SIGGRAPH 2014 course notes].

The input is a set of datapoints, each of which has coordinates in some multidimensional
space, and a value, also provided as a vector. For example, for the colorfield images below,
the coordinates are 2D, and the values are a 3-vector, one each for red, green, and blue.
The result is a `Scatter` struct that can then be evaluated at any coordinate. The idea
is that the values vary smoothly with the coordinates, but coincide with the input at each
of the provided datapoints.

There are a number of approaches to multidimensional interpolation, but the focus of this
crate is the family of radial basis functions. These include [Polyharmonic splines], of which
the thin plate spline is an instance, a Gaussian radial basis function, and others
(muti-quadric and inverse multi-quadric). The Gaussian and multi-quadric variants also have
a tunable size parameter.

In addition, there is an "order" parameter that controls low-order polynomial terms. An
order of 0 means no additional terms, just pure basis functions. An order of 1 means a constant
term, and and order of 2 means an affine term. With these additional terms, there is a
"best-fit" constant or affine approximation of the input, and the basis functions are layered.
(For a more precise description, see section 3.1 of the SIGGRAPH notes). Note that quadratic
and higher order polynomials also make sense, but are not implemented currently.

The plots below are made with a Gaussian basis function with a deliberately too-small size
parameter (0.05), to show more clearly the effect of the polynomial term:

![Gaussian order 0](https://raw.githubusercontent.com/linebender/rbf-interp/master/docs/gaussian_pure.svg?sanitize=true)
![Gaussian order 1](https://raw.githubusercontent.com/linebender/rbf-interp/master/docs/gaussian_constant.svg?sanitize=true)
![Gaussian order 2](https://raw.githubusercontent.com/linebender/rbf-interp/master/docs/gaussian_affine.svg?sanitize=true)

With a reasonable value (1.0), results are spot-on:

![Gaussian with properly tuned width](https://raw.githubusercontent.com/linebender/rbf-interp/master/docs/gaussian1.svg?sanitize=true)

Here are comparisons with two of the other basis functions, thin plate and triharmonic:

![Thin plate](https://raw.githubusercontent.com/linebender/rbf-interp/master/docs/thinplate.svg?sanitize=true)
![Triharmonic](https://raw.githubusercontent.com/linebender/rbf-interp/master/docs/triharmonic.svg?sanitize=true)

Note that the interpolation is pretty good, but the extrapolation (the region from 1.8 to 2.0
in these plots) is weaker.

## Colorfield results

A major motivation for this crate is computing smoother interpolation for [variable fonts].
Earlier work in this space is [MutatorMath], which uses multilinear interpolation for the
task. The project page has a "colorfield" to demonstrate their interpolation techniques, which
places an (r, g, b) color at each point in a 2D coordinate space. Note that for fonts generally
the output is a 2D coordinate and the input is any number of variation axes (perhaps weight
and width are the most common), but the colorfield is a good way to visualize the behavior of
an interpolation scheme.

Below are the image from MutatorMath (scaled down), and radial basis results using Gaussian
bumps with radius 4.5 and thin plate splines

![MutatorMath](https://raw.githubusercontent.com/linebender/rbf-interp/master/docs/mutatorMath_colorField_small.png)
![Gaussian radius 4.5](https://raw.githubusercontent.com/linebender/rbf-interp/master/docs/m_gaussian4_5.png)
![Thin plate spline](https://raw.githubusercontent.com/linebender/rbf-interp/master/docs/m_thinplate.png)

The radius tuning parameter has a fairly profound effect on the results. This is a liability
in many application domains for multidimensional interpolation, but perhaps a good thing
for variable fonts, as it provides choice over the tradeoff between smoothness (perhaps
with overshoots) and local control. Curious people are encouraged to experiment with the
`mutator` example in the examples directory.

To run the example:

```rust
cargo run --example mutator | > image.ppm && open image.ppm
```

## Other resources

The following articles were of interest:

* [Data Interpolation with Radial Basis Functions (RBFs)
](http://shihchinw.github.io/2018/10/data-interpolation-with-radial-basis-functions-rbfs.html)

* [A Practical Guide to Radial Basis Functions](https://num.math.uni-goettingen.de/schaback/teaching/sc.pdf)

* [Radial Basis Function Interpolation](https://core.ac.uk/download/pdf/37320748.pdf) (Wilma du Toit Masters Thesis)

* [A simple Julia implementation](https://gist.github.com/lstagner/04a05b120e0be7de9915)

* [Thin plate splines in darktable](https://www.darktable.org/2016/05/colour-manipulation-with-the-colour-checker-lut-module/)

Thanks to Jacob Rus for discussion and resources, and to [LGM 2019]
for providing a stimulating environment to develop these ideas.

[SIGGRAPH 2014 course notes]: http://scribblethink.org/Courses/ScatteredInterpolation/scatteredinterpcoursenotes.pdf
[MutatorMath]: https://github.com/LettError/MutatorMath
[Polyharmonic splines]: https://en.wikipedia.org/wiki/Polyharmonic_spline
[variable fonts]: https://docs.microsoft.com/en-us/typography/opentype/spec/otvaroverview
[LGM 2019]: https://libregraphicsmeeting.org/2019/
