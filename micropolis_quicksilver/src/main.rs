use quicksilver::{
    geom::{Shape, Vector},
    graphics::{Background::Img, Color, Image},
    lifecycle::{run, Asset, Settings, State, Window},
    Result,
};

struct MicropolisGame {
    asset: Asset<Image>,
}

impl State for MicropolisGame {
    fn new() -> Result<Self> {
        let asset = Asset::new(Image::load("test.png"));
        Ok(MicropolisGame { asset })
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::INDIGO)?;
        self.asset.execute(|img| {
            window.draw(&img.area().with_center((400, 300)), Img(&img));
            Ok(())
        })
    }
}

fn main() {
    run::<MicropolisGame>(
        "micropolis-rs",
        Vector::new(800, 600),
        Settings {
            icon_path: Some("test.png"),
            ..Settings::default()
        },
    )
}
