#![allow(deprecated)]
use axgeom::vec2;
use axgeom::Rect;
use axgeom::Vec2;

use axgeom;

use core::iter::FusedIterator;

/*
pub struct RangeGenIterf32{
    max:usize,
    counter:usize,
    rng:rand::StdRng,
    xvaluegen:UniformRangeGenerator,
    yvaluegen:UniformRangeGenerator,
    radiusgen:UniformRangeGenerator,
    velocity_dir:UniformRangeGenerator,
    velocity_mag:UniformRangeGenerator
}

pub struct Retf32{
    pub id:usize,
    pub pos:[f32;2],
    pub vel:[f32;2],
    pub radius:[f32;2],
}

pub struct RetInteger{
    pub id:usize,
    pub pos:[isize;2],
    pub vel:[isize;2],
    pub radius:[isize;2],
}
impl Retf32{
    pub fn into_isize(self)->RetInteger{
        let id=self.id;
        let pos=[self.pos[0] as isize,self.pos[1] as isize];
        let vel=[self.vel[0] as isize,self.vel[1] as isize];
        let radius=[self.radius[0] as isize,self.radius[1] as isize];
        RetInteger{id,pos,vel,radius}
    }
}
impl std::iter::FusedIterator for RangeGenIterf32{}
impl ExactSizeIterator for RangeGenIterf32{}
impl Iterator for RangeGenIterf32{
    type Item=Retf32;
    fn size_hint(&self)->(usize,Option<usize>){
        (self.max,Some(self.max))
    }
    fn next(&mut self)->Option<Self::Item>{

        if self.counter==self.max{
            return None
        }

        let rng=&mut self.rng;
        let px=self.xvaluegen.get(rng) as f32;
        let py=self.yvaluegen.get(rng) as f32;
        let rx=self.radiusgen.get(rng) as f32;
        let ry=self.radiusgen.get(rng) as f32;

        let (velx,vely)={
            let vel_dir=self.velocity_dir.get(rng) as f32;
            let vel_dir=vel_dir.to_radians();
            let (mut xval,mut yval)=(vel_dir.cos(),vel_dir.sin());
            let vel_mag=self.velocity_mag.get(rng) as f32;
            xval*=vel_mag;
            yval*=vel_mag;
            (xval,yval)
        };

        let curr=self.counter;
        self.counter+=1;
        let r=Retf32{id:curr,pos:[px,py],vel:[velx,vely],radius:[rx,ry]};
        Some(r)
    }
}
pub fn create_world_generator(num:usize,area:&[isize;4],radius:[isize;2],velocity:[isize;2])->RangeGenIterf32{
    let arr:&[usize]=&[100,42,6];
    let rng =  SeedableRng::from_seed(arr);


    let xvaluegen=UniformRangeGenerator::new(area[0],area[1]);
    let yvaluegen=UniformRangeGenerator::new(area[2],area[3]);
    let radiusgen= UniformRangeGenerator::new(radius[0],radius[1]);


    let velocity_dir=UniformRangeGenerator::new(0,360);
    let velocity_mag= UniformRangeGenerator::new(velocity[0],velocity[1]);

    RangeGenIterf32{max:num,counter:0,rng,xvaluegen,yvaluegen,radiusgen,velocity_dir,velocity_mag}
}
*/

use rand::prelude::*;

use crate::RadiusGen;
use crate::RadiusGenInt;

pub struct UniformRandGen {
    area: Rect<f32>,
    rng: ThreadRng,
}

impl UniformRandGen {
    #[deprecated(
        since = "0.3.1",
        note = "use rand_iter() instead"
    )]
    pub fn new(area: Rect<f32>) -> UniformRandGen {
        let rng = rand::thread_rng();
        UniformRandGen { area, rng }
    }
    #[deprecated(
        since = "0.3.1",
        note = "use rand_iter() instead"
    )]
    pub fn with_radius(self, min: f32, max: f32) -> core::iter::Zip<UniformRandGen, RadiusGen> {
        self.zip(RadiusGen::new(vec2(min, min), vec2(max, max)))
    }

    pub fn with_int(self) -> UniformRandGenInt {
        UniformRandGenInt(self)
    }
}




pub struct UniformRandGenInt(UniformRandGen);
impl UniformRandGenInt {
    #[deprecated(
        since = "0.3.1",
        note = "use rand_iter() instead"
    )]
    pub fn with_radius(
        self,
        min: i32,
        max: i32,
    ) -> core::iter::Zip<UniformRandGenInt, RadiusGenInt> {
        self.zip(RadiusGenInt::new(vec2(min, min), vec2(max, max)))
    }
}

impl Iterator for UniformRandGen {
    type Item = Vec2<f32>;
    fn next(&mut self) -> Option<Vec2<f32>> {
        let rng = &mut self.rng;
        let area = &self.area;
        let x: f32 = rng.gen::<f32>() * (area.x.end - area.x.start); // generates a float between 0 and 1
        let y: f32 = rng.gen::<f32>() * (area.y.end - area.y.start);
        Some(vec2(x, y))
    }
}
impl FusedIterator for UniformRandGen {}

//impl Dist<f32> for UniformRandGen{}

impl Iterator for UniformRandGenInt {
    type Item = Vec2<i32>;
    fn next(&mut self) -> Option<Vec2<i32>> {
        self.0.next().map(|a| vec2(a.x as i32, a.y as i32))
    }
}
impl FusedIterator for UniformRandGenInt {}

//impl Dist<i32> for UniformRandGenInt{}

/*

struct UniformRangeGenerator{
    range:Range<isize>
}

impl UniformRangeGenerator{
    pub fn new(a:isize,b:isize)->Self{
        //let rr = a.get_range2::<axgeom::XAXISS>();
        let xdist = rand::distributions::Range::new(a,b);
        UniformRangeGenerator{range:xdist}
    }
    pub fn get(&self,rng:&mut StdRng)->isize{
        self.range.ind_sample(rng)
    }
}

*/
