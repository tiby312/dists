
use axgeom::*;


/*
pub struct Grid2{
    test:Box<dyn Generator<Yield=Vec2<f32>,Return=()>+Unpin>
}

impl Iterator for Grid2{
    type Item=Vec2<f32>;
    fn next(&mut self)->Option<Self::Item>{
        match self.test.resume(){
            GeneratorState::Yielded(a) => {
                Some(a)
            },
            _ => panic!("unexpected value from resume"),
        }
    }
}
*/

pub fn from_center(start:Vec2<f32>,aspect_ratio:f32,spacing:f32,num:usize,mut func:impl FnMut(Vec2<f32>)){

    func(start);
    let mut rect=Rect::new(0isize,1,0,1);
    let mut counter=1;

    let mut grow_down=vec2same(true);

    loop{
        let dim=vec2(rect.x.right-rect.x.left,rect.y.right-rect.y.left);
        let curr_aspect_ratio = dim.x as f32/dim.y as f32;
        
        //if to wide
        if curr_aspect_ratio>aspect_ratio{

            let p=if grow_down.y{
                rect.y.right+=1;
                rect.y.right
            }else{
                rect.y.left-=1;
                rect.y.left
            };

            //add a row at the bottom
            for x in rect.x.left..rect.x.right{
                if counter==num{
                    return;
                }
                let v=vec2(x,p);
                func(start+(v.inner_as::<f32>()*spacing));
                counter+=1;
            }
            grow_down.y=!grow_down.y;
        }else{

            let p=if grow_down.x{
                rect.x.right+=1;
                rect.x.right
            }else{
                rect.x.left-=1;
                rect.x.left
            };

            //add a colum on the right
            for y in rect.y.left..rect.y.right{
                if counter==num{
                    return;
                }
                let v=vec2(p,y);
                func(start+(v.inner_as::<f32>()*spacing));
                counter+=1;
            }
            grow_down.x=!grow_down.x;
        }
    }
}

pub fn from_top_left(start:Vec2<f32>,aspect_ratio:f32,spacing:f32,num:usize,mut func:impl FnMut(Vec2<f32>)){    
    
    func(start);
    let mut dim=vec2same(1);
    
    let mut counter=1;
    loop{
        let curr_aspect_ratio=dim.x as f32/dim.y as f32;
        //dbg!(curr_aspect_ratio);
        //if to wide
        if curr_aspect_ratio>aspect_ratio{
            //add a row at the bottom
            for x in 0..dim.x{
                if counter==num{
                    return;
                }
                let v=vec2(x,dim.y+1);
                func(start+(v.inner_as::<f32>()*spacing));
                counter+=1;
            }
            dim.y+=1;
        }else{
            //add a colum on the right
            for y in 0..dim.y{
                if counter==num{
                    return;
                }
                let v=vec2(dim.x+1,y);
                func(start+(v.inner_as::<f32>()*spacing));
                counter+=1;
            }
            dim.x+=1;
        }
    }

}





#[derive(Clone)]
pub struct Grid{
    rect:Rect<f32>,
    grid_dim:Vec2<usize>,
    cursor:Vec2<usize>,
    spacing:Vec2<f32>
}



impl Grid{
    pub fn new(rect:Rect<f32>,num_bots:usize)->Grid{
        let width=rect.x.right-rect.x.left;
        let height=rect.y.right-rect.y.left;

        let aspect_ratio=width/height;


        //w*h=num_bots
        //w/h=width/height
        //solve for sx and sy


        //w=num_bots/h
        //num_bots/h^2=width/height
        //h^2=num_bots/(width/height);
        //h=sqrt(num_bots/(width/height));

        let h = (num_bots as f32/aspect_ratio).sqrt().ceil() as usize;
        let w = num_bots/h;

        let grid_dim=vec2(w,h);
        let cursor=vec2(0,0);
        let spacing=vec2(width/w as f32,height/h as f32);
        
        Grid{rect,grid_dim,cursor,spacing}
    }
}

impl std::iter::FusedIterator for Grid{}

impl Iterator for Grid{
    type Item=Vec2<f32>;
    fn next(&mut self)->Option<Vec2<f32>>{
        let topleft=vec2(self.rect.x.left,self.rect.y.left);

        let kk=vec2(self.cursor.x as f32*self.spacing.x,self.cursor.y as f32*self.spacing.y);
        let ans=topleft+kk;

        //increment
        if self.cursor.x<self.grid_dim.x{
            self.cursor.x+=1;
        }else{
            if self.cursor.y<self.grid_dim.y{
                self.cursor.x=0;
                self.cursor.y+=1;
            }else{
                return None
            }
        }

        debug_assert!(!ans.x.is_nan());
        debug_assert!(!ans.y.is_nan());
        Some(ans)
    }
}

use crate::Dist;

impl Dist<f32> for Grid{}