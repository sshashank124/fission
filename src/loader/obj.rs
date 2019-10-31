use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::rc::Rc;
use std::str::FromStr;

use crate::geometry::*;
use crate::structure::mesh::{Face, Mesh, MeshData, Triangle};
use crate::util::*;


pub fn load_from_file(filename: &str) -> Res<Mesh> {
    let f = File::open(filename).with_msg("Error opening OBJ file")?;
    ObjLoader::new().load(BufReader::new(f).lines())
}

struct ObjLoader {
    tmp_data: MeshData,
    obj_data: MeshData,
    faces: Vec<Face>,
    vertex_map: HashMap<Vertex, I>,
}

#[derive(Eq, Hash, PartialEq)]
struct Vertex {
    p: i32,
    t: i32,
    n: i32,
}

impl ObjLoader {
    #[inline]
    fn new() -> ObjLoader {
        ObjLoader {
            tmp_data: MeshData::new(),
            obj_data: MeshData::new(),
            faces: Vec::new(),
            vertex_map: HashMap::new(),
        }
    }

    fn load<S>(mut self, supplier: S) -> Res<Mesh>
            where S: Iterator<Item=Result<String, io::Error>> {
        for line_result in supplier {
            let line = line_result.with_msg("Error reading line")?;
            let mut tokens = line[..].split_whitespace();

            match tokens.next() {
                Some("v")  => self.add_point(&mut tokens),
                Some("vt") => self.add_uv(&mut tokens),
                Some("vn") => self.add_normal(&mut tokens),
                Some("f")  => self.add_face(&mut tokens),
                _ => Ok(()),
            }?
        }

        let mesh_data = Rc::new(self.obj_data);
        let triangles = self.faces.into_iter().map(|f| Triangle {
            f, mesh_data: mesh_data.clone(),
        }).collect();

        Ok(triangles)
    }

    #[inline]
    fn add_point<'a, It>(&mut self, tokens: &mut It) -> Res<()>
            where It: Iterator<Item=&'a str> {
        Ok(self.tmp_data.p.push(P(parse_f3(tokens)?)))
    }

    #[inline]
    fn add_uv<'a, It>(&mut self, tokens: &mut It) -> Res<()>
            where It: Iterator<Item=&'a str> {
        Ok(self.tmp_data.uv.push(parse_f2(tokens)?))
    }

    #[inline]
    fn add_normal<'a, It>(&mut self, tokens: &mut It) -> Res<()>
            where It: Iterator<Item=&'a str> {
        Ok(self.tmp_data.n.push(N(V(parse_f3(tokens)?).unit())))
    }

    #[inline]
    fn add_face<'a, It>(&mut self, tokens: &mut It) -> Res<()>
            where It: Iterator<Item=&'a str> {
        let vertices: Result<Vec<I>, _> = tokens.map(|st| {
            match self.parse_vertex(st) {
                Ok(v) => match self.vertex_map.get(&v) {
                    Some(&i) => Ok(i),
                    None => Ok(self.add_vertex(v)),
                },
                Err(e) => Err(e),
            }
        }).collect();
        let v = vertices?;

        match v.len() {
            3 => Ok(self.faces.push(Face::new(v[0], v[1], v[2]))),
            4 => Ok({
                self.faces.push(Face::new(v[0], v[1], v[2]));
                self.faces.push(Face::new(v[0], v[2], v[3]));
            }),
            _ => Err("unexpected number of vertices".into()),
        }
    }
    
    #[inline]
    fn add_vertex(&mut self, v: Vertex) -> I {
        self.obj_data.p.push(self.tmp_data.p[v.p as usize]);
        if v.t != -1 { self.obj_data.uv.push(self.tmp_data.uv[v.t as usize]); }
        if v.n != -1 { self.obj_data.n.push(self.tmp_data.n[v.n as usize]); }
        let n = self.vertex_map.len() as I;
        self.vertex_map.insert(v, n);
        n
    }

    #[inline]
    fn parse_vertex(&mut self, token: &str) -> Res<Vertex> {
        let mut tokens = token.split('/');
        Ok(Vertex {
            p: parse_index(&mut tokens, self.tmp_data.p.len())
                    .with_msg("index for position is required")?,
            t: parse_index(&mut tokens, self.tmp_data.uv.len()).unwrap_or(-1),
            n: parse_index(&mut tokens, self.tmp_data.n.len()).unwrap_or(-1),
        })
    }
}

#[inline]
fn parse_index<'a, It>(tokens: &mut It, n: usize) -> Res<i32>
        where It: Iterator<Item=&'a str> {
    parse(tokens).map(|i: i32| if i > 0 { i - 1 } else { i + n as i32 })
}

#[inline]
fn parse_f3<'a, It>(tokens: &mut It) -> Res<F3>
        where It: Iterator<Item=&'a str> {
    Ok(A3(parse(tokens)?, parse(tokens)?, parse(tokens)?))
}

#[inline]
fn parse_f2<'a, It>(tokens: &mut It) -> Res<F2>
        where It: Iterator<Item=&'a str> {
    Ok(P2(parse(tokens)?, parse(tokens)?))
}

#[inline]
fn parse<'a, S, It>(tokens: &mut It) -> Res<S>
        where It: Iterator<Item=&'a str>,
              S: FromStr,
              <S as FromStr>::Err: Display {
    tokens.next().ok_or("missing scalar")?
          .parse::<S>().with_msg("malformed scalar")
}
