use std::marker::PhantomData;

use log::warn;
use sp_runtime::traits::Block;

use crate::{
	abft::SignatureSet,
	crypto::Signature,
	data_io::{SelendraData, SelendraNetworkMessage},
	network::{data::Network, Data},
	Hasher, Recipient,
};

pub type LegacyNetworkData<B> =
	legacy_selendra_bft::NetworkData<Hasher, SelendraData<B>, Signature, SignatureSet<Signature>>;

pub type CurrentNetworkData<B> =
	selendra_bft::NetworkData<Hasher, SelendraData<B>, Signature, SignatureSet<Signature>>;

impl<B: Block> SelendraNetworkMessage<B>
	for legacy_selendra_bft::NetworkData<Hasher, SelendraData<B>, Signature, SignatureSet<Signature>>
{
	fn included_data(&self) -> Vec<SelendraData<B>> {
		self.included_data()
	}
}

impl<B: Block> SelendraNetworkMessage<B>
	for selendra_bft::NetworkData<Hasher, SelendraData<B>, Signature, SignatureSet<Signature>>
{
	fn included_data(&self) -> Vec<SelendraData<B>> {
		self.included_data()
	}
}

/// A wrapper needed only because of type system theoretical constraints. Sadness.
pub struct NetworkWrapper<D: Data, DN: Network<D>> {
	inner: DN,
	_phantom: PhantomData<D>,
}

impl<D: Data, DN: Network<D>> From<DN> for NetworkWrapper<D, DN> {
	fn from(inner: DN) -> Self {
		NetworkWrapper { inner, _phantom: PhantomData }
	}
}

impl<D: Data, DN: Network<D>> NetworkWrapper<D, DN> {
	fn send<R>(&self, data: D, recipient: R)
	where
		R: Into<Recipient>,
	{
		if let Err(e) = self.inner.send(data, recipient.into()) {
			warn!(target: "selendra-network", "Error '{:?}' while sending an SelendraBFT message to the network.", e);
		}
	}

	async fn next_event(&mut self) -> Option<D> {
		self.inner.next().await
	}
}

#[async_trait::async_trait]
impl<D: Data, DN: Network<D>> selendra_bft::Network<D> for NetworkWrapper<D, DN> {
	fn send(&self, data: D, recipient: selendra_bft::Recipient) {
		NetworkWrapper::send(self, data, recipient)
	}

	async fn next_event(&mut self) -> Option<D> {
		NetworkWrapper::next_event(self).await
	}
}

#[async_trait::async_trait]
impl<D: Data, DN: Network<D>> legacy_selendra_bft::Network<D> for NetworkWrapper<D, DN> {
	fn send(&self, data: D, recipient: legacy_selendra_bft::Recipient) {
		NetworkWrapper::send(self, data, recipient)
	}

	async fn next_event(&mut self) -> Option<D> {
		NetworkWrapper::next_event(self).await
	}
}