use crate::{BAR_HEIGHT_PX, BLACK, FONT, GREY, RED, MAX_ACTIVE_WINDOW_CHARS, WHITE};
use penrose::{util::spawn_for_output_with_args, x::XConn, Color};
use penrose_ui::{
    bar::{
        widgets::{
            amixer_volume, wifi_network, current_date_and_time, ActiveWindowName,
            CurrentLayout, IntervalText, Workspaces,
            Text,
        },
        Position, StatusBar,
    },
    core::TextStyle,
};
use std::time::Duration;

//use penrose_ui::{bar::Position, core::TextStyle, status_bar};
// Mostly the example dwm bar from the main repo but recreated here so it's easier to tinker
// with and add in debug widgets when needed.
pub fn status_bar<X: XConn>() -> penrose_ui::Result<StatusBar<X>> {
    let highlight: Color = RED.into();
    let empty_ws: Color = GREY.into();

    let style = TextStyle {
        fg: WHITE.into(),
        bg: Some(BLACK.into()),
        padding: (2, 2),
    };

    let padded_style = TextStyle {
        padding: (4, 2),
        ..style
    };

    StatusBar::try_new(
        Position::Bottom,
        BAR_HEIGHT_PX,
        style.bg.unwrap_or_else(|| 0x000000.into()),
        FONT,
        14,
        vec![
            Box::new(Workspaces::new(style, highlight, empty_ws)),
            //esta função mostra o layout present, eu também não gosto, não faz falta
            Box::new(CurrentLayout::new(style)),
            //esta nunca la probe ja veio do penrose
            // Box::new(penrose_bar::widgets::debug::StateSummary::new(style)),
            //esta daquí mostra o nome da janela presente, tambén não faz falta para mim
            Box::new(ActiveWindowName::new(
                MAX_ACTIVE_WINDOW_CHARS,
                TextStyle {
                    bg: Some(empty_ws),
                    padding: (6, 4),
                    ..style
                },
                true,
                false,
            )),
            //Box::new(wifi_network(padded_style)),
            //Box::new(amixer_volume("Master", padded_style)),
            Box::new(current_weather_info(&padded_style)),
            //esta função daquí e puro v00d00m00
            //Box::new(Text::new(
            //    txt: impl Into<String>,
            //    style: TextStyle,
            //    is_greedy: false,
            //    right_justified: true)),
            Box::new(current_date_and_time(padded_style)),
        ],
    )
}
//Esta função foi um experimento, funcionou. Mas eu não gosto 
//de huevadas na barra. O importante a ter em conta é que
//se a função falha e ruim e já comprovei que as vezes falha

fn current_weather_info(style: &TextStyle) -> IntervalText {
  IntervalText::new(*style, get_weather_text, Duration::from_secs(60 * 15))
}

fn get_weather_text() -> String {
  spawn_for_output_with_args("curl", &["-s", "http://wttr.in/SP?format=%l:+%m+%M+%c%p%t"])
      .unwrap_or_default()
      .trim()
      .to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn get_weather_text_works() {
    let s = get_weather_text();
    assert!(!s.is_empty());
  }
}
