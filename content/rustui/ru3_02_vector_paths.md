@@lesson ru3-vector-paths Drawing Custom Shapes: Flattening Curves, Fill Rules and Hit Testing
@@concepts numbers, lists, functions
@@lang rust
@@text
Custom widgets (a donut chart, a curved slider, an icon drawn in code) end up as **paths**: lines and Bézier curves. GPU renderers like egui's tessellator, `lyon`, `tiny-skia` and Vello turn paths into triangles or pixels. Two steps happen in all of them, and you'll need both yourself when you write hit testing for a custom shape:
1. **Flattening**: approximating curves with straight segments, within a tolerance.
2. **Filling**: deciding which points are inside, using a **fill rule**.

## Flattening Béziers

- A quadratic Bézier: B(t) = (1−t)²P0 + 2(1−t)t P1 + t² P2.
- Evaluating it at N equal steps is simple but wasteful: flat parts get too many points and tight bends too few.
- **Adaptive subdivision**: if the curve is "flat enough" (the control point is within the tolerance of the chord between the endpoints), emit a line. Otherwise split it at t = 0.5 (de Casteljau: midpoints of midpoints) and recurse.
- Tolerance is in **pixels**: 0.25 px is a common choice. Scale it by the zoom factor so curves stay smooth when zoomed in.

## Fill rules

- **Non-zero** (the default in SVG and most renderers): cast a ray from the point and add +1 when an edge crosses it upwards and −1 downwards. The point is inside if the sum isn't zero.
- **Even-odd**: count the crossings, and the point is inside if the count is odd. This makes holes from overlapping shapes, e.g. a ring drawn as two circles in the same direction.
- A donut with non-zero needs the inner circle drawn in the **opposite direction**.

## Hit testing custom shapes

- Flatten the shape's path to a polygon (once, cached), then use the same ray-crossing test for pointer events.
- Add a **tolerance** so thin shapes are still clickable: test the distance from the point to the nearest segment, or grow the shape.
- Cheap first: check the bounding box, and only then the polygon.

@@quiz
What does flattening a curve mean?
* Approximating it with straight line segments within a tolerance
- Making it horizontal
- Removing its fill
- Converting it to a bitmap
? Renderers and hit tests work on segments.

@@quiz
When does adaptive subdivision stop splitting?
* When the curve segment is flat enough: the control point is within tolerance of the chord
- After exactly 8 splits
- Never
- When t reaches 1
? Tight bends get more segments, flat parts fewer.

@@quiz
Two overlapping circles drawn in the same direction. Which fill rule leaves a hole where they overlap?
- Non-zero
* Even-odd
- Both
- Neither
? Even-odd counts crossings; the overlap has two.

@@quiz
How do you make a donut with the non-zero rule?
* Draw the inner circle in the opposite direction
- Draw the inner circle twice
- Use a thicker stroke
- It's impossible
? The winding numbers cancel inside.

@@quiz
Why should the flattening tolerance scale with zoom?
* So curves stay smooth when zoomed in
- To save memory when zoomed out only
- It shouldn't
- Because tolerance is in centimetres
? Tolerance is in screen pixels.

@@quiz
Why check the bounding box before the polygon in a hit test?
* It rejects most points cheaply
- It is more accurate
- Polygons have no edges
- To support even-odd
? Then run the exact test for the few that remain.

@@flashcards
path :: lines and curves describing a shape
flattening :: curves to line segments
tolerance :: maximum error in pixels, e.g. 0.25
de Casteljau :: split a curve by midpoints of midpoints
adaptive subdivision :: split until flat enough
non-zero :: sum of crossing directions ≠ 0
even-odd :: odd number of crossings
donut with non-zero :: inner path in the opposite direction
ray casting :: count edge crossings from the point
bounding box first :: cheap rejection
stroke tolerance :: make thin shapes clickable

@@drill
With the even-odd rule, a ray from a point crosses 4 edges. Is the point inside? (yes/no)
= no
= No
? Even count means outside.

@@drill
What is the point of a quadratic Bézier at t = 0.5 for P0 = (0, 0), P1 = (10, 20), P2 = (20, 0)? Give the y value.
= 10
? 0.25×0 + 0.5×20 + 0.25×0.

