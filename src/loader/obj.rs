use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;

use super::*;
use crate::geometry::*;
use crate::shape::*;


pub fn load_from_file(filename: &str, to_world: T) -> Res<Mesh> {
    let f = File::open(filename).with_msg("Error opening OBJ file")?;
    ObjLoader::new(to_world).load(BufReader::new(f))
}

struct ObjLoader {
    tmp_data: MeshData,
    obj_data: MeshData,
    faces: Vec<Face>,
    vertex_map: HashMap<Vertex, I>,
    to_world: T,
}

#[derive(Eq, Hash, PartialEq)]
struct Vertex {
    p: I,
    t: I,
    n: I,
}

impl ObjLoader {
    fn new(to_world: T) -> ObjLoader {
        ObjLoader {
            tmp_data: MeshData::new(),
            obj_data: MeshData::new(),
            faces: Vec::new(),
            vertex_map: HashMap::new(),
            to_world,
        }
    }

    fn load(mut self, mut buf: impl BufRead) -> Res<Mesh> {
        let mut line = String::with_capacity(120);
        while buf.read_line(&mut line).with_msg("Error reading line")? > 0 {
            let mut tokens = line[..].split_whitespace();

            match tokens.next() {
                Some("v")  => self.add_point(&mut tokens),
                Some("vt") => self.add_uv(&mut tokens),
                Some("vn") => self.add_normal(&mut tokens),
                Some("f")  => self.add_face(&mut tokens),
                _ => Ok(()),
            }?;

            line.clear();
        }

        let mesh_data = Arc::new(self.obj_data);
        let triangles = self.faces.into_iter().map(|f| Triangle {
            f, mesh_data: mesh_data.clone(),
        }).collect();

        Ok(Mesh::new(triangles))
    }

    fn add_point<'a>(&mut self, tokens: &mut impl Iterator<Item=&'a str>)
            -> Res<()> {
        self.tmp_data.p.push(self.to_world * P(parse_f3(tokens)?));
        Ok(())
    }

    fn add_uv<'a>(&mut self, tokens: &mut impl Iterator<Item=&'a str>)
            -> Res<()> {
        self.tmp_data.uv.push(parse_f2(tokens)?);
        Ok(())
    }

    fn add_normal<'a>(&mut self, tokens: &mut impl Iterator<Item=&'a str>)
            -> Res<()> {
        self.tmp_data.n.push(self.to_world * N::a3(parse_f3(tokens)?));
        Ok(())
    }

    fn add_face<'a>(&mut self, tokens: &mut impl Iterator<Item=&'a str>)
            -> Res<()> {
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
            3 => self.faces.push(Face(v[0], v[1], v[2])),
            4 => {
                self.faces.push(Face(v[0], v[1], v[2]));
                self.faces.push(Face(v[0], v[2], v[3]));
            },
            _ => return Err("unexpected number of vertices".into()),
        }
        Ok(())
    }
    
    fn add_vertex(&mut self, v: Vertex) -> I {
        self.obj_data.p.push(self.tmp_data.p[v.p as usize]);
        if v.t != -1 { self.obj_data.uv.push(self.tmp_data.uv[v.t as usize]); }
        if v.n != -1 { self.obj_data.n.push(self.tmp_data.n[v.n as usize]); }
        let n = self.vertex_map.len() as I;
        self.vertex_map.insert(v, n);
        n
    }

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

fn parse_index<'a>(tkns: &mut impl Iterator<Item=&'a str>, n: usize) -> Res<I>
{ parse(tkns).map(|i: I| if i > 0 { i - 1 } else { i + n as I }) }

fn parse_f3<'a>(tokens: &mut impl Iterator<Item=&'a str>) -> Res<F3>
{ Ok(A3(parse(tokens)?, parse(tokens)?, parse(tokens)?)) }

fn parse_f2<'a>(tokens: &mut impl Iterator<Item=&'a str>) -> Res<F2>
{ Ok(A2(parse(tokens)?, parse(tokens)?)) }

fn parse<'a, S>(tokens: &mut impl Iterator<Item=&'a str>) -> Res<S>
        where S: FromStr,
              <S as FromStr>::Err: Display {
    tokens.next().ok_or("missing scalar")?
          .parse::<S>().with_msg("malformed scalar")
}
