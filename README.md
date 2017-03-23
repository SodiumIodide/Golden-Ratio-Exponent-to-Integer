# Golden Ratio Exponent to Integer

The Golden Ratio, when taken to an integral power, tends to be very close to an integer value. This was something that I had to test for myself. As quoted by Terry Tao in his overview of the work of Yves Meyer (I don't think GitHub renders LaTeX in their Markdown, so bear with me):

> The powers `\phi, \phi^2, \phi^3, ...` of the golden ratio lie unexpectedly close to integers: for instance, `\phi^{11} = 199.005...` is unusually close to 199.

So I've been wanting to use Rust more. Perfect small project! Interface it with Gnuplot for laughs and we're away. Check it out:

![Golden Ratios are awesome](goldenratio.png?raw=true "It's so cool!")

Note that within the confines of Rust as known to me (which isn't very much, but I really love the language concepts), this is the most precision that I can get. As `\phi^n` increases in size, numerical overflow artifacts start to crop up from the stringent datatype comparator rules. I admit it, I don't know how to use a quadruple-precision float value for these calculations, so \phi^{60} is approximately as high as I can go without hitting those floating point artifacts and shooting this thing way off the chart.

Shouts out to [RustGnuplot](https://github.com/SiegeLord/RustGnuplot) for being awesome.