@@exercise Flatten a quadratic Bézier adaptively
Implement `flatten(p0, p1, p2, tolerance)` returning points from p0 to p2. If the distance from p1 to the chord p0–p2 is at most the tolerance, return just the endpoints. Otherwise split with de Casteljau at t = 0.5 and recurse on both halves (without duplicating the shared point).
@@starter
type P = (f64, f64);

fn distance_to_line(p: P, a: P, b: P) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len = (dx * dx + dy * dy).sqrt();
    if len == 0.0 { return ((p.0 - a.0).powi(2) + (p.1 - a.1).powi(2)).sqrt(); }
    ((p.0 - a.0) * dy - (p.1 - a.1) * dx).abs() / len
}

fn flatten(p0: P, p1: P, p2: P, tolerance: f64) -> Vec<P> {
    todo!()
}

fn main() {
    for tol in [4.0, 1.0, 0.25] {
        let pts = flatten((0.0, 0.0), (50.0, 100.0), (100.0, 0.0), tol);
        println!("tolerance {tol}: {} points", pts.len());
    }
}
@@tests
#[test]
fn flat_curve_is_one_segment() {
    assert_eq!(flatten((0.0, 0.0), (5.0, 0.1), (10.0, 0.0), 0.25), vec![(0.0, 0.0), (10.0, 0.0)]);
}

#[test]
fn bends_get_more_points_and_ends_are_kept() {
    let coarse = flatten((0.0, 0.0), (50.0, 100.0), (100.0, 0.0), 4.0);
    let fine = flatten((0.0, 0.0), (50.0, 100.0), (100.0, 0.0), 0.25);
    assert!(fine.len() > coarse.len());
    assert_eq!(fine.first(), Some(&(0.0, 0.0)));
    assert_eq!(fine.last(), Some(&(100.0, 0.0)));
    assert!(fine.windows(2).all(|w| w[0] != w[1]), "no duplicated points");
    assert!(fine.iter().any(|p| (p.0 - 50.0).abs() < 1e-9 && (p.1 - 50.0).abs() < 1e-9), "midpoint B(0.5) = (50, 50)");
}
@@hint
Midpoints: `m01 = mid(p0, p1)`, `m12 = mid(p1, p2)`, `m = mid(m01, m12)`. The left half is (p0, m01, m) and the right half is (m, m12, p2). Recurse on both, then drop the first point of the right result before appending it.
@@solution
type P = (f64, f64);

fn distance_to_line(p: P, a: P, b: P) -> f64 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let len = (dx * dx + dy * dy).sqrt();
    if len == 0.0 { return ((p.0 - a.0).powi(2) + (p.1 - a.1).powi(2)).sqrt(); }
    ((p.0 - a.0) * dy - (p.1 - a.1) * dx).abs() / len
}

fn mid(a: P, b: P) -> P {
    ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0)
}

fn flatten(p0: P, p1: P, p2: P, tolerance: f64) -> Vec<P> {
    if distance_to_line(p1, p0, p2) <= tolerance {
        return vec![p0, p2];
    }
    let (m01, m12) = (mid(p0, p1), mid(p1, p2));
    let m = mid(m01, m12);
    let mut left = flatten(p0, m01, m, tolerance);
    let right = flatten(m, m12, p2, tolerance);
    left.extend_from_slice(&right[1..]);
    left
}

fn main() {
    for tol in [4.0, 1.0, 0.25] {
        let pts = flatten((0.0, 0.0), (50.0, 100.0), (100.0, 0.0), tol);
        println!("tolerance {tol}: {} points", pts.len());
    }
}

@@exercise Point in polygon with both fill rules
Implement `winding(point, polygon)` (the non-zero winding number) and `crossings(point, polygon)` (the number of edges a horizontal ray to the right crosses), then `inside(point, polygons, rule)` for shapes made of several closed polygons, such as a donut.
@@starter
type P = (f64, f64);

#[derive(Clone, Copy)]
enum Rule { NonZero, EvenOdd }

fn winding(p: P, poly: &[P]) -> i32 {
    todo!()
}

fn crossings(p: P, poly: &[P]) -> u32 {
    todo!()
}

fn inside(p: P, polys: &[Vec<P>], rule: Rule) -> bool {
    todo!()
}

fn square(x: f64, y: f64, s: f64, clockwise: bool) -> Vec<P> {
    let pts = vec![(x, y), (x + s, y), (x + s, y + s), (x, y + s)];
    if clockwise { pts } else { pts.into_iter().rev().collect() }
}

