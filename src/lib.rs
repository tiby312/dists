//!
//! Provides a way to generate different 2d distributions of bots, such as a spiral, or a uniform random distribution.
//!

use axgeom::*;

use core::iter::FusedIterator;
use rand::prelude::*;

///Produces grid distributions
pub mod grid;

///Produces an archimedean spiral distribution
pub mod spiral;

///Produces a random distribution over a rectangular area
pub mod uniform_rand;

pub fn grid_rect_iter(
    num_bots: usize,
    rect: Rect<f32>,
) -> impl Iterator<Item = [f32; 2]> + Clone + Send + Sync {
    let width = rect.x.end - rect.x.start;
    let height = rect.y.end - rect.y.start;

    let aspect_ratio = width / height;

    //w*h=num_bots
    //w/h=width/height
    //solve for sx and sy

    //w=num_bots/h
    //num_bots/h^2=width/height
    //h^2=num_bots/(width/height);
    //h=sqrt(num_bots/(width/height));

    let h = (num_bots as f32 / aspect_ratio).sqrt().ceil() as usize;
    let w = num_bots / h;

    let grid_dim = [w, h];
    let spacing = vec2(width / w as f32, height / h as f32);

    let topstart = vec2(rect.x.start, rect.y.start);
    grid_iter(grid_dim).map(move |[x, y]| {
        let v = topstart + vec2(x, y).inner_as().scale(spacing);
        [v.x, v.y]
    })
}

//TODO use
pub fn grid_iter(dim: [usize; 2]) -> impl Iterator<Item = [usize; 2]> + Clone + Send + Sync {
    let mut xcounter = 0;
    let mut ycounter = 0;
    core::iter::from_fn(move || {
        if ycounter >= dim[1] {
            None
        } else {
            if xcounter >= dim[0] {
                xcounter = 0;
                ycounter += 1;
            }

            let c = [xcounter, ycounter];

            xcounter += 1;

            Some(c)
        }
    })
}

pub fn fib_iter(point: [f64; 2], out_incr: f64) -> impl Iterator<Item = [f64; 2]> {
    const PHI: f64 = 1.6180339887498948482;

    //
    //     x        PHI
    //  -----   =  -----
    //   TAU         1
    //
    //const INCR:f64=PHI*std::f64::consts::TAU;

    let mut counter = 0;
    core::iter::repeat_with(move || {
        let l = out_incr * (counter as f64).sqrt();
        let rad = (std::f64::consts::TAU / (PHI * PHI)) * (counter as f64);
        let x = point[0] + (rad.cos() * l);
        let y = point[1] + (rad.sin() * l);
        counter += 1;
        [x, y]
    })
}

pub fn spiral_iter(
    point: [f64; 2],
    circular_grow: f64,
    outward_grow: f64,
) -> impl Iterator<Item = [f64; 2]> + Clone + FusedIterator {
    let start = 1.0;
    let rate = outward_grow;
    let mut rad = 0.0;
    let width = circular_grow;

    core::iter::repeat_with(move || {
        let length = start + rate * rad;

        let x = point[0] + rad.cos() * length;
        let y = point[1] + rad.sin() * length;

        rad += width / length;

        [x, y]
    })
}

/*
///Every distribution implements this.
pub trait Dist<K>:Iterator<Item=Vec2<K>>+FusedIterator{}
*/

/*
use core::marker::PhantomData;

pub struct ConstantAabbAdapter<K,I>{
    a:I,
    _p:PhantomData<K>,
    radius:K
}

impl<K,I> ConstantAabbAdapter<K,I>{
    pub fn new(radius:K,a:I)->Self{
        ConstantAabbAdapter{a,radius,_p:PhantomData}
    }
}

impl<K:Clone,I:Clone> Clone for ConstantAabbAdapter<K,I>{
    fn clone(&self)->Self{
        ConstantAabbAdapter::new(self.radius.clone(),self.a.clone())
    }
}

use core::ops::*;

impl<K:Add<Output=K>+Sub<Output=K>+Copy,I:Iterator<Item=Vec2<K>>> Iterator for ConstantAabbAdapter<K,I>{
    type Item=Rect<K>;
    fn next(&mut self)->Option<Self::Item>{
        self.a.next().map(|a|{
            let r=self.radius;
            Rect::new(a.x-r,a.x+r,a.y-r,a.y+r)
        })
    }
}
*/

pub fn rand2_iter(rect: Rect<f32>) -> impl Iterator<Item = [f32; 2]> + FusedIterator + Clone {
    rand_iter(rect.x.start, rect.x.end)
        .zip(rand_iter(rect.y.start, rect.y.end))
        .map(|(x, y)| [x, y])
}

