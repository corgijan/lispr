 use std::fmt::Debug;
// 1.4.0
use std::thread::LocalKey;
use std::collections::HashMap;
use std::sync::Mutex;
use std::ops::{Add,Sub,Div,Mul, Index};


thread_local! {
    static ARR: Mutex<HashMap<String,Lispex>> = Mutex::new(HashMap::new());
}

macro_rules! lsp {
    ((+ $a:tt $b:tt)) => {{lsp![$a]+lsp![$b] }};
    ((- $a:tt $b:tt)) => { lsp![$a]-lsp![$b] };
    ((div $a:tt $b:tt)) => { lsp![$a] /lsp![$b] };
    ((mul $a:tt $b:tt)) => { lsp![$a] *lsp![$b] };
    ((print $($a:tt)+ )) =>{println!("{:?}", lsp![$($a)+])};
    ((first $($a:tt)+))=>{{
        let v=lsp![$($a)+];
        v[0]
        }};
    ((rest $($a:tt)+))=>{{
        let mut v=lsp![$($a)+];
        v.remove(0);
        v
        }};
    (#($($a:tt)+))=>{{
        lsp![(#$($a)+)]   
        }};

    ((# $($a:tt)+))=>{{
        let mut v=Vec::new();
        $(v.push(lsp![$a]);)+
        Lispex::LIST(v)
        }};
     ((nth $n:tt $($a:tt)+))=>{{
        let v = lsp![$($a)+];
        v[$n].clone()
        }};
      ((setq $varname:ident $($more:tt)+))=>{{
          ARR.with(|arr| arr.lock()
                   .unwrap()
                   .insert(stringify!($varname).to_string(),lsp![$($more)+]));
        }};
      ($varname: ident) => {
        ARR.with(|a|a.lock().unwrap().get(stringify!($varname)).unwrap().clone())
        };
    ($a:expr)=>{Lispex::NUMBER($a)};
}

macro_rules! lsp_program {
    ($($a:tt)+)=>{{
        $(lsp![$a];)+
    }};
}

macro_rules! lspenv {
    ($n:ident) => {
        let $n = 2;
    };
}

#[derive(Debug,Clone,PartialEq)]
enum  Lispex{
   ATOM(String),
   LIST(Vec<Lispex>),
   NUMBER(i32)
}



impl Index<usize> for Lispex {
    type Output = Self;

    fn index(&self, n:usize ) -> &Self {
        if let Lispex::LIST(vec)=&self{
            &vec[n]
        }else{
            panic!("Cant index a non indexable thing")
        }
    }
}

impl Add for Lispex{
    type Output = Self;
    fn add(self,o:Self)->Self{
        if let (Lispex::NUMBER(a),Lispex::NUMBER(b))= (self,o){
            Lispex::NUMBER(a+b)
        }else{
            panic!("cant add two not addable types")
        }
    }
}

impl Sub for Lispex{
    type Output = Self;
    fn sub(self,o:Self)->Self{
        if let (Lispex::NUMBER(a),Lispex::NUMBER(b))= (self,o){
            Lispex::NUMBER(a-b)
        }else{
            panic!("cant subtract two not addable types")
        }
    }
}

impl Div for Lispex{
    type Output = Self;
    fn div(self,o:Self)->Self{
        if let (Lispex::NUMBER(a),Lispex::NUMBER(b))= (self,o){
            Lispex::NUMBER(a/b)
        }else{
            panic!("cant div two not addable types")
        }
    }
}

impl Mul for Lispex{
    type Output = Self;
    fn mul(self,o:Self)->Self{
        if let (Lispex::NUMBER(a),Lispex::NUMBER(b))= (self,o){
            Lispex::NUMBER(a*b)
        }else{
            panic!("cant mul two not addable types")
        }
    }
}



impl From<i32> for Lispex {
    fn from(s:i32)->Lispex{
        return Lispex::NUMBER(s);
    }
}

impl From<String> for Lispex {
    fn from(s:String)->Lispex{
        return Lispex::ATOM(s);
    }
}
impl From<Vec<Lispex>> for Lispex {
    fn from(s:Vec<Lispex>)->Lispex{
        return Lispex::LIST(s);
    }
}


impl Lispex{
    fn remove(&mut self,n:usize){
    if let  Lispex::LIST(vec) = self{
        vec.remove(n);
    }else {
        panic!("cant remove from a non-List")
    }
    }
}


fn main() {
    lsp_program![
        (setq a #(1 2 3))
        (print a)
        (print #(1 a))
    ];
 assert_eq!(Lispex::NUMBER(3),(lsp![(nth 0 (rest #(2 3 5)))]));
    println!("{:?}",lsp![#(1 2 3 4)]);
}



#[test]
fn it_works() {
    assert_eq!(4, lsp![(+ 2 2)]);
    assert_eq!(1,lsp![(first #(1 2 3))]);
    assert_eq!(vec![2,3],lsp![(rest #(1 2 3))]);
    assert_eq!(vec![2,3],lsp![#(2 3)]);
    assert_eq!(3,lsp![(nth 0 (rest #(2 3 5)))])
}
