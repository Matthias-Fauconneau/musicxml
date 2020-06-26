mod xml;
mod music_xml; use music_xml::MusicXML;
use druid::{kurbo::Line, piet::{Color, FontBuilder, Text}, widget::prelude::*, WindowDesc, AppLauncher};

impl Widget<()> for MusicXML{
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut (), _env: &Env) {}
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &(), _env: &Env) {}
    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &(), _data: &(), _env: &Env) {}
    fn layout(&mut self, _layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &(), _env: &Env) -> Size { bc.max() }
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &(), _env: &Env) {
        let (bg, fg) = (Color::BLACK, Color::WHITE);
        ctx.clear(bg); // full window (otherwise fill(Rect::from_origin_size(Point::ORIGIN, ctx.size()); ctx.fill(rect, bg)))
        let staff_height = 360.;
        let margin = staff_height / 2.; // 180
        //let interval_height = staff_height / 4.; // 90

        let size = ctx.size();
        ctx.stroke(Line::new((0.,margin),(size.width,margin)), &fg, 1.);
        let _font = ctx.text().new_font_by_name("Bravura", staff_height).build().unwrap();
        //let layout = ctx.text().new_text_layout(&font, , std::f64::INFINITY).build().unwrap();
		//ctx.draw_text(&layout, (80.0, 40.0), fg);
    }
}

#[fehler::throws(anyhow::Error)] fn main() {
	let score : MusicXML = xml::from_document(&xml::parse(&std::fs::read("../test.xml")?)?)?;
	AppLauncher::with_window(WindowDesc::new(|| score))
        .use_simple_logger()
        .launch(())?
}
