pub mod ident;

/// Marker trait representing identification of property.
///
/// ## Examples
///
/// - `I<D<()>>` represents `id` property.
/// - `R<E<Q<__<B<O<D<Y<()>>>>>>>>` represents `req_body` property.
pub trait PropIdent {}

/// This trait represents that implemented struct has a property whose identification is
/// represented by `Ident` and type is `Type`.
///
/// Suppose we have a struct `Foo` which has a property `foo: usize`, then `Foo` can implement
/// `Property<F<O<O<()>>>, usize>`.
pub trait Property<Ident, Type>: PropertyBase
where
    Ident: PropIdent,
    <Self as PropertyBase>::Mid: TakeProperty<Ident, Type>,
{
    fn get(&self) -> &Type;
}

pub trait PropertyBase {
    type Mid;

    fn into_mid(self) -> Self::Mid;
}

pub trait TakeProperty<Ident, Type>
where
    Ident: PropIdent,
{
    fn take_only_once(&mut self) -> Type;
}

/// This trait is much like `From` of standard library.
/// The difference is that this trait represents property based conversion.
///
/// This will automatically be implemented like below but generated code is slightly diferrent.
///
/// ```ignore
/// struct Foo {
///     foo: usize,
///     bar: String,
/// }
///
/// type FOO = F<O<O<()>>>;
/// type BAR = B<A<R<()>>>;
///
/// impl<T> PropFrom<T> for Foo
/// where
///     T: Property<FOO, usize> + Property<BAR, usize>
/// {
///     fn prop_from(t: T) -> Self {
///         todo!()
///     }
/// }
/// ```
pub trait PropFrom<T> {
    fn prop_from(t: T) -> Self;
}

/// This trait is much like `Into` of standard library.
/// The difference is that this trait represents property based conversion.
///
/// This is automatically implemented in most case.
pub trait PropInto<T> {
    fn prop_into(self) -> T;
}

impl<T, U> PropInto<U> for T
where
    U: PropFrom<T>,
{
    fn prop_into(self) -> U {
        U::prop_from(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ident::*;

    struct Hoge {
        id: usize,
        body: String,
    }

    struct Fuga {
        body: String,
    }

    #[test]
    fn valid_conversion() {
        let hoge = Hoge {
            id: 42,
            body: "hoge".to_string(),
        };
        let fuga = Fuga::prop_from(hoge);
        assert_eq!(fuga.body, "hoge");

        let hoge2 = Hoge {
            id: 21,
            body: "hoge2".to_string(),
        };
        let fuga2: Fuga = hoge2.prop_into();
        assert_eq!(fuga2.body, "hoge2");
    }

    /*
     * ===========================
     * Start auto-generated codes
     * ===========================
     */
    struct __HogeMid {
        id: Option<usize>,
        body: Option<String>,
    }

    type ID = I<D<()>>;
    type BODY = B<O<D<Y<()>>>>;

    impl PropertyBase for Hoge {
        type Mid = __HogeMid;

        fn into_mid(self) -> Self::Mid {
            __HogeMid {
                id: Some(self.id),
                body: Some(self.body),
            }
        }
    }

    impl TakeProperty<ID, usize> for __HogeMid {
        fn take_only_once(&mut self) -> usize {
            self.id.take().unwrap()
        }
    }

    impl TakeProperty<BODY, String> for __HogeMid {
        fn take_only_once(&mut self) -> String {
            self.body.take().unwrap()
        }
    }

    impl Property<ID, usize> for Hoge {
        fn get(&self) -> &usize {
            &self.id
        }
    }

    impl Property<BODY, String> for Hoge {
        fn get(&self) -> &String {
            &self.body
        }
    }

    impl<T> PropFrom<T> for Hoge
    where
        T: Property<ID, usize> + Property<BODY, String> + PropertyBase,
        <T as PropertyBase>::Mid: TakeProperty<ID, usize> + TakeProperty<BODY, String>,
    {
        fn prop_from(t: T) -> Self {
            let mut mid = t.into_mid();
            let id = TakeProperty::<ID, usize>::take_only_once(&mut mid);
            let body = TakeProperty::<BODY, String>::take_only_once(&mut mid);

            Hoge { id, body }
        }
    }

    struct __FugaMid {
        body: Option<String>,
    }

    impl PropertyBase for Fuga {
        type Mid = __FugaMid;

        fn into_mid(self) -> Self::Mid {
            __FugaMid {
                body: Some(self.body),
            }
        }
    }

    impl TakeProperty<BODY, String> for __FugaMid {
        fn take_only_once(&mut self) -> String {
            self.body.take().unwrap()
        }
    }

    impl Property<BODY, String> for Fuga {
        fn get(&self) -> &String {
            &self.body
        }
    }

    impl<T> PropFrom<T> for Fuga
    where
        T: Property<BODY, String> + PropertyBase,
        <T as PropertyBase>::Mid: TakeProperty<BODY, String>,
    {
        fn prop_from(t: T) -> Self {
            let mut mid = t.into_mid();
            let body = TakeProperty::<BODY, String>::take_only_once(&mut mid);
            Fuga { body }
        }
    }
    /*
     * ===========================
     * End auto-generated codes
     * ===========================
     */
}