fn main() {
    let same = vec![square(0.0, 0.0, 10.0, true), square(3.0, 3.0, 4.0, true)];
    let opposite = vec![square(0.0, 0.0, 10.0, true), square(3.0, 3.0, 4.0, false)];
    let centre = (5.0, 5.0);
    println!("same direction: nonzero {} evenodd {}", inside(centre, &same, Rule::NonZero), inside(centre, &same, Rule::EvenOdd));
    println!("opposite: nonzero {} evenodd {}", inside(centre, &opposite, Rule::NonZero), inside(centre, &opposite, Rule::EvenOdd));
}
@@tests
#[test]
fn single_square() {
    let sq = square(0.0, 0.0, 10.0, true);
    assert_eq!(winding((5.0, 5.0), &sq).abs(), 1);
    assert_eq!(winding((15.0, 5.0), &sq), 0);
    assert_eq!(crossings((5.0, 5.0), &sq), 1);
    assert_eq!(crossings((-5.0, 5.0), &sq), 2);
}

#[test]
fn donut_rules() {
    let same = vec![square(0.0, 0.0, 10.0, true), square(3.0, 3.0, 4.0, true)];
    let opposite = vec![square(0.0, 0.0, 10.0, true), square(3.0, 3.0, 4.0, false)];
    let (centre, ring) = ((5.0, 5.0), (1.0, 5.0));
    assert!(inside(centre, &same, Rule::NonZero), "same direction: centre filled with non-zero");
    assert!(!inside(centre, &same, Rule::EvenOdd), "even-odd makes a hole");
    assert!(!inside(centre, &opposite, Rule::NonZero), "opposite direction: hole with non-zero too");
    assert!(inside(ring, &same, Rule::EvenOdd) && inside(ring, &opposite, Rule::NonZero));
    assert!(!inside((20.0, 20.0), &same, Rule::NonZero));
}
@@hint
Loop over edges `(a, b)` with `b = poly[(i + 1) % n]`. An edge crosses the ray when `(a.1 <= p.1) != (b.1 <= p.1)` and the crossing x, `a.0 + (p.1 - a.1) / (b.1 - a.1) * (b.0 - a.0)`, is greater than `p.0`. Winding adds +1 when the edge goes up (`b.1 > a.1`), otherwise −1. For several polygons, sum the windings or the crossings.
@@solution
type P = (f64, f64);

#[derive(Clone, Copy)]
enum Rule { NonZero, EvenOdd }

fn crossing_edges(p: P, poly: &[P]) -> Vec<i32> {
    let n = poly.len();
    let mut out = Vec::new();
    for i in 0..n {
        let (a, b) = (poly[i], poly[(i + 1) % n]);
        if (a.1 <= p.1) != (b.1 <= p.1) {
            let x = a.0 + (p.1 - a.1) / (b.1 - a.1) * (b.0 - a.0);
            if x > p.0 {
                out.push(if b.1 > a.1 { 1 } else { -1 });
            }
        }
    }
    out
}

fn winding(p: P, poly: &[P]) -> i32 {
    crossing_edges(p, poly).iter().sum()
}

fn crossings(p: P, poly: &[P]) -> u32 {
    crossing_edges(p, poly).len() as u32
}

fn inside(p: P, polys: &[Vec<P>], rule: Rule) -> bool {
    match rule {
        Rule::NonZero => polys.iter().map(|poly| winding(p, poly)).sum::<i32>() != 0,
        Rule::EvenOdd => polys.iter().map(|poly| crossings(p, poly)).sum::<u32>() % 2 == 1,
    }
}

fn square(x: f64, y: f64, s: f64, clockwise: bool) -> Vec<P> {
    let pts = vec![(x, y), (x + s, y), (x + s, y + s), (x, y + s)];
    if clockwise { pts } else { pts.into_iter().rev().collect() }
}

fn main() {
    let same = vec![square(0.0, 0.0, 10.0, true), square(3.0, 3.0, 4.0, true)];
    let opposite = vec![square(0.0, 0.0, 10.0, true), square(3.0, 3.0, 4.0, false)];
    let centre = (5.0, 5.0);
    println!("same direction: nonzero {} evenodd {}", inside(centre, &same, Rule::NonZero), inside(centre, &same, Rule::EvenOdd));
    println!("opposite: nonzero {} evenodd {}", inside(centre, &opposite, Rule::NonZero), inside(centre, &opposite, Rule::EvenOdd));
}