pub fn rand_iter(min: f32, max: f32) -> impl Iterator<Item = f32> + FusedIterator + Clone {
    let mut rng = rand::thread_rng();

    core::iter::repeat_with(move || min + rng.gen::<f32>() * (max - min))
}

///Randomly generates radiuses.
pub struct RadiusGen {
    min: Vec2<f32>,
    max: Vec2<f32>,
    rng: ThreadRng,
}
impl RadiusGen {
    #[deprecated(since = "0.3.1", note = "use rand_iter() instead")]
    pub fn new(min_radius: Vec2<f32>, max_radius: Vec2<f32>) -> RadiusGen {
        let rng = rand::thread_rng();
        RadiusGen {
            min: min_radius,
            max: max_radius,
            rng,
        }
    }
}
impl Iterator for RadiusGen {
    type Item = Vec2<f32>;
    fn next(&mut self) -> Option<Vec2<f32>> {
        let x = self.min.x + self.rng.gen::<f32>() * (self.max.x - self.min.x);
        let y = self.min.y + self.rng.gen::<f32>() * (self.max.y - self.min.y);
        Some(vec2(x, y))
    }
}
impl FusedIterator for RadiusGen {}

///A wrapper around a RadiusGen that produced integers
pub struct RadiusGenInt(RadiusGen);
impl RadiusGenInt {
    pub fn new(min_radius: Vec2<i32>, max_radius: Vec2<i32>) -> RadiusGenInt {
        let rng = rand::thread_rng();
        RadiusGenInt(RadiusGen {
            min: vec2(min_radius.x as f32, min_radius.y as f32),
            max: vec2(max_radius.x as f32, max_radius.y as f32),
            rng,
        })
    }
}
impl Iterator for RadiusGenInt {
    type Item = Vec2<i32>;
    fn next(&mut self) -> Option<Vec2<i32>> {
        self.0.next().map(|a| vec2(a.x as i32, a.y as i32))
    }
}
impl FusedIterator for RadiusGenInt {}

//TODO add more distributions.
/*
fn test_bot_layout(mut bots: Vec<BBox<isize, Bot>>) {
    let mut control_result = {
        let mut src: Vec<(usize, usize)> = Vec::new();

        let control_bots = bots.clone();
        for (i, el1) in control_bots.iter().enumerate() {
            for el2 in control_bots[i + 1..].iter() {
                let a = el1;
                let b = el2;
                let ax = (a.get().0).0.get_range2::<XAXISS>();
                let ay = (a.get().0).0.get_range2::<YAXISS>();
                let bx = (b.get().0).0.get_range2::<XAXISS>();
                let by = (b.get().0).0.get_range2::<YAXISS>();

                if ax.intersects(bx) && ay.intersects(by) {
                    src.push(test_support::create_unordered(&a.val, &b.val));
                }
            }
        }
        src
    };

    let mut test_result = {
        let mut src: Vec<(usize, usize)> = Vec::new();

        {
            let mut dyntree = DinoTree::new_seq(&mut bots,  StartAxis::Xaxis);

            let clos = |a: ColSingle<BBox<isize, Bot>>, b: ColSingle<BBox<isize, Bot>>| {
                //let (a,b)=(ca,ca.1);
                //let a=ca[0];
                //let b=ca[1];
                src.push(test_support::create_unordered(&a.inner, &b.inner));
            };

            dyntree.intersect_every_pair_seq(clos);
        }

        src
    };

    control_result.sort_by(&test_support::compair_bot_pair);
    test_result.sort_by(&test_support::compair_bot_pair);

    println!(
        "control length={} test length={}",
        control_result.len(),
        test_result.len()
    );
    {
        use std::collections::HashSet;
        println!(
            "control vs test len={:?}",
            (control_result.len(), test_result.len())
        );

        let mut control_hash = HashSet::new();
        for k in control_result.iter() {
            control_hash.insert(k);
        }

        let mut test_hash = HashSet::new();
        for k in test_result.iter() {
            test_hash.insert(k);
        }

        let diff = control_hash
            .symmetric_difference(&test_hash)
            .collect::<Vec<_>>();

        if diff.len() != 0 {
            //println!("diff={:?}",diff);

            //println!("first={:?}",(&bots[diff[0].0],&bots[diff[0].1]));
            //let bots_copy = bots.clone();


            let mut dyntree = DinoTree::new(&mut bots, StartAxis::Xaxis);

            for i in diff.iter(){
                let id1=i.0;
                let id2=i.1;
                println!("------------------");
                println!("{:?}",dyntree.find_element(|bla|{bla.val.id==id1}));
                println!("{:?}",dyntree.find_element(|bla|{bla.val.id==id2}));

                println!("------------------");
            }
            //use compt::CTreeIterator;
            /*
            for i in diff.iter(){
                let level=dyntree.0.get_level_desc();
                let first={
                  let dd=dyntree.0.get_iter_mut();
                  let ll=compt::LevelIter::new(dd,level);
                  let mut first=None;
                  'bla:for (level,n) in ll.dfs_preorder_iter(){
                     for bot in n.range.iter(){
                        if bot.get().1.id==i.0{
                          first=Some(level.get_depth());
                          break 'bla;
                        }
                     }
                  }
                  first
                };

                let second={
                  let dd=dyntree.0.get_iter_mut();
                  let ll=compt::LevelIter::new(dd,level);

                  let mut second=None;
                  'bla2:for (level,n) in ll.dfs_preorder_iter(){
                     for bot in n.range.iter(){
                        if bot.get().1.id==i.1{
                          second=Some(level.get_depth());
                          break 'bla2;
                        }
                     }
                  }
                  second
                };

                println!("debug={:?}",(first,second));

                let first_bot=bots_copy.iter().find(|a|a.get().1.id==i.0).unwrap();
                let second_bot=bots_copy.iter().find(|a|a.get().1.id==i.1).unwrap();
                println!("{:?}",(first_bot.get().0,second_bot.get().0));
            }*/
        }

        assert!(diff.len() == 0);
    }
}

