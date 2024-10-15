use crate::vec3f;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

pub struct model {
    verts: Vec<vec3f>,
    faces: Vec<Vec<i32>>
}
impl model {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut verts = Vec::new();
        let mut faces = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            match parts.next() {
                Some("v") => {
                    let x: f32 = parts.next().unwrap().parse()?;
                    let y: f32 = parts.next().unwrap().parse()?;
                    let z: f32 = parts.next().unwrap().parse()?;
                    verts.push(vec3f::new(x, y, z));
                },
                Some("f") => {
                    let mut face = Vec::new();
                    while let Some(idx_str) = parts.next() {
                        // 处理 "v/vt/vn" 或 "v//vn" 格式
                        let v_idx = idx_str.split('/').next().unwrap(); // 仅提取顶点索引
                        let idx: u32 = v_idx.parse()?;  // 解析为整数
                        face.push(idx as i32 - 1); // 将1基索引转换为0基索引
                        
                        // let idx: u32 = idx_str.parse()?;
                        // face.push(idx as i32 - 1);
                        // // 跳过两个额外的整数（顶点的纹理坐标和法线索引）
                        // parts.next(); // 纹理坐标索引
                        // parts.next(); // 法线索引
                    }
                    faces.push(face);
                },
                _ => continue,
            }
        }
        Ok(model{verts, faces})
    }
    pub fn nverts(&self) -> usize {
        return self.verts.len();
    }
    pub fn nfaces(&self) -> usize {
        return self.faces.len();
    }
    pub fn vert(&self, i: usize) -> vec3f {
        return self.verts[i].clone();
    }
    pub fn face(&self, i: usize) -> Vec<i32> {
        return self.faces[i].clone();
    }
}