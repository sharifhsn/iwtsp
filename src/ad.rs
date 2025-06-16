use reverse::*;

pub fn a() {
    let tape = Tape::new();
    let a = tape.add_var(2.5);
    let b = tape.add_var(14.);
    let c = (a.sin().powi(2) + b.ln() * 3.) - 5.;
    let gradients = c.grad();

    assert_eq!(gradients.wrt(&a), (2.0f64 * 2.5).sin());
    assert_eq!(gradients.wrt(&b), 3. / 14.);
}
