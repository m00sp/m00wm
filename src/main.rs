use crate::{BAR_HEIGHT_PX, BLACK, FONT, GREY, RED, MAX_ACTIVE_WINDOW_CHARS, WHITE};
use penrose::{
    builtin::{
        actions::{modify_with, send_layout_message, spawn, key_handler},
        hooks::SpacingHook,
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    core::{
        bindings::{parse_keybindings_with_xmodmap, KeyEventHandler},
        layout::LayoutStack,
        Config, WindowManager,
    },
    extensions::{
        hooks::{
            add_ewmh_hooks,
            startup::SpawnOnStartup,
            manage::{FloatingFixed, SetWorkspace, FloatingCentered, FloatingRelative},
        },
        util::dmenu::{DMenu, DMenuConfig, MenuMatch},
    },
    map, util,
    x::query::ClassName,
    x11rb::RustConn,
    manage_hooks,
};
use std::{collections::HashMap, process::exit};
use tracing_subscriber::{self, prelude::*};
use m00wm::*;
use m00wm::layouts::layouts;
use m00wm::bar::status_bar;

fn power_menu() -> Box<dyn KeyEventHandler<RustConn>> {
    key_handler(|state, _| {
        let screen_index = state.client_set.current_screen().index();
        let dmenu = DMenu::new(
            &DMenuConfig {
                show_on_bottom: true,
                selected_color: RED.into(),
                //custom_prompt: Some("->".to_string()),
                ..Default::default()
            },
            screen_index,
        );
        let choices = vec!["desligar", "reiniciar", "sair", "bloquear"];

        if let Ok(MenuMatch::Line(_, choice)) = dmenu.build_menu(choices) {
            match choice.as_ref() {
                "desligar" => util::spawn("sudo poweroff"),
                "reiniciar" => util::spawn("sudo reboot"),
                "sair" => util::spawn("pkill startplasma-x11"),
                "bloquear" => util::spawn("i3lock -c 3f0000 -f"),
                //"suspender" => util::spawn("notify-se")
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    })
}

fn raw_key_bindings() -> HashMap<String, Box<dyn KeyEventHandler<RustConn>>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_string();

        "M-j" => modify_with(|cs| cs.focus_down()),
        "M-k" => modify_with(|cs| cs.focus_up()),
        "M-S-j" => modify_with(|cs| cs.swap_down()),
        "M-S-k" => modify_with(|cs| cs.swap_up()),
        "M-S-q" => modify_with(|cs| cs.kill_focused()),
        "M-Tab" => modify_with(|cs| cs.toggle_tag()),
        "M-bracketright" => modify_with(|cs| cs.next_screen()),
        "M-bracketleft" => modify_with(|cs| cs.previous_screen()),
        "M-grave" => modify_with(|cs| cs.next_layout()),
        "M-S-grave" => modify_with(|cs| cs.previous_layout()),
        "M-S-Up" => send_layout_message(|| IncMain(1)),
        "M-S-Down" => send_layout_message(|| IncMain(-1)),
        "M-S-Right" => send_layout_message(|| ExpandMain),
        "M-S-Left" => send_layout_message(|| ShrinkMain),
        "M-d" => spawn("rofi -theme /usr/share/rofi/themes/gruvbox-dark-hard.rasi -modi drun,run -show run"),
        "M-l" => spawn("dmenu_run"),
        "M-Return" => spawn("konsole"),
        "M-s" => power_menu(), //sair tipo logoff
        //"M-S-l" => spawn("pkill -fi m00wm"), //sair matando o ps
    };

    for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
        raw_bindings.extend([
            (
                format!("M-{tag}"),
                modify_with(move |client_set| client_set.focus_tag(tag)),
            ),
            (
                format!("M-S-{tag}"),
                modify_with(move |client_set| client_set.move_focused_to_tag(tag)),
            ),
        ]);
    }

    raw_bindings
}

//fn layouts() -> LayoutStack {
    //LayoutStack::default()
//}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .finish()
        .init();

    let conn = RustConn::new()?;
    let key_bindings = parse_keybindings_with_xmodmap(raw_key_bindings())?;
    let startup_hook = SpawnOnStartup::boxed("$HOME./config/m00wm.init");
    let layout_hook = Box::new(SpacingHook {
        inner_px: 0,
        outer_px: 0,
        top_px: 2,
        bottom_px: BAR_HEIGHT_PX,
    });
    //let manage_hook = Box::new((ClassName("firefox"),SetWorkspace("2")));
    let my_manage_hook = manage_hooks! {
        ClassName("firefox") => SetWorkspace("2"),
        ClassName("plasmashell") => SetWorkspace("9"),
		ClassName("Xephyr") => FloatingCentered::new(0.8, 0.8),
		ClassName("Xephyr") => SetWorkspace("5"),
    }; 
    let config = add_ewmh_hooks(Config {
        default_layouts: layouts(),
        floating_classes: vec!["plasmashell".to_owned(), "krunner".to_owned(), "Plasma".to_owned(), "Kmix".to_owned(), "Klipper".to_owned(),"Plasmoidviewer".to_owned(), "systemsettings".to_owned()],
        startup_hook: Some(startup_hook),
        layout_hook: Some(layout_hook),
        manage_hook: Some(my_manage_hook),
        ..Config::default()
    });
    
    let bar = status_bar()?;
    let wm = bar.add_to(WindowManager::new(
        config, 
        key_bindings, 
        HashMap::new(), 
        conn,
    )?);

    wm.run()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        let res = parse_keybindings_with_xmodmap(raw_key_bindings());

        if let Err(e) = res {
            panic!("{e}");
        }
    }
}
