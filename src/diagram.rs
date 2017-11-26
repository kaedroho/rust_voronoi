use std::slice;

use point::Point;
use dcel::{DCEL, Face};

/// Represents a generated Voronoi diagram
#[derive(Debug)]
pub struct VoronoiDiagram {
    /// The internal Doubly Connected Edge List
    pub dcel: DCEL,

    /// The ID of the face that represents the outside of the Voronoi diagram
    outside_face_id: usize,
}

impl VoronoiDiagram {
    /// Constructs a VoronoiDiagram from a Doubly Connected Edge List
    pub fn from_dcel(dcel: DCEL) -> VoronoiDiagram {
        // Work out outside_face_id by finding the face with the most edges
        // FIXME: This feels hacky and there might be cases where this gets the wrong face
        let mut highest_edges_count = 0;
        let mut highest_edges_face = 0;
        for (i, face) in dcel.faces.iter().enumerate() {
            if !face.alive { continue; }
            let mut num_edges = 0;
            let start_edge = face.outer_component;
            let mut current_edge = start_edge;
            loop {
                num_edges += 1;
                current_edge = dcel.halfedges[current_edge].next;
                if current_edge == start_edge { break; }
            }

            if num_edges > highest_edges_count {
                highest_edges_count = num_edges;
                highest_edges_face = i;
            }
        }

        VoronoiDiagram {
            dcel: dcel,
            outside_face_id: highest_edges_face,
        }
    }

    /// Returns an iterator over the cells in the diagram
    pub fn cells<'a>(&'a self) -> VoronoiCellsIterator<'a> {
        VoronoiCellsIterator {
            diagram: &self,
            faces_iter: self.dcel.faces.iter().enumerate(),
        }
    }
}

/// Represents a cell in a Voronoi diagram
#[derive(Debug)]
pub struct VoronoiCell<'a> {
    dcel: &'a DCEL,
    face_id: usize,
}

impl<'a> VoronoiCell<'a> {
    /// Returns a list of points that represent the border of this cell
    pub fn points(&self) -> Vec<Point> {
        let face = &self.dcel.faces[self.face_id];
        let mut points = vec![];

        let start_edge = face.outer_component;
        let mut current_edge = start_edge;
        loop {
            points.push(self.dcel.get_origin(current_edge));
            current_edge = self.dcel.halfedges[current_edge].next;
            if current_edge == start_edge { break; }
        }

        points
    }

    /// Calculates the centroid of the cell
    pub fn centroid(&self) -> Point {
        let points = self.points();

        let mut sum = Point::new(0.0, 0.0);
        for pt in self.points() {
            sum = pt + sum;
        }
        sum * (1.0 / (points.len() as f64))
    }
}

#[derive(Debug)]
pub struct VoronoiCellsIterator<'a> {
    diagram: &'a VoronoiDiagram,
    faces_iter: ::std::iter::Enumerate<slice::Iter<'a, Face>>,
}

impl<'a> Iterator for VoronoiCellsIterator<'a> {
    type Item = VoronoiCell<'a>;

    fn next(&mut self) -> Option<VoronoiCell<'a>> {
        while let Some((i, face)) = self.faces_iter.next() {
            if face.alive && i != self.diagram.outside_face_id {
                return Some(VoronoiCell {
                    dcel: &self.diagram.dcel,
                    face_id: i,
                });
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use voronoi::voronoi;

    use super::*;

    #[test]
    fn test_cells_iterator() {
        let vor_pts = vec![Point::new(0.0, 1.0), Point::new(2.0, 3.0), Point::new(10.0, 12.0)];
        let vor_diagram = voronoi(vor_pts, 800.);
        assert_eq!(vor_diagram.cells().count(), 3);
    }

    #[test]
    fn test_cells_points() {
        let vor_pts = vec![Point::new(0.0, 1.0), Point::new(2.0, 3.0), Point::new(10.0, 12.0)];
        let vor_diagram = voronoi(vor_pts, 800.);
        assert_eq!(vor_diagram.cells().nth(0).unwrap().points().len(), 5);
    }
}
