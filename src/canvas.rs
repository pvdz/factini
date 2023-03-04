use std::rc::Rc;

pub fn canvas_round_rect_and_fill_stroke(context: &Rc<web_sys::CanvasRenderingContext2d>, x: f64, y: f64, w: f64, h: f64, fill: &str, stroke: &str) {
  canvas_round_rect_rc(context, x, y, w, h);
  context.set_fill_style(&fill.into());
  context.fill();
  context.set_stroke_style(&stroke.into());
  context.stroke();
}

pub fn canvas_round_rect_rc(context: &Rc<web_sys::CanvasRenderingContext2d>, x: f64, y: f64, w: f64, h: f64) {
  // web_sys is not exposing the new roundRect so this SO answer will have to do
  // https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect
  let mut r = 10.0;
  if w < 2.0 * r { r = w / 2.0; }
  if h < 2.0 * r { r = h / 2.0; }
  context.begin_path();
  context.move_to(x+r, y);
  context.arc_to(x+w, y,   x+w, y+h, r).expect("canvas api call to work");
  context.arc_to(x+w, y+h, x,   y+h, r).expect("canvas api call to work");
  context.arc_to(x,   y+h, x,   y,   r).expect("canvas api call to work");
  context.arc_to(x,   y,   x+w, y,   r).expect("canvas api call to work");
  context.close_path();
}

pub fn canvas_round_rect(context: &web_sys::CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64) {
  // web_sys is not exposing the new roundRect so this SO answer will have to do
  // https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/roundRect
  let mut r = 10.0;
  if w < 2.0 * r { r = w / 2.0; }
  if h < 2.0 * r { r = h / 2.0; }
  context.begin_path();
  context.move_to(x+r, y);
  context.arc_to(x+w, y,   x+w, y+h, r).expect("canvas api call to work");
  context.arc_to(x+w, y+h, x,   y+h, r).expect("canvas api call to work");
  context.arc_to(x,   y+h, x,   y,   r).expect("canvas api call to work");
  context.arc_to(x,   y,   x+w, y,   r).expect("canvas api call to work");
  context.close_path();
}
