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
