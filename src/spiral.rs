
use cgmath::Vector2;
use cgmath::vec2;

#[derive(Clone)]
pub struct Spiral{
    point:[f64;2],
    rad:f64,
    start:f64,
    rate:f64,
    width:f64
}

pub struct SpiralInt(Spiral);
impl Iterator for SpiralInt{
    type Item=Vector2<isize>;
    fn next(&mut self)->Option<Vector2<isize>>{
        self.0.next().map(|a|a.cast().unwrap())
    }
}
impl std::iter::FusedIterator for SpiralInt{}

pub struct SpiralF32(Spiral);
impl Iterator for SpiralF32{
    type Item=Vector2<f32>;
    fn next(&mut self)->Option<Vector2<f32>>{
        self.0.next().map(|a|a.cast().unwrap())
    }
}
impl std::iter::FusedIterator for SpiralF32{}



impl Spiral{
    pub fn new(point:[f64;2],circular_grow:f64,outward_grow:f64)->Spiral{
        Spiral{point,rad:0.0,start:1.0,rate:outward_grow,width:circular_grow}
    }
    pub fn get_circular_grow(&self)->f64{
        self.width
    }
    pub fn get_outward_grow(&self)->f64{
        self.rate
    }
    pub fn as_isize(self)->SpiralInt{
        SpiralInt(self)
    }
    pub fn as_f32(self)->SpiralF32{
        SpiralF32(self)
    }
}

impl std::iter::FusedIterator for Spiral{}

impl Iterator for Spiral{
    type Item=Vector2<f64>;
    fn next(&mut self)->Option<Vector2<f64>>{
        
        let length=self.start+self.rate*self.rad;

        let x=self.point[0]+self.rad.cos()*length;
        let y=self.point[1]+self.rad.sin()*length;

        self.rad+=self.width/length;

        Some(vec2(x,y))

    }
}

use crate::Dist2;

impl Dist2<f64> for Spiral{}
impl Dist2<f32> for SpiralF32{}
impl Dist2<isize> for SpiralInt{}