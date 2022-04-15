//! 资源加载器

use futures::{future::BoxFuture, FutureExt};

use pi_enum_default_macro::EnumDefault;
use pi_share::Share;

// TODO
use pi_res::{Res, ResMgr};

/// 资源
/// 一个需要LoadMgr加载的资源，需要实现该trait
pub trait Asset: Res {
	type Desc;

	/// 取到资源的状态
	fn state(&self) -> State;
	/// 资源是否是异步加载
    fn is_async() -> bool;

	/// 异步加载资源
	// 加载资源前，需要设置资源状态为Loading， 加载资源后，需要设置资源状态为Ok
	// asset可能需要是Share<ShareCell<Self>>
    fn async_load(
		load_mgr: &mut LoadMgr, 
		asset: Share<Self>/* Share<ShareCell<Self>>?*/, 
		desc: Self::Desc) -> BoxFuture<'static, ()>;
	/// 同步加载资源
	// 加载资源前，需要设置资源状态为Loading， 加载资源后，需要设置资源状态为Ok
	// asset可能需要是Share<ShareCell<Self>>
    fn load(
		load_mgr: &mut LoadMgr,
		asset: Share<Self>/* Share<ShareCell<Self>>?*/, 
		desc: Self::Desc);
}

/// 资源加载管理器
pub struct LoadMgr {
	// 资源管理器。TODO, 暂时为pi_res中的资源管理器
	res_mgr: ResMgr,
}

impl LoadMgr {
	/// 创建资源加载器
	pub fn new(res_mgr: ResMgr) -> LoadMgr {
		LoadMgr { res_mgr }
	}
	/// 取到资源
	pub fn get<A: Asset>(&mut self, key: &A::Key, group: usize) -> Option<Share<A>> {
		self.res_mgr.get(key, group)
	}

	/// 创建资源
	/// 如果get方法取不到资源，调用该方法创建一个资源
	pub fn create<A: Asset>(&mut self, key: A::Key, group_i: usize, cost: usize, asset: A, desc: A::Desc) -> BoxFuture<'static, ()> {
		let res = self.res_mgr.create(key, group_i, asset, cost);
		self.load(res, desc)
	}

	// 加载资源
	pub fn load<A: Asset>(&mut self, asset: Share<A>, desc: A::Desc) -> BoxFuture<'static, ()> {
		match asset.state() {
			State::Waiting => if A::is_async() {
				A::async_load(self, asset, desc)
			} else {
				A::load(self, asset, desc);
				//todo
				async move {}.boxed()
			},
			State::Loading => {
				todo!()
			},
			State::Ok => {
				todo!()
			},
		}
		
	}

	/// 取到资源管理器的只读引用
	pub fn res_mgr(&self) -> &ResMgr {
		&self.res_mgr
	}
	/// 取到资源管理器的可写引用
	pub fn res_mgr_mut(&mut self) -> &mut ResMgr {
		&mut self.res_mgr
	}
}

#[derive(Debug, Clone, Copy, EnumDefault, PartialEq, Eq)]
pub enum State {
	Waiting,
	Loading,
	Ok,
}

