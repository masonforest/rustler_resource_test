#[macro_use]
extern crate rustler;
#[macro_use]
extern crate lazy_static;

extern crate rocksdb;
use rocksdb::{DB};
use std::sync::{RwLock,Arc};
use std::ops::Deref;

use rustler::{NifEnv, NifTerm, NifResult, NifEncoder};
use rustler::resource::ResourceArc;

mod atoms {
    rustler_atoms! {
        atom ok;
        //atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}


pub fn on_load<'a>(env: NifEnv<'a>) -> bool {
    resource_struct_init!(DBHandle, env);
    true
}


struct DBHandle {
    pub db: Arc<RwLock<DB>>,
}

impl Deref for DBHandle {
    type Target = Arc<RwLock<DB>>;

    fn deref(&self) -> &Self::Target { &self.db }
}

rustler_export_nifs! {
    "Elixir.NativeModule",
    [("get_resource", 0, get_resource)],
    None
}

fn get_resource<'a>(env: NifEnv<'a>, args: &[NifTerm<'a>]) -> NifResult<NifTerm<'a>> {

    let db = DB::open_default("tmp/blockchain.db").unwrap();
    let resp =
        (atoms::ok(), ResourceArc::new(DBHandle{
            db: Arc::new(RwLock::new(db)),
        })).encode(env);

    Ok(resp)
}
