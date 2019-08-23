# Rust-Plotter

### Intended Use Case

Rust-Plotter is intended to function as a stop gap plotting library until someone gets around to making a proper one in Rust. As it stands, this is simply an interface to allow you to graph two Vec<f64>s with [Matplotlib](https://github.com/matplotlib/matplotlib) via [serde-pickle](https://github.com/birkenfeld/serde-pickle). It does *nothing* more than that. This was created as a quick and easy method to graph a single variable as a function of time. If you aren't doing that, I'd advise you look elsewhere.

### Dependencies

R-P requires first and foremost Python 3+ and Matplotlib. If you don't have those, you can find them here: [Python](https://www.python.org/downloads/) and [Matplotlib](https://github.com/matplotlib/matplotlib). We also use [Serde](https://github.com/serde-rs/serde) and [serde-pickle](https://github.com/birkenfeld/serde-pickle), but these are dealt with in Cargo.toml.