*/

/*

#[test]
fn test_bounding_boxes_as_points() {


    let mut bots=create_bots_isize(|id|Bot{id,col:Vec::new()},&[0,800,0,800],500,[2,3]);
    /*
    let spawn_world = make_rect((-990, 990), (-90, 90));

    let mut p = PointGenerator::new(&spawn_world, &[1, 2, 3, 4, 5]);

    let bots: Vec<BBox<isize, Bot>> = {
        (0..2000)
            .map(|id| {
                let p=p.random_point();
                let rect = AABBox::new((p.0,p.0),(p.1,p.1));
                BBox::new(
                    Bot {
                        id,
                        col: Vec::new(),
                    },
                    rect,
                )
            })
            .collect()
    };
    */

    test_bot_layout(bots);

}
*/

/*
///TODO
pub mod russian_doll{
    /*

        #[test]
        fn test_russian_doll() {
            //In this test, test larger and larger rectangles overlapping each other.


            let mut bots = Vec::new();
            let mut id_counter = 0..;

            for x in (-1000..2000).step_by(20) {
                for y in (-100..200).step_by(20) {
                    if x > y {
                        let id = id_counter.next().unwrap();

                        //let rect = AABBox(make_rect((-1000, -100), (x, y)));
                        let rect =AABBox::new((-1000,-100),(y,x));

                        bots.push(BBox::new(
                            Bot {
                                id,
                                col: Vec::new(),
                            },
                            rect,
                        ));
                    }
                }
            }

            test_bot_layout(bots);
        }

    */
}


///TODO
pub mod one_apart{
    /*
    #[test]
    fn test_1_apart() {

        let mut bots = Vec::new();
        let mut id_counter = 0..;
        for x in (-1000..2000).step_by(21) {
            for y in (-100..200).step_by(21) {
                let id = id_counter.next().unwrap();
                //let rect = create_rect_from_point((x, y));
                let rect =AABBox::new((x-10,x+10),(y-10,y+10));

                bots.push(BBox::new(
                    Bot {
                        id,
                        col: Vec::new(),
                    },
                    rect,
                ));
            }
        }

        test_bot_layout(bots);
    }
    */

}

///TODO
pub mod lattice{
    /*
    #[test]
    fn test_corners_touch() {

        //# # # #
        // # # #
        //# # # #
        let mut bots = Vec::new();
        let mut id_counter = 0..;
        let mut a = false;
        for y in (-100..200).step_by(20) {
            if a {
                for x in (-1000..2000).step_by(20).step_by(2) {
                    let id = id_counter.next().unwrap();
                    let rect =AABBox::new((x-10,x+10),(y-10,y+10));
                    bots.push(BBox::new(
                        Bot {
                            id,
                            col: Vec::new(),
                        },
                        rect,
                    ));
                }
            } else {
                for x in (-1000..2000).step_by(20).skip(1).step_by(2) {
                    let id = id_counter.next().unwrap();
                    //let rect = create_rect_from_point((x, y));
                    let rect =AABBox::new((x-10,x+10),(y-10,y+10));

                    bots.push(BBox::new(
                        Bot {
                            id,
                            col: Vec::new(),
                        },
                        rect,
                    ));
                }
            }
            a = !a;
        }

        test_bot_layout(bots);
        //assert!(false);
    }
    */
}



*/
