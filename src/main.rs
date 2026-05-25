use std::ffi::FromBytesUntilNulError;
use std::fmt::{format, Display, Formatter};
use std::ops::{Add, Sub, Div, Mul, Index};
use std::ptr::write;

#[derive(Debug,Clone,PartialEq)]
enum Lispex{
    ATOM(String),
    LIST(Vec<Lispex>),
    NUMBER(i32),
    STRING(String),
    OBJECT(Vec<(String, Lispex)>),  // ordered field list
}
macro_rules! lsp {
    ((+ $a:tt $b:tt)) => {{lsp![$a]+lsp![$b] }};
    ((- $a:tt $b:tt)) => { lsp![$a]-lsp![$b] };
    ((div $a:tt $b:tt)) => { lsp![$a] /lsp![$b] };
    ((eq $a:tt $b:tt)) => { lsp![$a].internal()==lsp![$b].internal() };
    ((mul $a:tt $b:tt)) => { lsp![$a] *lsp![$b] };
    ((print $($a:tt)+ )) =>{println!("{}", lsp![$($a)+])};
    ((first $($a:tt)+))=>{{
        let v=lsp![$($a)+];
        v[0].clone().clone().clone()
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
    ((if $cond:tt ($($then:tt)+) ($($else:tt)*) ))=>{
        if lsp![$cond] {
            lsp![($($then)+)]
        }else{
             lsp![($($else)*)]
        }
    };
    ((setq $varname:ident $($more:tt)+))=>{
          let $varname = lsp![$($more)+];  
        };

    ((split $s:tt $sep:tt)) => {{
    let s = lsp![$s];
    let sep = lsp![$sep];
    if let Lispex::STRING(sep_str) = sep {
        if let Lispex::STRING(s_str) = s {
            let parts: Vec<Lispex> = s_str.split(&sep_str)
                .map(|part| Lispex::STRING(part.to_string()))
                .collect();
            Lispex::LIST(dbg!(parts))
        } else {
            panic!("split expects the first argument to be a string")
        }
    } else {
        panic!("split separator must be a string")
    }
    }};

    // Create an object: (obj (name "Alex") (age 33))
    ((obj $(($field:ident $($val:tt)+))* )) => {{
        let mut fields: Vec<(String, Lispex)> = Vec::new();
        $(
            fields.push((stringify!($field).to_string(), lsp![$($val)+]));
        )*
        Lispex::OBJECT(fields)
    }};

    // Field access sugar: (. object fieldname) -> (get object fieldname)
    ((. $obj:tt $field:ident)) => {
        lsp![(get $obj $field)]
    };

     // Field access: (. object fieldname)
    ((get $obj:tt $field:ident)) => {{
        let o = lsp![$obj];
        o.get_field(stringify!($field))
    }};

    // Field set: (set! object fieldname value)
    ((set! $obj:ident $field:ident $($val:tt)+)) => {{
        $obj.set_field(stringify!($field), lsp![$($val)+]);
    }};

    ((readfile $filename:tt)) => {{
        use std::fs;
        let f = lsp![$filename];
        if let Lispex::STRING(s) = f {
            let contents = fs::read_to_string(&s)
                .expect(&format!("Error reading file: {}", s));
            Lispex::STRING(contents)
        } else {
            panic!("readfile expects a string argument")
        }
    }};

    ((writefile $filename:tt $content:tt)) => {{
        use std::fs;
        let f = lsp![$filename];
        let c = lsp![$content];
        if let (Lispex::STRING(fname), Lispex::STRING(text)) = (f, c) {
            fs::write(&fname, &text)
                .expect(&format!("Error writing file: {}", fname));
            Lispex::STRING(format!("Wrote {} bytes to {}", text.len(), fname))
        } else {
            panic!("writefile expects (writefile <string-path> <string-content>)")
        }
    }};

    ((appendfile $filename:tt $content:tt)) => {{
        use std::fs::OpenOptions;
        use std::io::Write;
        let f = lsp![$filename];
        let c = lsp![$content];
        if let (Lispex::STRING(fname), Lispex::STRING(text)) = (f, c) {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(&fname)
                .expect(&format!("Error opening file for append: {}", fname));
            file.write_all(text.as_bytes())
                .expect(&format!("Error appending to file: {}", fname));
            Lispex::STRING(format!("Appended {} bytes to {}", text.len(), fname))
        } else {
            panic!("appendfile expects (appendfile <string-path> <string-content>)")
        }
    }};

    ((defun $fname:ident ($($paramname:ident)*)  $($body:tt)+))=>{
         //let $fname = |&$($paramname,)*|{lsp![$($body)+]};
         fn $fname($($paramname:&Lispex,)*)->Lispex{
            lsp![$($body)+]
         }
      };

    (($fname:ident $($param:tt)*))=>{
        $fname($(&lsp![$param],)*)

        };

    ($name:lifetime) => {{
        let s = stringify!($name);
        Lispex::ATOM(s[1..].to_string())
    }};


    ($varname: ident) => {
       $varname
        };

    ($a:expr)=>{Lispex::from($a)};

}

macro_rules! lsp_program {
    ($($a:tt)+)=>{
        $(lsp![$a];)+
    };
}

macro_rules! impl_ops {
    ($op:tt,$fnname:ident,$implType:ty,$forType:ty,$patern:pat,$out:expr)=>{

impl $op<$implType> for $forType{
    type Output = Lispex;
    fn $fnname(self,o:$implType)->Lispex{
        if let $patern = (self,o){
            Lispex::NUMBER($out)
        }else{
            panic!(stringify!($implType encountered an error with $op))
        }
    }
}

    };
}







// 2. Update Display
impl Display for Lispex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Lispex::ATOM(a) => format!("'{a}"),
            Lispex::LIST(a) => {
                let x = a.iter().map(|x| format!("{}", x)).collect::<Vec<_>>().join(" ");
                return write!(f, "#({})", x);
            }
            Lispex::NUMBER(a) => format!("{a}"),
            Lispex::STRING(a) => format!("\"{a}\""),
            Lispex::OBJECT(fields) => {
                let x = fields.iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ");
                return write!(f, "{{{}}}", x);
            }
        })
    }
}

