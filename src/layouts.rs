use crate::{MAX_MAIN, RATIO, RATIO_STEP};
use penrose::{
    builtin::layout::{
        messages::{ExpandMain, ShrinkMain},
        transformers::ReflectVertical,
        CenteredMain, Grid, MainAndStack, Monocle
    },
    core::layout::{Layout, LayoutStack, Message},
    pure::{geometry::Rect, Stack},
    Xid,
    extensions::layout::{Conditional, Tatami},
    stack,
};

pub fn layouts() -> LayoutStack {
    stack!(
        MainAndStack::bottom(MAX_MAIN, RATIO, RATIO_STEP),        
        MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP),        
        //Mimomu::boxed_clone(), ta mal
        //ReflectVertical::wrap(Mimomu::boxed_clone()),
        Tatami::boxed(0.6, 0.1),
        Grid::boxed(),
        Monocle::boxed()
    )
}

#[derive(Debug, Clone, Copy)]
pub struct Mimomu;
impl Layout for Mimomu {
    fn name(&self) -> String {
        "m00".to_string()
    }
    fn boxed_clone(&self) -> Box<dyn Layout> {
        Box::new(*self)
    }
    fn layout(&mut self, s: &Stack<Xid>, r: Rect) -> (Option<Box<dyn Layout>>, Vec<(Xid, Rect)>) {
        //Exemplo manual de divisão da tela 
        let (r1, r2) = r.split_at_mid_height(); // usar split_at_mid_width para o inverso
        let (r2, r3) = r2.split_at_mid_width();
        let (r1, r4) = r1.split_at_mid_width();
        let positions = vec![(1.into(), r1), (2.into(), r2), (3.into(), r3), (4.into(), r4)];

        (None, positions)
    }
    fn handle_message(&mut self, _: &Message) -> Option<Box<dyn Layout>> {
        //a fazer,
        None
    }
}
    //stack!(
        //flex_tall(),
        //flex_wide(),
        //MainAndStack::side(MAX_MAIN, RATIO, RATIO_STEP),
        //Tatami::boxed(RATIO, RATIO_STEP),
        //Grid::boxed(),
        //Monocle::boxed()
    //)
//}
//
//fn flex_tall() -> Box<dyn Layout> {
    //Conditional::boxed(
        //"FlexTall",
        //MainAndStack::side_unboxed(MAX_MAIN, RATIO, RATIO_STEP, false),
        //CenteredMain::vertical_unboxed(MAX_MAIN, RATIO, RATIO_STEP),
        //|_, r| r.w <= 1400,
    //)
//}
//
//fn flex_wide() -> Box<dyn Layout> {
    //Conditional::boxed(
        //"FlexWide",
        //MainAndStack::bottom_unboxed(MAX_MAIN, RATIO, RATIO_STEP, false),
        //CenteredMain::horizontal_unboxed(MAX_MAIN, RATIO, RATIO_STEP),
        //|_, r| r.w <= 1400,
    //)
//}
