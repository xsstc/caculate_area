pub trait Calculate {
    fn area (&self) ->f64;
}

struct Triangle {
    d:f64,
    h:f64,
}

impl Calculate for Triangle {
    fn area (&self) ->f64 {
        0.5 * &self.d * &self.h
    }
}

pub fn print_area <T:Calculate> (figure:T){
    let figure_area = figure.area();
    println!("该图形的面积为:{}",figure_area);
} 


fn main() {
   let triangle = Triangle{d:10.0,h:10.0};
   print_area(triangle);
}
