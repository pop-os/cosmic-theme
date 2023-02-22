use cosmic_theme::util::over;
use palette::Srgba;

fn main() {
    let c1 = Srgba::new(0.5, 0.5, 0.5, 0.5);
    let overlay_1 = Srgba::new(1.0, 1.0, 1.0, 0.1);
    let overlay_2 = Srgba::new(1.0, 0.0, 1.0, 0.05);
    let target = over(overlay_2, over(overlay_1, c1));
    dbg!(target);
    let simplified = over(over(overlay_2, overlay_1), c1);
    dbg!(simplified);
}
