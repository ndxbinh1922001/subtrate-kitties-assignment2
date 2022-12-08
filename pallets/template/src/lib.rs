#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::DispatchResult;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::dispatch::Vec;
	use scale_info::TypeInfo;
	use frame_support::BoundedVec;
	pub type Index = u64;

	pub type Limit = u8;

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
	pub enum Class {
		A, 
		B, 
		C,
	}

	#[derive(Encode, Decode, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Student<T:Config> {
		id: Index,
		class: Class,
		name:Name,
		account:T::AccountId,
	
	}

	#[derive(Encode,Decode, TypeInfo)]
	pub struct Name(Vec<u8>);
	impl MaxEncodedLen for Name {

		fn max_encoded_len() -> usize {
		
			10
		}
		
	
	}
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	//#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn student)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Students<T:Config> = StorageValue<_, Student<T>>;


	#[pallet::storage]
	#[pallet::getter(fn account)]
	// Learn more about declaring storage items:
	//frame_system::Config::AccountId
	// thông qua 1 generic type  -> constraint bởi trait Config
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Account<T: Config> = StorageValue<_, T::AccountId>;



	#[pallet::storage]
	#[pallet::getter(fn name_student)]
	#[pallet::unbounded]
	pub type StudentName<T> = StorageValue<_, Vec<u8>, ValueQuery>;

	// #[pallet::storage]
	// #[pallet::getter(fn extrinsic_data)]
	// #[pallet::unbounded]
	// pub(super) type ExtrinsicData<T: Config> =
	// 	StorageMap<_, Twox64Concat, u32, Vec<u8>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}


// helper function
impl<T:Config> Pallet<T> {

	pub fn update_storage(new_value: u32) -> DispatchResult {
	
		
		Something::<T>::put(new_value);
		
		Ok(())
	}

}


pub trait DoSome {

	fn increase_value(value: u32) -> u32;
}


impl<T> DoSome for Pallet<T> {
	fn increase_value(value: u32) -> u32 {
		value + 5
	
	}
}