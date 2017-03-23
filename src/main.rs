// Plot of the Golden Ratio to a power, as a function of how close
// the number is to an integer value

extern crate gnuplot;

use common::*;

use gnuplot::*;

mod common;

// Extended precision is maintained throughout code
// This is to avoid numerical artifacts once phi^n becomes very large
struct Data {
    xs: Vec<i64>,
    ys: Vec<f64>,
}

fn plot(c: Common, data: Data) {
    let Data {xs: x, ys: y} = data;
    let x = x.iter2();
    let y = y.iter2();
    let mut fg = Figure::new();
    c.set_term(&mut fg);
    fg.axes2d()
        .set_size(0.75, 1.0)
        .set_title("Golden Ratio to an Integer Power, Distance From the Nearest Integer", &[])
        .set_x_ticks(Some((Fix(10.0), 1)), &[Mirror(false)], &[])
        .set_y_ticks(Some((Fix(0.1), 1)), &[Mirror(false)], &[])
        .set_legend(Graph(1.0), Graph(0.5), &[Placement(AlignLeft, AlignCenter)],
            &[TextAlign(AlignRight)])
        .set_border(true, &[Left, Bottom], &[LineWidth(2.0)])
        .set_x_label("n", &[])
        .set_y_label("phi^n - nearest integer", &[])
        .lines(x, y, &[Caption("Distance"), LineWidth(1.5), Color("blue")]);
    c.show(&mut fg, "goldenratio.gnuplot");
    if !c.no_show {
        fg.set_terminal("pdfcairo", "goldenratio.pdf");
        fg.show();
        fg.set_terminal("pngcairo", "goldenratio.png");
        fg.show();
    }
}

fn build_data(numpoints: i64) -> Data {
    // Definition of the Golden Ratio
    let phi: f64 = (1.0_f64 + (5.0_f64).sqrt()) / 2.0_f64;
    // Allocate vectors
    let mut xpoints: Vec<i64> = Vec::with_capacity(numpoints as usize);
    let mut ypoints: Vec<f64> = Vec::with_capacity(numpoints as usize);
    // Fill x vector; straightforward
    for x in 1..(numpoints + 1) {
        xpoints.push(x);
    }
    // Now fill y vector
    for point in &xpoints {
        let nearestint: i64 = phi.powi(*point as i32).round() as i64;
        // Note that without extending the primitive capacity of
        // Rust, these casts remove precision
        ypoints.push(phi.powi(*point as i32) - (nearestint as f64));
    }
    Data {xs: xpoints, ys: ypoints}
}

fn main() {
    let numpoints: i64 = 60;
    let data: Data = build_data(numpoints);
    Common::new().map(|c| plot(c, data));
}
