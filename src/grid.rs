
use axgeom::*;

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