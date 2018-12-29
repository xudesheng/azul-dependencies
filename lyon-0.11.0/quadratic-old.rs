use math::*;
use builder::*;

use std::ops::Range;
use std::mem;

struct Segment {
    ctrl: u16,
    to: u16,
}

struct SubPathInfo {
    range: Range<usize>,
    is_closed: bool,
}

pub struct Path {
    points: Vec<Point>,
    segments: Vec<Segment>,
    sub_paths: Vec<SubPathInfo>,
}

pub struct Builder {
    points: Vec<Point>,
    segments: Vec<Segment>,
    sub_paths: Vec<SubPathInfo>,
    sp_start: usize,
}

impl Builder {
    pub fn new() -> Self {
        Builder {
            points: Vec::new(),
            segments: Vec::new(),
            sub_paths: Vec::new(),
            sp_start: 0,
        }
    }

    fn add_segment(&mut self, to: Point) {
        self.points.push(to);
        let sp_end = self.segments.len();
        if self.sp_start != sp_end {
            self.sub_paths.push(SubPathInfo {
                range: self.sp_start..sp_end,
                is_closed: false,
            });
        }
        self.sp_start = sp_end;
        let point_idx = self.points.len() as u16;
        self.segments.push(Segment {
            ctrl: point_idx,
            to: point_idx,
        });
        self.points.push(to);
    }
}

impl FlatPathBuilder for Builder {
    type PathType = Path;
    fn move_to(&mut self, to: Point) {
        self.points.push(to);
        let sp_end = self.segments.len();
        if self.sp_start != sp_end {
            self.sub_paths.push(SubPathInfo {
                range: self.sp_start..sp_end,
                is_closed: false,
            });
        }
        self.sp_start = sp_end;
        let ctrl_idx = self.points.len() as u16;
        let to_idx = self.points.len() as u16;
        self.segments.push(Segment {
            ctrl: ctrl_idx,
            to: to_idx,
        });
        self.points.push(to);
    }

    fn line_to(&mut self, to: Point) {
        unimplemented!();
    }

    fn close(&mut self) {
        unimplemented!();
    }

    fn current_position(&self) -> Point {
        unimplemented!();
    }

    fn build(self) -> Path {
        Path {
            points: self.points,
            segments: self.segments,
            sub_paths: self.sub_paths,
        }
    }

    fn build_and_reset(&mut self) -> Path {
        self.sp_start = 0;
        Path {
            points: mem::replace(&mut self.points, Vec::new()),
            segments: mem::replace(&mut self.segments, Vec::new()),
            sub_paths: mem::replace(&mut self.sub_paths, Vec::new()),
        }
    }
}

impl PathBuilder for Builder {
    fn quadratic_bezier_to(&mut self, ctrl: Point, to: Point) {
        unimplemented!();
    }

    fn cubic_bezier_to(&mut self, ctrl1: Point, ctrl2: Point, to: Point) {
        unimplemented!();
    }

    fn arc(
        &mut self,
        center: Point,
        radii: Vector,
        sweep_angle: Angle,
        x_rotation: Angle
    ) {
        unimplemented!();
    }
}
