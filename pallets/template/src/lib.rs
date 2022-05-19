//! Substrate Proof-of-Existence Pallet
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, storage::bounded_vec::BoundedVec};
	use frame_system::pallet_prelude::*;

	// Define the pallet struct placeholder, various pallet function are implemented on it.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config] // <-- Step 2. code block will replace this.
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// For constraining the maximum bytes of a hash used for any proof
		type MaxBytesInHash: Get<u32>;
		/// Music Id
		type MusicId: Get<u32>;
		/// Artist name
		type Artist: Get<u32>;
		/// Title
		type TrackTitle: Get<u32>;
		/// Album name
		type Album: Get<u32>;
		/// Genre
		type Genre: Get<u32>;
		/// Bpm
		type Bpm: Get<u32>;
		/// Key
		type Key: Get<u32>;
		/// Time Signature
		type TimeSignature: Get<u32>;
		/// Number of Bars
		type Bars: Get<u32>;
		/// Duration
		type Duration: Get<u32>;
		/// Start Beat Offset in ms
		type StartBeatOffsetMs: Get<u32>;
	}

	// Pallets use events to inform users when important changes are made.
	// Event documentation should end with an array that provides descriptive names for parameters.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event] // <-- Step 3. code block will replace this.
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event emitted when a proof has been claimed. [who, claim]
		ClaimCreated(
			T::AccountId,
			BoundedVec<u8, T::MusicId>,
		),
		/// Event emitted when a claim is revoked by the owner. [who, claim]
		ClaimRevoked(T::AccountId, BoundedVec<u8, T::MaxBytesInHash>),
	}

	#[pallet::error] // <-- Step 4. code block will replace this.
	pub enum Error<T> {
		/// The proof has already been claimed.
		ProofAlreadyClaimed,
		/// The proof does not exist, so it cannot be revoked.
		NoSuchProof,
		/// The proof is claimed by another account, so caller can't revoke it.
		NotProofOwner,
	}

	#[pallet::storage] // <-- Step 5. code block will replace this.
	/// Maps each proof to its owner and block number when the proof was made
	pub(super) type FullTracks<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MusicId>,
		(
			T::AccountId,
			BoundedVec<u8, T::MaxBytesInHash>,
			BoundedVec<u8, T::Artist>,
			BoundedVec<u8, T::TrackTitle>,
			BoundedVec<u8, T::Album>,
			BoundedVec<u8, T::Genre>,
			BoundedVec<u8, T::Bpm>,
			BoundedVec<u8, T::Key>,
			BoundedVec<u8, T::TimeSignature>,
			BoundedVec<u8, T::Bars>,
			BoundedVec<u8, T::Duration>,
			BoundedVec<u8, T::StartBeatOffsetMs>,
			T::BlockNumber,
		),
		OptionQuery,
	>;
	// 	// (
	// 	// 	T::Album,
	// 	// 	T::Artwork,
	// 	// 	T::Duration,
	// 	// 	T::Genre,
	// 	// 	T::Key,
	// 	// 	T::TimeSignature,
	// ),
	// pub(super) type Proofs<T: Config> = StorageDoubleMap<
	// 	_,
	// 	Blake2_128Concat,
	// 	BoundedVec<u8, T::Artist>,
	// 	Blake2_128Concat,
	// 	(
	// 		BoundedVec<u8, T::MaxBytesInHash>,
	// 		BoundedVec<u8, T::Artist>,
	// 		BoundedVec<u8, T::Title>,
	// 		BoundedVec<u8, T::Bpm>,
	// 	),
	// 	(T::AccountId, T::BlockNumber),
	// 	OptionQuery,
	// >;
	// Dispatchable functions allow users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call] // <-- Step 6. code block will replace this.
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1_000)]
		pub fn create_fulltrack(
			origin: OriginFor<T>,
			music_id: BoundedVec<u8, T::MusicId>,
			music_file: BoundedVec<u8, T::MaxBytesInHash>,
			artist: BoundedVec<u8, T::Artist>,
			track_title: BoundedVec<u8, T::TrackTitle>,
			album: BoundedVec<u8, T::Album>,
			genre: BoundedVec<u8, T::Genre>,
			bpm: BoundedVec<u8, T::Bpm>,
			key: BoundedVec<u8, T::Key>,
			time_signature: BoundedVec<u8, T::TimeSignature>,
			bars: BoundedVec<u8, T::Bars>,
			duration: BoundedVec<u8, T::Duration>,
			start_beat_offset_ms: BoundedVec<u8, T::StartBeatOffsetMs>,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;

			// Verify that the specified proof has not already been claimed.
			// ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

			// Get the block number from the FRAME System pallet.
			let current_block = <frame_system::Pallet<T>>::block_number();

			// Store the proof with the sender and block number.
			FullTracks::<T>::insert(
				&music_id,
				(&sender, &music_file, &artist, &track_title, &album, &genre, &bpm, &key, &time_signature, &bars, &duration, &start_beat_offset_ms, current_block),
			);


			// Emit an event that the claim was created.
			Self::deposit_event(Event::ClaimCreated(sender, music_id));

			Ok(())
		}

		// #[pallet::weight(10_000)]
		// pub fn revoke_claim(
		// 	origin: OriginFor<T>,
		// 	proof: BoundedVec<u8, T::MaxBytesInHash>,
		// ) -> DispatchResult {
		// 	// Check that the extrinsic was signed and get the signer.
		// 	// This function will return an error if the extrinsic is not signed.
		// 	// https://docs.substrate.io/v3/runtime/origins
		// 	let sender = ensure_signed(origin)?;

		// 	// Verify that the specified proof has been claimed.
		// 	// ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

		// 	// Get owner of the claim.
		// 	// Panic condition: there is no way to set a `None` owner, so this must always unwrap.
		// 	let (owner, _) = Proofs::<T>::get(&proof).expect("All proofs must have an owner!");

		// 	// Verify that sender of the current call is the claim owner.
		// 	ensure!(sender == owner, Error::<T>::NotProofOwner);

		// 	// Remove claim from storage.
		// 	Proofs::<T>::remove(&proof);

		// 	// Emit an event that the claim was erased.
		// 	Self::deposit_event(Event::ClaimRevoked(sender, proof));
		// 	Ok(())
		// }
	}
}
