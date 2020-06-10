use crate::PropIdent;

macro_rules! prop_ident {
    ( $a:tt ) => {
        pub struct $a<SUCC>(std::marker::PhantomData<SUCC>);

        impl<SUCC> PropIdent for $a<SUCC> where SUCC: PropIdent {}
        impl PropIdent for $a<()> {}
    };
}

prop_ident!(__);
prop_ident!(A);
prop_ident!(B);
prop_ident!(C);
prop_ident!(D);
prop_ident!(E);
prop_ident!(F);
prop_ident!(G);
prop_ident!(H);
prop_ident!(I);
prop_ident!(J);
prop_ident!(K);
prop_ident!(L);
prop_ident!(M);
prop_ident!(N);
prop_ident!(O);
prop_ident!(P);
prop_ident!(Q);
prop_ident!(R);
prop_ident!(S);
prop_ident!(T);
prop_ident!(U);
prop_ident!(V);
prop_ident!(W);
prop_ident!(X);
prop_ident!(Y);
prop_ident!(Z);
