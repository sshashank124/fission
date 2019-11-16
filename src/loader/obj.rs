use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::sync::Arc;

use crate::geometry::*;
use crate::shape::*;
use crate::util::*;


pub fn load_from_file(filename: &str) -> Res<Mesh> {
    let f = File::open(filename).with_msg("Error opening OBJ file")?;
    ObjLoader::new().load(BufReader::new(f))
}

struct ObjLoader {
    tmp_data: MeshData,
    obj_data: MeshData,
    faces: Vec<Face>,
    vertex_map: HashMap<Vertex, I>,
}

#[derive(Eq, Hash, PartialEq)]
struct Vertex {
    p: I,
    t: I,
    n: I,
}

impl ObjLoader {
    #[inline(always)]
    fn new() -> ObjLoader {
        ObjLoader {
            tmp_data: MeshData::new(),
            obj_data: MeshData::new(),
            faces: Vec::new(),
            vertex_map: HashMap::new(),
        }
    }

    fn load<B>(mut self, mut buf: B) -> Res<Mesh> where B: BufRead {
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

    #[inline(always)]
    fn add_point<'a, It>(&mut self, tokens: &mut It) -> Res<()>
            where It: Iterator<Item=&'a str> {
        self.tmp_data.p.push(P(parse_f3(tokens)?));
        Ok(())
    }

    #[inline(always)]
    fn add_uv<'a, It>(&mut self, tokens: &mut It) -> Res<()>
            where It: Iterator<Item=&'a str> {
        self.tmp_data.uv.push(parse_f2(tokens)?);
        Ok(())
    }

    #[inline(always)]
    fn add_normal<'a, It>(&mut self, tokens: &mut It) -> Res<()>
            where It: Iterator<Item=&'a str> {
        self.tmp_data.n.push(N(V(parse_f3(tokens)?).unit()));
        Ok(())
    }

    #[inline(always)]
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
            3 => self.faces.push(Face::new(v[0], v[1], v[2])),
            4 => {
                self.faces.push(Face::new(v[0], v[1], v[2]));
                self.faces.push(Face::new(v[0], v[2], v[3]));
            },
            _ => return Err("unexpected number of vertices".into()),
        }
        Ok(())
    }
    
    #[inline(always)]
    fn add_vertex(&mut self, v: Vertex) -> I {
        self.obj_data.p.push(self.tmp_data.p[v.p as usize]);
        if v.t != -1 { self.obj_data.uv.push(self.tmp_data.uv[v.t as usize]); }
        if v.n != -1 { self.obj_data.n.push(self.tmp_data.n[v.n as usize]); }
        let n = self.vertex_map.len() as I;
        self.vertex_map.insert(v, n);
        n
    }

    #[inline(always)]
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

#[inline(always)]
fn parse_index<'a, It>(tokens: &mut It, n: usize) -> Res<I>
        where It: Iterator<Item=&'a str> {
    parse(tokens).map(|i: I| if i > 0 { i - 1 } else { i + n as I })
}

#[inline(always)]
fn parse_f3<'a, It>(tokens: &mut It) -> Res<F3>
        where It: Iterator<Item=&'a str> {
    Ok(A3(parse(tokens)?, parse(tokens)?, parse(tokens)?))
}

#[inline(always)]
fn parse_f2<'a, It>(tokens: &mut It) -> Res<F2>
        where It: Iterator<Item=&'a str> {
    Ok(A2(parse(tokens)?, parse(tokens)?))
}

#[inline(always)]
fn parse<'a, S, It>(tokens: &mut It) -> Res<S>
        where It: Iterator<Item=&'a str>,
              S: FromStr,
              <S as FromStr>::Err: Display {
    tokens.next().ok_or("missing scalar")?
          .parse::<S>().with_msg("malformed scalar")
}


#[cfg(test)]
mod benches {
    extern crate test;

    use super::*;
    use test::Bencher;

    macro_rules! bench_obj {
        ($name: ident, $file: expr) => {
            #[bench]
            fn $name(b: &mut Bencher) {
                b.iter(|| {
                    load_from_file($file, T::ONE);
                });
            }
        }
    }

    bench_obj!(plane, "obj/plane.obj");
    bench_obj!(disk, "obj/disk.obj");
    bench_obj!(teapot, "obj/teapot.obj");
    bench_obj!(sphere, "obj/sphere.obj");
    bench_obj!(camelhead, "obj/camelhead.obj");
    bench_obj!(sponza, "obj/sponza.obj");
    bench_obj!(ajax, "obj/ajax.obj");
}
