
extern crate autopilot;
extern crate inputbot;

use autopilot::{geometry::Point, mouse::Button};
use std::{thread::sleep, time::Duration};
use std::process;
use inputbot::{KeybdKey::*, MouseCursor, MouseButton::*};




fn main() {
    //         // 1920x1080 ---> x : 1168 y : 919
    //         // 1366x768 ----> x : 843.78, y : 653.51
    //         // 2560x1080 ---> x : 1488.0, y : 919.0

    //         // 1920x1080 ---> x : 960, y : 809
    //         // 1366x768 ----> x : 683.0, y : 575.28
    //         // 2560x1080 ---> x : 1280.0, y : 809.0

    let fullHDx = 1920; // ---> x : 1168
    let fullHDy = 1080; // ---> y : 919

    let screen_user = autopilot::screen::size();
    let screen_user_width = screen_user.width as i32;
    let screen_user_height = screen_user.height as i32;
    
    let mut linhaY1 = 920;
    let mut linhaY2 = 1005;


    let mut agent1 = 588;
    
    let mut agent2 = 668;

    let mut agent3 = 753;

    let mut agent4 = 834;

    let mut agent5 = 926;

    let mut agent6 = 1005;

    let mut agent7 = 1090;

    let mut agent8 = 1175;

    let mut agent9 = 1251;

    let mut agent10 = 1334;
    
    

    let mut pos_selec_x = 960;
    let mut pos_selec_y = 809;


    if (screen_user_width != fullHDx) || (screen_user_height != fullHDy) {

        linhaY1 = screen_user_height * 920 / fullHDy;
        linhaY2 = screen_user_height * 1005 / fullHDy;
        
        agent1 = screen_user_width * 588 / fullHDx;

        agent2 = screen_user_width * 668 / fullHDx;

        agent3 = screen_user_width * 753 / fullHDx;

        agent4 = screen_user_width * 834 / fullHDx;

        agent5 = screen_user_width * 926 / fullHDx;

        agent6 = screen_user_width * 1005 / fullHDx;

        agent7 = screen_user_width * 1090 / fullHDx;

        agent8 = screen_user_width * 1175 / fullHDx;

        agent9 = screen_user_width * 1251 / fullHDx;
        
        agent10 = screen_user_width * 1334 / fullHDx;

        pos_selec_x = screen_user_width * 960 / fullHDx;
        pos_selec_y = screen_user_height * 809 / fullHDy;


    }

    
    
    Numrow1Key.bind(move || {
        MouseCursor::move_abs(agent1, linhaY1);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });
    Numrow2Key.bind(move || {
        MouseCursor::move_abs(agent2, linhaY1);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });
    Numrow3Key.bind(move || {
        MouseCursor::move_abs(agent3, linhaY1);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });
    Numrow4Key.bind(move || {
        MouseCursor::move_abs(agent4, linhaY1);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });
    Numrow5Key.bind(move || {
        MouseCursor::move_abs(agent5, linhaY1);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });
    Numrow6Key.bind(move || {
        MouseCursor::move_abs(agent6, linhaY1);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });
    Numrow7Key.bind(move || {
        MouseCursor::move_abs(agent7, linhaY1);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });
    Numrow8Key.bind(move || {
        MouseCursor::move_abs(agent8, linhaY1);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });
    Numrow9Key.bind(move || {
        MouseCursor::move_abs(agent9, linhaY1);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });
    Numrow0Key.bind(move || {
        MouseCursor::move_abs(agent10, linhaY1);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });

    F1Key.bind(move || {
        MouseCursor::move_abs(agent1, linhaY2);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });

    F2Key.bind(move || {
        MouseCursor::move_abs(agent2, linhaY2);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });

    F3Key.bind(move || {
        MouseCursor::move_abs(agent3, linhaY2);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });

    F4Key.bind(move || {
        MouseCursor::move_abs(agent4, linhaY2);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });

    F5Key.bind(move || {
        MouseCursor::move_abs(agent5, linhaY2);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });

    F6Key.bind(move || {
        MouseCursor::move_abs(agent6, linhaY2);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });

    F7Key.bind(move || {
        MouseCursor::move_abs(agent7, linhaY2);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });

    F8Key.bind(move || {
        MouseCursor::move_abs(agent8, linhaY2);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });

    F9Key.bind(move || {
        MouseCursor::move_abs(agent9, linhaY2);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });

    F10Key.bind(move || {
        MouseCursor::move_abs(agent10, linhaY2);
        LeftButton.press();
        LeftButton.release();
        sleep(Duration::from_millis(10));
        MouseCursor::move_abs(pos_selec_x, pos_selec_y);
        LeftButton.press();
        LeftButton.release();
    });
    
    inputbot::handle_input_events();
    

    
    
        
}

    
    


    

