use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlImageElement, MouseEvent};

struct Player {
    y: f64,
    velocity_y: f64,
    gravity: f64,
    jump_force: f64,
    is_on_ground: bool,
}

impl Player {
    fn new() -> Self {
        Self {
            y: 260.0,
            velocity_y: 0.0,
            gravity: 0.8,
            jump_force: -18.0,
            is_on_ground: false,
        }
    }

    fn update(&mut self) {
        self.velocity_y += self.gravity;
        self.y += self.velocity_y;
        if self.y > 260.0 {
            self.y = 260.0;
            self.velocity_y = 0.0;
            self.is_on_ground = true;
        } else {
            self.is_on_ground = false;
        }
    }

    fn jump(&mut self) {
        if self.is_on_ground {
            self.velocity_y = self.jump_force;
        }
    }
}

struct Obstacle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    speed: f64,
}

impl Obstacle {
    fn new(canvas_width: f64, current_speed: f64) -> Self {
        Self {
            x: canvas_width,
            y: 280.0,
            width: 60.0,
            height: 60.0,
            speed: current_speed,
        }
    }

    fn update(&mut self) {
        self.x -= self.speed;
    }
}

#[wasm_bindgen]
pub fn run_game() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no window");
    let document = window.document().expect("no document");
    let canvas = document.get_element_by_id("game-canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas.get_context("2d")?.unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let bg_image = HtmlImageElement::new()?;
    let bg_loaded = Rc::new(RefCell::new(false));
    let bg_loaded_clone = bg_loaded.clone();
    let bg_onload = Closure::wrap(Box::new(move || { *bg_loaded_clone.borrow_mut() = true; }) as Box<dyn FnMut()>);
    bg_image.set_onload(Some(bg_onload.as_ref().unchecked_ref()));
    bg_image.set_src("background.png"); 
    bg_onload.forget();

    let player_image = HtmlImageElement::new()?;
    let player_loaded = Rc::new(RefCell::new(false));
    let player_loaded_clone = player_loaded.clone();
    let p_onload = Closure::wrap(Box::new(move || { *player_loaded_clone.borrow_mut() = true; }) as Box<dyn FnMut()>);
    player_image.set_onload(Some(p_onload.as_ref().unchecked_ref()));
    player_image.set_src("player.png");
    p_onload.forget();

    let obstacle_image = HtmlImageElement::new()?;
    let obstacle_loaded = Rc::new(RefCell::new(false));
    let obstacle_loaded_clone = obstacle_loaded.clone();
    let obs_onload = Closure::wrap(Box::new(move || { *obstacle_loaded_clone.borrow_mut() = true; }) as Box<dyn FnMut()>);
    obstacle_image.set_onload(Some(obs_onload.as_ref().unchecked_ref()));
    obstacle_image.set_src("obstacle.png");
    obs_onload.forget();

    let player = Rc::new(RefCell::new(Player::new()));
    let obstacles = Rc::new(RefCell::new(Vec::<Obstacle>::new()));
    let is_game_over = Rc::new(RefCell::new(false));
    let frame_count = Rc::new(RefCell::new(0));
    let score = Rc::new(RefCell::new(0));
    let game_speed = Rc::new(RefCell::new(5.0));

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    {
        let p = player.clone();
        let obs = obstacles.clone();
        let go = is_game_over.clone();
        let fc = frame_count.clone();
        let sc = score.clone();
        let gs = game_speed.clone();
        let loop_trigger = g.clone();
        
        let start_btn = document.get_element_by_id("start-btn").unwrap();
        let restart_btn = document.get_element_by_id("restart-btn").unwrap();
        
        let start_btn_clone = start_btn.clone();
        let on_start = Closure::wrap(Box::new(move || {
            let _ = start_btn_clone.set_attribute("style", "display: none;");
            request_animation_frame(loop_trigger.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>);
        start_btn.add_event_listener_with_callback("click", on_start.as_ref().unchecked_ref())?;
        on_start.forget();

        let loop_trigger_restart = g.clone();
        let btn_handle = restart_btn.clone();
        let on_restart = Closure::wrap(Box::new(move || {
            *p.borrow_mut() = Player::new();
            *obs.borrow_mut() = Vec::new();
            *go.borrow_mut() = false;
            *fc.borrow_mut() = 0;
            *sc.borrow_mut() = 0;
            *gs.borrow_mut() = 5.0;
            let _ = btn_handle.set_attribute("style", "display: none;");
            request_animation_frame(loop_trigger_restart.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>);
        restart_btn.add_event_listener_with_callback("click", on_restart.as_ref().unchecked_ref())?;
        on_restart.forget();
    }

    {
        let player_for_events = player.clone();
        let is_game_over_for_events = is_game_over.clone();
        let on_keydown = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
            if !*is_game_over_for_events.borrow() {
                if event.key() == " " || event.key() == "ArrowUp" {
                    player_for_events.borrow_mut().jump();
                    event.prevent_default();
                }
            }
        }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        window.add_event_listener_with_callback("keydown", on_keydown.as_ref().unchecked_ref())?;
        on_keydown.forget();

        let player_for_mouse = player.clone();
        let is_game_over_for_mouse = is_game_over.clone();
        let on_mousedown = Closure::wrap(Box::new(move |event: MouseEvent| {
            if !*is_game_over_for_mouse.borrow() {
                player_for_mouse.borrow_mut().jump();
                event.prevent_default();
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        canvas.add_event_listener_with_callback("mousedown", on_mousedown.as_ref().unchecked_ref())?;
        on_mousedown.forget();
    }

    *g.borrow_mut() = Some(Closure::wrap(Box::new({
        let obstacles = obstacles.clone();
        let player = player.clone();
        let is_game_over = is_game_over.clone();
        let frame_count = frame_count.clone();
        let score = score.clone();
        let game_speed = game_speed.clone();
        let f_inner = f.clone();
        let bg_image = bg_image.clone();
        let bg_loaded = bg_loaded.clone();
        let p_image = player_image.clone();
        let p_loaded = player_loaded.clone();
        let obs_image = obstacle_image.clone();
        let obs_loaded = obstacle_loaded.clone();
        move || {
            if *is_game_over.borrow() { return; }

            let mut p = player.borrow_mut();
            p.update();
            let mut obs_list = obstacles.borrow_mut();
            let mut fc = frame_count.borrow_mut();
            *fc += 1;
            if *fc % 5 == 0 {
                *score.borrow_mut() += 1;
                if *score.borrow() % 100 == 0 { *game_speed.borrow_mut() += 0.4; }
            }
            if *fc % 100 == 0 { obs_list.push(Obstacle::new(canvas.width() as f64, *game_speed.borrow())); }
            
            for obs in obs_list.iter_mut() {
                obs.update();
                let p_hit_w = 40.0;
                let p_hit_h = 40.0;
                let p_hit_x = 50.0 + 20.0;
                let p_hit_y = p.y + 20.0;
                let o_hit_w = 30.0;
                let o_hit_h = 30.0;
                let o_hit_x = obs.x + 15.0;
                let o_hit_y = obs.y + 15.0;
                if p_hit_x < o_hit_x + o_hit_w && p_hit_x + p_hit_w > o_hit_x && p_hit_y < o_hit_y + o_hit_h && p_hit_y + p_hit_h > o_hit_y {
                    *is_game_over.borrow_mut() = true;
                }
            }
            obs_list.retain(|obs| obs.x > -100.0);

            context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
            
            if *bg_loaded.borrow() {
                let img_w = bg_image.width() as f64;
                let img_h = bg_image.height() as f64;
                let canvas_h = canvas.height() as f64;
                let aspect_ratio = img_w / img_h;
                let new_width = canvas_h * aspect_ratio;
                let x_offset = (canvas.width() as f64 - new_width) / 2.0;
                let _ = context.draw_image_with_html_image_element_and_dw_and_dh(&bg_image, x_offset, 0.0, new_width, canvas_h);
            }

            if *p_loaded.borrow() {
                let _ = context.draw_image_with_html_image_element_and_dw_and_dh(&p_image, 50.0, p.y, 80.0, 80.0);
            } else {
                context.set_fill_style_str("green");
                context.fill_rect(50.0, p.y, 80.0, 80.0);
            }

            if *obs_loaded.borrow() {
                for obs in obs_list.iter() {
                    let _ = context.draw_image_with_html_image_element_and_dw_and_dh(&obs_image, obs.x, obs.y, obs.width, obs.height);
                }
            } else {
                context.set_fill_style_str("red");
                for obs in obs_list.iter() {
                    context.fill_rect(obs.x, obs.y, obs.width, obs.height);
                }
            }

            context.set_fill_style_str("white");
            context.set_font("bold 20px 'Courier New'");
            let _ = context.fill_text(&format!("SCORE: {:05}", *score.borrow()), 20.0, 35.0);
            let _ = context.fill_text(&format!("SPEED: {:.1}", *game_speed.borrow()), 20.0, 65.0);

            if *is_game_over.borrow() {
                context.set_fill_style_str("rgba(0, 0, 0, 0.5)");
                context.fill_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());
                context.set_fill_style_str("white");
                context.set_font("48px Arial");
                let _ = context.fill_text("Game Over!", (canvas.width() as f64 / 2.0) - 120.0, 250.0);
                if let Some(btn) = document.get_element_by_id("restart-btn") {
                    let _ = btn.set_attribute("style", "display: block; position: absolute; top: 60%; left: 50%; transform: translate(-50%, -50%); padding: 10px 20px; font-size: 20px; cursor: pointer; background: white; border: 2px solid black;");
                }
            } else {
                request_animation_frame(f_inner.borrow().as_ref().unwrap());
            }
        }
    }) as Box<dyn FnMut()>));

    Ok(())
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window().unwrap().request_animation_frame(f.as_ref().unchecked_ref()).expect("frame error");
}