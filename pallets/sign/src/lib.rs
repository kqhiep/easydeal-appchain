#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
use frame_support::RuntimeDebug;

use sp_std::prelude::*;

use codec::{Decode, Encode};
pub use pallet::*;

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
pub struct Sign<AccountId, BlockNumber> {
	total_count: u32,
	daily_count: u32,
	total_reword: u32,
	last_signed_time: u32,
	creator: AccountId,
	block: BlockNumber,
}

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(crate) type DailyReword<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub(crate) type DailyBase<T> = StorageValue<_, u32, ValueQuery>;

	//TODO dont need
	#[pallet::storage]
	pub(crate) type TotalCount<T> = StorageValue<_, u32, ValueQuery>;

	//TODO dont need
	#[pallet::storage]
	pub(crate) type DailyCount<T> = StorageValue<_, u32, ValueQuery>;

	//TODO dont need
	#[pallet::storage]
	pub(crate) type TotalReword<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	pub(crate) type SignInfo<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, Sign<T::AccountId, T::BlockNumber>>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_sign(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let block_number = frame_system::Pallet::<T>::block_number();

			// TODO fetch the storage signInfo 
			let t_count = TotalCount::<T>::get();
			let total_count = t_count + 1;
			let d_count = DailyCount::<T>::get();
			// signing timestamp is less than a day , sum daily_count, otherwise sub
			let daily_count = d_count + 1;
			let total_reword = TotalReword::<T>::get();

			let d_reword = DailyReword::<T>::get();
			let g_reword_base = DailyBase::<T>::get();
			//TODO
			let my_daily_base = g_reword_base + daily_count;
			let t_reword = total_reword + d_reword * my_daily_base;
			let sign: Sign<T::AccountId, T::BlockNumber> = Sign {
				total_count: total_count,
				daily_count: daily_count,
				total_reword: t_reword,
				// isSigned: true,
				last_signed_time: total_count, //TODO time
				creator: who.clone(),
				block: block_number,
			};

			TotalCount::<T>::put(total_count);
			DailyCount::<T>::put(daily_count);
			TotalReword::<T>::put(total_reword);

			SignInfo::<T>::insert(who.clone(), sign);

			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_daily_reword(origin: OriginFor<T>, amount: u32) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			//TODO amount have to bigger than zero
			// let newAmount = amount > 0
			TotalCount::<T>::put(amount);
			Ok(().into())

		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_daily_base(origin: OriginFor<T>, amount: u32) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			//TODO amount have to bigger than zero
			// let newAmount = amount > 0
			DailyBase::<T>::put(amount);
			Ok(().into())

		}
	}
}
