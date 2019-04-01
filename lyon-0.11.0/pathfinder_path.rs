// A significant portion of the code in this file was ported from pathfinder
// https://github.com/pcwalton/pathfinder

use geom::{LineSegment, QuadraticBezierSegment};
use math::Point;
use events::{QuadraticEvent, Segment};

use std::mem;
use std::ops::Range;
use std::u32;

/// Holds one or more paths in memory in an efficient form.
///
/// This structure is generally preferable to `Vec<QuadraticEvent>` if you need to buffer paths in
/// memory. It is both smaller and offers random access to individual subpaths.
#[derive(Clone, Debug)]
pub struct PathBuffer {
    /// All endpoints of all subpaths.
    pub endpoints: Vec<Endpoint>,
    /// All control points of all subpaths.
    pub control_points: Vec<Point>,
    /// A series of ranges defining each subpath.
    pub subpaths: Vec<Subpath>,
}

impl PathBuffer {
    /// Creates a new, empty path buffer.
    #[inline]
    pub fn new() -> PathBuffer {
        PathBuffer {
            endpoints: vec![],
            control_points: vec![],
            subpaths: vec![],
        }
    }

    /// Appends a sequence of path commands to this path buffer.
    pub fn add_stream<I>(&mut self, stream: I) where I: Iterator<Item = QuadraticEvent> {
        let mut first_subpath_endpoint_index = self.endpoints.len() as u32;
        for segment in stream {
            match segment {
                QuadraticEvent::Close => self.close_subpath(&mut first_subpath_endpoint_index),

                QuadraticEvent::MoveTo(to) => {
                    self.end_subpath(&mut first_subpath_endpoint_index);
                    self.endpoints.push(Endpoint {
                        position: to,
                        control_point_index: u32::MAX,
                        subpath_index: self.subpaths.len() as u32,
                    })
                }

                QuadraticEvent::LineTo(to) => {
                    self.endpoints.push(Endpoint {
                        position: to,
                        control_point_index: u32::MAX,
                        subpath_index: self.subpaths.len() as u32,
                    })
                }

                QuadraticEvent::QuadraticTo(ctrl, to) => {
                    let control_point_index = self.control_points.len() as u32;
                    self.control_points.push(ctrl);
                    self.endpoints.push(Endpoint {
                        position: to,
                        control_point_index: control_point_index,
                        subpath_index: self.subpaths.len() as u32,
                    })
                }
            }
        }

        self.end_subpath(&mut first_subpath_endpoint_index)
    }

    fn close_subpath(&mut self, first_subpath_endpoint_index: &mut u32) {
        if self.endpoints.len() > *first_subpath_endpoint_index as usize {
            let first_endpoint = self.endpoints[*first_subpath_endpoint_index as usize];
            self.endpoints.push(first_endpoint);
        }

        self.do_end_subpath(first_subpath_endpoint_index, true)
    }

    fn end_subpath(&mut self, first_subpath_endpoint_index: &mut u32) {
        self.do_end_subpath(first_subpath_endpoint_index, false)
    }

    fn do_end_subpath(&mut self, first_subpath_endpoint_index: &mut u32, closed: bool) {
        let last_subpath_endpoint_index = self.endpoints.len() as u32;
        if *first_subpath_endpoint_index != last_subpath_endpoint_index {
            self.subpaths.push(Subpath {
                first_endpoint_index: *first_subpath_endpoint_index,
                last_endpoint_index: last_subpath_endpoint_index,
                closed: closed,
            })
        }

        *first_subpath_endpoint_index = last_subpath_endpoint_index;
    }

    /// Reverses the winding order of the subpath with the given index.
    pub fn reverse_subpath(&mut self, subpath_index: u32) {
        let subpath = &self.subpaths[subpath_index as usize];
        let endpoint_range = subpath.range();
        if endpoint_range.start == endpoint_range.end {
            return
        }

        self.endpoints[endpoint_range.clone()].reverse();

        for endpoint_index in (endpoint_range.start..(endpoint_range.end - 1)).rev() {
            let control_point_index = self.endpoints[endpoint_index].control_point_index;
            self.endpoints[endpoint_index + 1].control_point_index = control_point_index;
        }

        self.endpoints[endpoint_range.start].control_point_index = u32::MAX;
    }
}

/// Converts a path buffer back into a series of path commands.
#[derive(Clone)]
pub struct PathBufferStream<'a> {
    path_buffer: &'a PathBuffer,
    endpoint_index: u32,
    subpath_index: u32,
    last_subpath_index: u32,
}

impl<'a> PathBufferStream<'a> {
    /// Prepares a path buffer stream to stream all subpaths from the given path buffer.
    #[inline]
    pub fn new<'b>(path_buffer: &'b PathBuffer) -> PathBufferStream<'b> {
        PathBufferStream {
            path_buffer: path_buffer,
            endpoint_index: 0,
            subpath_index: 0,
            last_subpath_index: path_buffer.subpaths.len() as u32,
        }
    }

    /// Prepares a path buffer stream to stream only a subset of subpaths from the given path
    /// buffer.
    #[inline]
    pub fn subpath_range<'b>(path_buffer: &'b PathBuffer, subpath_range: Range<u32>)
                             -> PathBufferStream<'b> {
        let first_endpoint_index = if subpath_range.start == subpath_range.end {
            0
        } else {
            path_buffer.subpaths[subpath_range.start as usize].first_endpoint_index
        };
        PathBufferStream {
            path_buffer: path_buffer,
            endpoint_index: first_endpoint_index,
            subpath_index: subpath_range.start,
            last_subpath_index: subpath_range.end,
        }
    }
}

