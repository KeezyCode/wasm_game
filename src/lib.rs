#![allow(deprecated)]

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    window, HtmlCanvasElement, CanvasRenderingContext2d, KeyboardEvent,
    AudioContext, OscillatorType
};
use serde::{Serialize, Deserialize};
use serde_json;

// Struct to track player position
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Player {
    x: f64,
    y: f64,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Initial player
    let mut player = Player { x: 50.0, y: 50.0 };

    draw_square(&ctx, &player);

    {
        let ctx = ctx.clone();
        let canvas = canvas.clone();
        let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
            match event.key().as_str() {
                "w" => player.y -= 10.0,
                "s" => player.y += 10.0,
                "a" => player.x -= 10.0,
                "d" => player.x += 10.0,
                " " => play_beep(),
                _ => {}
            }

            // Draw
            ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            draw_square(&ctx, &player);

            // Serialize player state to JSON
            let json = serde_json::to_string(&player).unwrap();
            web_sys::console::log_1(&JsValue::from_str(&json));
        }) as Box<dyn FnMut(_)>);

        window
            .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

fn draw_square(ctx: &CanvasRenderingContext2d, player: &Player) {
    ctx.set_fill_style(&JsValue::from_str("red"));
    ctx.fill_rect(player.x, player.y, 50.0, 50.0);
}

fn play_beep() {
    let audio_ctx = AudioContext::new().unwrap();
    let oscillator = audio_ctx.create_oscillator().unwrap();
    oscillator.set_type(OscillatorType::Sine);
    oscillator.frequency().set_value(440.0);
    oscillator.connect_with_audio_node(&audio_ctx.destination()).unwrap();
    oscillator.start().unwrap();
    oscillator.stop_with_when(audio_ctx.current_time() + 0.1).unwrap();
}
