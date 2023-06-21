use crate::library::prelude::*;

pub fn get_hiearchy() -> Ui::Hiearchy {
    let mut system = Ui::Hiearchy::new();

    let mut _widget = Ui::Widget::new(&mut system, "App", Ui::Pos::Relative {
        point_relative_1: [0.0, 0.0],
        point_relative_2: [100.0, 100.0],
        ..Default::default()
    }.wrap()).unwrap();

    let mut _background = Ui::Widget::new_in(&mut system, &_widget, "Background", Ui::Pos::Solid {
        width: 2560,
        height: 1440,
        size: Ui::SolidSize::Fill,
        ..Default::default()
    }.wrap()).unwrap();

    let mut _board = Ui::Widget::new_in(&mut system, &_widget, "Board", Ui::Pos::Solid {
        width: 807,
        height: 1432,
        horizontal_anchor: -0.85,
        size: Ui::SolidSize::Fit,
        ..Default::default()
    }.wrap()).unwrap();

    let mut _logo_boundary = Ui::Widget::new_in(&mut system, &_board, "", Ui::Pos::Relative {
        point_relative_1: [-5.0, 70.0],
        point_relative_2: [105.0, 85.0],
        ..Default::default()
    }.wrap()).unwrap();

    let mut _logo = Ui::Widget::new_in(&mut system, &_board, "#p0/Logo", Ui::Pos::Solid {
        width: 681,
        height: 166,
        size: Ui::SolidSize::Fit,
        ..Default::default()
    }.wrap()).unwrap();

    let mut _logo_boundary = Ui::Widget::new_in(&mut system, &_logo, "LogoShadow", Ui::Pos::Relative {
        point_relative_1: [-5.0, -10.0],
        point_relative_2: [105.0, 110.0],
        ..Default::default()
    }.wrap()).unwrap();

    //==============================================
    //#BUTTONS
    //--------
    let mut _button_list = Ui::Widget::new_in(&mut system, &_board, "ButtonList", Ui::Pos::Relative {
        point_relative_1: [17.0, 21.0],
        point_relative_2: [82.0, 66.0],
        ..Default::default()
    }.wrap()).unwrap();

    let button_list = ["continue", "new_game", "load_game", "settings", "additional_content", "credits", "quit_game"];
    let step = 2.0/button_list.len() as f32;

    for i in 0..button_list.len() {
        Ui::Widget::new_in(&mut system, &_button_list, button_list[i], Ui::Pos::Solid {
            width: 532,
            height: 75,
            size: Ui::SolidSize::Fit,
            vertical_anchor: 1.0 - step * i as f32,
            ..Default::default()
        }.wrap()).unwrap();
    }

    //println!("{}",system.map_debug());
    println!("{}",system.map());

    system
}