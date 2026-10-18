#![no_std]

#[macro_export]
macro_rules! emplace {
   (@tuple $x:expr, $p:path, [$a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr$(,)?]) => {{
      let x = &mut $x;
      *x = $p($a, $b, $c, $d, $e, $f, $g, $h);
      match x {
         $p(a, b, c, d, e, f, g, h) => (a, b, c, d, e, f, g, h),
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@tuple $x:expr, $p:path, [$a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr$(,)?]) => {{
      let x = &mut $x;
      *x = $p($a, $b, $c, $d, $e, $f, $g);
      match x {
         $p(a, b, c, d, e, f, g) => (a, b, c, d, e, f, g),
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@tuple $x:expr, $p:path, [$a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr$(,)?]) => {{
      let x = &mut $x;
      *x = $p($a, $b, $c, $d, $e, $f);
      match x {
         $p(a, b, c, d, e, f) => (a, b, c, d, e, f),
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@tuple $x:expr, $p:path, [$a:expr, $b:expr, $c:expr, $d:expr, $e:expr$(,)?]) => {{
      let x = &mut $x;
      *x = $p($a, $b, $c, $d, $e);
      match x {
         $p(a, b, c, d, e) => (a, b, c, d, e),
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@tuple $x:expr, $p:path, [$a:expr, $b:expr, $c:expr, $d:expr$(,)?]) => {{
      let x = &mut $x;
      *x = $p($a, $b, $c, $d);
      match x {
         $p(a, b, c, d) => (a, b, c, d),
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@tuple $x:expr, $p:path, [$a:expr, $b:expr, $c:expr$(,)?]) => {{
      let x = &mut $x;
      *x = $p($a, $b, $c);
      match x {
         $p(a, b, c) => (a, b, c),
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@tuple $x:expr, $p:path, [$a:expr, $b:expr$(,)?]) => {{
      let x = &mut $x;
      *x = $p($a, $b);
      match x {
         $p(a, b) => (a, b),
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@tuple $x:expr, $p:path, [$a:expr$(,)?]) => {{
      let x = &mut $x;
      *x = $p($a);
      match x {
         $p(a) => a,
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@struct $x:expr, $p:path, [$a:ident: $a1:expr, $b:ident: $b1:expr, $c:ident: $c1:expr, $d:ident: $d1:expr, $e:ident: $e1:expr, $f:ident: $f1:expr, $g:ident: $g1:expr, $h:ident: $h1:expr$(,)?]) => {{
      let x = &mut $x;
      *x = { $p { $a: $a1, $b: $b1, $c: $c1, $d: $d1, $e: $e1, $f: $f1, $g: $g1, $h: $h1 } };
      match x {
         $p { $a, $b, $c, $d, $e, $f, $g, $h } => ($a, $b, $c, $d, $e, $f, $g, $h),
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@struct $x:expr, $p:path, [$a:ident: $a1:expr, $b:ident: $b1:expr, $c:ident: $c1:expr, $d:ident: $d1:expr, $e:ident: $e1:expr, $f:ident: $f1:expr, $g:ident: $g1:expr$(,)?]) => {{
      let x = &mut $x;
      *x = { $p { $a: $a1, $b: $b1, $c: $c1, $d: $d1, $e: $e1, $f: $f1, $g: $g1 } };
      match x {
         $p { $a, $b, $c, $d, $e, $f, $g } => ($a, $b, $c, $d, $e, $f, $g),
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@struct $x:expr, $p:path, [$a:ident: $a1:expr, $b:ident: $b1:expr, $c:ident: $c1:expr, $d:ident: $d1:expr, $e:ident: $e1:expr, $f:ident: $f1:expr$(,)?]) => {{
      let x = &mut $x;
      *x = { $p { $a: $a1, $b: $b1, $c: $c1, $d: $d1, $e: $e1, $f: $f1 } };
      match x {
         $p { $a, $b, $c, $d, $e, $f } => ($a, $b, $c, $d, $e, $f),
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@struct $x:expr, $p:path, [$a:ident: $a1:expr, $b:ident: $b1:expr, $c:ident: $c1:expr, $d:ident: $d1:expr, $e:ident: $e1:expr$(,)?]) => {{
      let x = &mut $x;
      *x = { $p { $a: $a1, $b: $b1, $c: $c1, $d: $d1, $e: $e1 } };
      match x {
         $p { $a, $b, $c, $d, $e } => ($a, $b, $c, $d, $e),
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@struct $x:expr, $p:path, [$a:ident: $a1:expr, $b:ident: $b1:expr, $c:ident: $c1:expr, $d:ident: $d1:expr$(,)?]) => {{
      let x = &mut $x;
      *x = { $p { $a: $a1, $b: $b1, $c: $c1, $d: $d1 } };
      match x {
         $p { $a, $b, $c, $d } => ($a, $b, $c, $d),
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@struct $x:expr, $p:path, [$a:ident: $a1:expr, $b:ident: $b1:expr, $c:ident: $c1:expr$(,)?]) => {{
      let x = &mut $x;
      *x = { $p { $a: $a1, $b: $b1, $c: $c1 } };
      match x {
         $p { $a, $b, $c } => ($a, $b, $c),
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@struct $x:expr, $p:path, [$a:ident: $a1:expr, $b:ident: $b1:expr$(,)?]) => {{
      let x = &mut $x;
      *x = { $p { $a: $a1, $b: $b1 } };
      match x {
         $p { $a, $b } => ($a, $b),
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@struct $x:expr, $p:path, [$a:ident: $a1:expr$(,)?]) => {{
      let x = &mut $x;
      *x = { $p {$a: $a1 } };
      match x {
         $p { $a } => $a,
         #[allow(unreachable_patterns)]
         _ => unsafe { ::core::hint::unreachable_unchecked() },
      }
   }};
   (@tag $x:expr, [$($p:tt)*], $p1:ident:: $($xs:tt)*) => {
      $crate::emplace!(@tag $x, [$($p)*$p1::], $($xs)+)
   };
   (@tag $x:expr, [$($p:tt)*], $p1:ident($($xs:tt)*)) => {
      $crate::emplace!(@tuple $x, $($p)*$p1, [$($xs)*])
   };
   (@tag $x:expr, [$($p:tt)*], $p1:ident { $($xs:tt)* }) => {
      $crate::emplace!(@struct $x, $($p)*$p1, [$($xs)*])
   };
   (@init [$($x:tt)+], <- $($xs:tt)+) => {
      $crate::emplace!(@tag $($x)+, [], $($xs)+)
   };
   (@$($xs:tt)*) => {
      compile_error!("Failed to match rule")
   };
   ($x:tt $($xs:tt)*) => {
      $crate::emplace!(@init [$x], $($xs)*)
   };
}

#[macro_export]
macro_rules! renew {
   (@init [$($x:tt)+], <- $f:expr) => {{
      let x = &mut $($x)+;
      ::core::ptr::write(x, ($f)(::core::ptr::read(x)));
   }};
   (@init [$($x:tt)+], $y:tt $($xs:tt)+) => {
      $crate::renew!(@init [$($x)+ $y], $($xs)+)
   };
   (@$($xs:tt)*) => {
      compile_error!("Failed to match rule")
   };
   ($x:tt $($xs:tt)*) => {
      $crate::renew!(@init [$x], $($xs)*)
   };
}

#[cfg(test)]
mod test {
   enum Foo {
      A(u32),
      B { bar: u32, baz: u32 },
   }

   #[test]
   fn lol() {
      let mut x = Foo::A(0);
      let (y, &mut z) = emplace!(x <- Foo::B { bar: 1, baz: 2 });
      assert_eq!(*y, 1);
      assert_eq!(z, 2);
      let &mut n = emplace!(x <- Foo::A(3));
      assert_eq!(n, 3);
   }

   struct Blah;

   enum Bleh {
      A(Blah),
      B(Blah),
   }

   #[test]
   fn loll() {
      let mut x = (Bleh::A(Blah), ());
      unsafe {
         renew!(x.0 <- |x| match x {
            Bleh::A(x) => Bleh::B(x),
            Bleh::B(x) => Bleh::A(x),
         })
      };
   }
}
