use prop::ident::*;
use prop::*;

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