impl Lispex {
    fn internal(&self) -> i32 {
        if let Lispex::NUMBER(a)= self{ 
            *a
        }else{
            0
        }
    }
    fn get_field(&self, name: &str) -> Lispex {
        if let Lispex::OBJECT(fields) = self {
            for (k, v) in fields {
                if k == name {
                    return v.clone();
                }
            }
            panic!("Field '{}' not found in object", name);
        } else {
            panic!("Cannot access field on non-object")
        }
    }

    fn set_field(&mut self, name: &str, value: Lispex) {
        if let Lispex::OBJECT(fields) = self {
            for (k, v) in fields.iter_mut() {
                if k == name {
                    *v = value;
                    return;
                }
            }
            fields.push((name.to_string(), value));
        } else {
            panic!("Cannot set field on non-object")
        }
    }
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

impl From<&str> for Lispex {
    fn from(s: &str) -> Lispex { Lispex::STRING(s.to_string()) }
}
impl From<String> for Lispex {
    fn from(s: String) -> Lispex { Lispex::STRING(s.to_string()) }
}
impl From<i32> for Lispex {
    fn from(n: i32) -> Lispex { Lispex::NUMBER(n) }
}

impl From<Vec<Lispex>> for Lispex {
    fn from(v: Vec<Lispex>) -> Lispex { Lispex::LIST(v) }
}


impl_ops!(Add,add,Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a+b);
impl_ops!(Add,add,Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a+b);
impl_ops!(Add,add,&Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a+b);
impl_ops!(Add,add,&Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a+b);


impl_ops!(Sub,sub,Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a-b);
impl_ops!(Sub,sub,&Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a-b);
impl_ops!(Sub,sub,Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a-b);
impl_ops!(Sub,sub,&Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a-b);


impl_ops!(Mul,mul,Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a*b);
impl_ops!(Mul,mul,&Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a*b);
impl_ops!(Mul,mul,Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a*b);
impl_ops!(Mul,mul,&Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a*b);


impl_ops!(Div,div,Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a/b);
impl_ops!(Div,div,&Lispex,Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a/b);
impl_ops!(Div,div,Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a/b);
impl_ops!(Div,div,&Lispex,&Lispex,(Lispex::NUMBER(a),Lispex::NUMBER(b)),a/b);




/*

impl Sub<Lispex> for i32{
    type Output = Lispex;
    fn sub(self,o:Lispex)->Lispex{
        if let (a,Lispex::NUMBER(b))= (self,o){
            Lispex::NUMBER(a-b)
        }else{
            panic!("cant subtract two not addable types")
        }
    }
}

impl Sub<&Lispex> for i32{
    type Output = Lispex;
    fn sub(self,o:&Lispex)->Lispex{
        if let (a,&Lispex::NUMBER(b))= (self,o){
            Lispex::NUMBER(a-b)
        }else{
            panic!("cant subtract two not addable types")
        }
    }
}


impl Mul<Lispex> for i32{
    type Output = Lispex;
    fn mul(self,o:Lispex)->Lispex{
        if let (a,Lispex::NUMBER(b))= (self,o){
            Lispex::NUMBER(a*b)
        }else{
            panic!("cant subtract two not addable types")
        }
    }
}

impl Add<Lispex> for i32{
    type Output = Lispex;
    fn add(self,o:Lispex)->Lispex{
        if let (a,Lispex::NUMBER(b))= (self,o){
            Lispex::NUMBER(a+b)
        }else{
            panic!("cant subtract two not addable types")
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
*/

impl Lispex{
    fn remove(&mut self,n:usize){
    if let  Lispex::LIST(vec) = self{
        vec.remove(n);
    }else {
        panic!("cant remove from a non-List")
    }
    }
}




#[test]
fn it_works() {
    assert_eq!(Lispex::NUMBER(4), lsp![(+ 2 2)]);
  }

fn main() {
    lsp_program![
        (defun square (x) (mul x x))
        (print (square 3))

        (defun fac (x) (if (eq x 1) (1) (mul x (fac (- x 1))) ))
        (print (fac 3))
        (print (if (eq 1 0) (1) (+ 1 1111)))
        (setq a #(1 2 3))
        (setq name #('test "aa" 'apples 'alex_payne "123.3" 3))
        (print (nth 0 name))

        (setq user1 #('alex_payne 33))
        (setq user2 #('jan_vaorin 22))
        (setq user3 #('john_doe 44))

        (setq users #(user1 user2))
        (print (nth 1 users))

        (setq users #(
            (obj (name "Alex") (age 33))
            (obj (name "Jan") (age 22))
        ))
        (print (. (nth 0 users) age))
        //(writefile "test.txt" "This is a test file created by the Lispex interpreter.")
        //(appendfile "test.txt" "\nAppending a new line to the test file.")
        //(print (readfile "test.txt"))
        (print (split  "a,b,c,a,d" "c"))
    ];
}
