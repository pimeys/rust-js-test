#[macro_use]
extern crate neon;

use neon::prelude::*;
use prisma_query::{ast::*, connector::*};
use std::{convert::TryFrom, sync::{Arc, Mutex}};

pub struct Connection {
    inner: Arc<Mutex<dyn Queryable>>,
}

declare_types! {
    pub class JsConnection for Connection {
        init(mut cx) {
            let uri: String = cx.argument::<JsString>(0)?.value();
            let mut sqlite = Sqlite::try_from(uri.as_str()).unwrap();
            sqlite.attach_database("test").unwrap();

            Ok(Connection {
                inner: Arc::new(Mutex::new(sqlite))
            })
        }

        method all_test(mut cx) {
            let this = cx.this();

            let result_set = {
                let guard = cx.lock();
                let conn = this.borrow(&guard);
                let mut inner = conn.inner.lock().unwrap();
                inner.query(Select::from_table("test").into()).unwrap()
            };

            let js_object = JsObject::new(&mut cx);

            for row in result_set {
                let id1 = row[0].as_i64().unwrap();
                let id2 = row[1].as_i64().unwrap();
                let id3 = row[2].as_i64().unwrap();

                let num1 = cx.number(id1 as f64);
                let num2 = cx.number(id2 as f64);
                let num3 = cx.number(id3 as f64);

                js_object.set(&mut cx, "id1", num1)?;
                js_object.set(&mut cx, "id2", num2)?;
                js_object.set(&mut cx, "id3", num3)?;
            }

            Ok(js_object.upcast())
        }
    }
}

register_module!(mut m, { m.export_class::<JsConnection>("Connection") });
