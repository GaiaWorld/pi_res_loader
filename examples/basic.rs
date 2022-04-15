use pi_res::{Res, ResMgr};
use pi_res_loader::{Asset, State, LoadMgr};
use pi_share::Share;

#[derive(Debug, Default)]
pub struct MyRes(pub usize, State);

impl Res for MyRes {
	type Key = u64;
}

impl Asset for MyRes {
    type Desc = usize;

    fn state(&self) -> State {
        self.1
    }

    fn is_async() -> bool {
        false
    }

    fn async_load(
		load_mgr: &mut pi_res_loader::LoadMgr, 
		asset: pi_share::Share<Self>/* Share<ShareCell<Self>>?*/, 
		desc: Self::Desc) -> futures::future::BoxFuture<'static, ()> {
			todo!()
    }

    fn load(
		load_mgr: &mut pi_res_loader::LoadMgr,
		asset: pi_share::Share<Self>, 
		desc: Self::Desc) {
		let res = unsafe { &mut *(Share::as_ptr(&asset) as usize as *mut Self)};
		//res.1 = State::Loading;
		res.0 = desc;
		res.1 = State::Ok;
    }
}

fn main() {
	let mut res_mgr = ResMgr::default();
	res_mgr.register::<MyRes>(1 * 1024 * 1024, 10 * 1024 * 1024, 60, 0, "my_res".to_string());

	let mut load_mgr = LoadMgr::new(res_mgr);

	let key = 1;
	let group = 0;
	
	let mut loop_count = 2;
	while loop_count > 0 {
		match load_mgr.get::<MyRes>(&key, group) {
			Some(r) => println!("res==={:?}", r),
			// 不存在资源时，创建资源
			None => {
				// MyRes是同步创建，下次循环取到的MyRes应该已经加载成功
				load_mgr.create(key, group, 10, MyRes::default(), 1);
			},
		};

		loop_count -= 1;
	}
	
}