impl<'a> Iterator for PathBufferStream<'a> {
    type Item = QuadraticEvent;

    fn next(&mut self) -> Option<QuadraticEvent> {
        if self.subpath_index == self.last_subpath_index {
            return None
        }

        let subpath = &self.path_buffer.subpaths[self.subpath_index as usize];
        if self.endpoint_index == subpath.last_endpoint_index {
            self.subpath_index += 1;
            if subpath.closed {
                return Some(QuadraticEvent::Close)
            }
            return self.next()
        }

        let endpoint_index = self.endpoint_index;
        self.endpoint_index += 1;

        let endpoint = &self.path_buffer.endpoints[endpoint_index as usize];

        if endpoint_index == subpath.first_endpoint_index {
            return Some(QuadraticEvent::MoveTo(endpoint.position))
        }

        if endpoint.control_point_index == u32::MAX {
            return Some(QuadraticEvent::LineTo(endpoint.position))
        }

        let control_point = &self.path_buffer
            .control_points[endpoint.control_point_index as usize];
        Some(QuadraticEvent::QuadraticTo(*control_point, endpoint.position))
    }
}

/// Describes a path endpoint in a path buffer.
#[repr(C)]
//#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(Debug, Clone, Copy)]
pub struct Endpoint {
    /// The 2D position of the endpoint.
    pub position: Point,
    /// The index of the control point *before* this endpoint in the `control_points` vector, or
    /// `u32::MAX` if this endpoint is the end of a line segment.
    pub control_point_index: u32,
    /// The index of the subpath that this endpoint belongs to.
    pub subpath_index: u32,
}

/// Stores the endpoint indices of each subpath.
#[repr(C)]
//#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[derive(Debug, Clone, Copy)]
pub struct Subpath {
    /// The index of the first endpoint that makes up this subpath.
    pub first_endpoint_index: u32,
    /// One plus the index of the last endpoint that makes up this subpath.
    pub last_endpoint_index: u32,
    /// Whether the subpath is closed (i.e. fully connected).
    pub closed: bool,
}

impl Subpath {
    /// Returns the endpoint indices as a `Range`.
    #[inline]
    pub fn range(self) -> Range<usize> {
        (self.first_endpoint_index as usize)..(self.last_endpoint_index as usize)
    }
}

// TODO: make a generic Segments iterator implemented on top of PathIterator and friends.

/// Yields a set of `Segment`s corresponding to a list of `QuadraticEvent`s.
///
/// For example, the path commands `[MoveTo(A), LineTo(B), LineTo(C), Close]` become
/// `[Line(A, B), Line(B, C), Line(C, A)]`.
///
/// This representation can simplify the implementation of certain geometric algorithms, such as
/// offset paths (stroking).
pub struct QuadraticSegments<I> {
    inner: I,
    current_subpath_index: u32,
    current_point: Point,
    current_subpath_start_point: Point,
}

impl<I> QuadraticSegments<I> where I: Iterator<Item = QuadraticEvent> {
    /// Creates a new path segment stream that will yield path segments from the given collection
    /// of path commands.
    pub fn new(inner: I) -> QuadraticSegments<I> {
        QuadraticSegments {
            inner: inner,
            current_subpath_index: u32::MAX,
            current_point: Point::zero(),
            current_subpath_start_point: Point::zero(),
        }
    }
}

impl<I> Iterator for QuadraticSegments<I> where I: Iterator<Item = QuadraticEvent> {
    type Item = (Segment, u32);

    fn next(&mut self) -> Option<(Segment, u32)> {
        loop {
            match self.inner.next() {
                None => return None,
                Some(QuadraticEvent::MoveTo(point)) => {
                    self.current_subpath_index = self.current_subpath_index.wrapping_add(1);
                    self.current_point = point;
                    self.current_subpath_start_point = point;
                }
                Some(QuadraticEvent::LineTo(to)) => {
                    if points_are_sufficiently_far_apart(&self.current_point, &to) {
                        let from = mem::replace(&mut self.current_point, to);
                        return Some((
                            Segment::Line(LineSegment { from, to }),
                            self.current_subpath_index
                        ));
                    }
                }
                Some(QuadraticEvent::QuadraticTo(ctrl, to)) => {
                    if points_are_sufficiently_far_apart(&self.current_point, &to) {
                        let from = mem::replace(&mut self.current_point, to);
                        return Some((
                            Segment::Quadratic(QuadraticBezierSegment { from, ctrl, to }),
                            self.current_subpath_index
                        ));
                    }
                }
                Some(QuadraticEvent::Close) => {
                    let to = self.current_subpath_start_point;
                    if points_are_sufficiently_far_apart(&self.current_point, &to) {
                        let from = mem::replace(&mut self.current_point, to);
                        return Some((
                            Segment::Line(LineSegment { from, to }),
                            self.current_subpath_index
                        ));
                    }
                }
            }
        }

        fn points_are_sufficiently_far_apart(point_a: &Point, point_b: &Point)
                                             -> bool {
            (point_a.x - point_b.x).abs() > 0.001 ||
                (point_a.y - point_b.y).abs() > 0.001
        }
    }
}

