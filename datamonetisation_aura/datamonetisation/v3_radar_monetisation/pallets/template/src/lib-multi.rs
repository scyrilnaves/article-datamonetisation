#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::dispatch::DispatchResult;
	//Pallet access to substrate the hash types from frame_support library
	use frame_support::debug;
	use frame_support::sp_io::hashing;
	use frame_support::sp_runtime::print;
	use frame_support::sp_runtime::traits::Zero;
	use frame_support::sp_runtime::SaturatedConversion;
	use frame_support::storage::bounded_btree_set::BoundedBTreeSet;
	use frame_support::storage::IterableStorageMap;
	use frame_support::weights::Pays;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_support::{
		sp_runtime::traits::Hash,
		traits::{tokens::ExistenceRequirement, Currency, Randomness},
		transactional,
	};
	use frame_system::pallet_prelude::*;
	use frame_system::{
		self as system,
		offchain::{
			AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction,
			SignedPayload, Signer, SigningTypes, SubmitTransaction,
		},
	};
	use scale_info::TypeInfo;
	use serde_json::{Result, Value};
	use sp_std::collections::btree_set::BTreeSet;
	use sp_std::convert::TryInto;
	use sp_std::vec;
	use sp_std::vec::Vec;
	//Use H256 Hash output type to generate a randomness of its type
	use frame_support::sp_io::offchain_index;
	use frame_support::sp_runtime::traits::{
		AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Saturating,
	};
	use frame_support::sp_runtime::{
		offchain as rt_offchain,
		offchain::{
			storage::StorageValueRef,
			storage_lock::{BlockAndTime, StorageLock},
		},
		transaction_validity::{
			InvalidTransaction, TransactionSource, TransactionValidity, ValidTransaction,
		},
		RuntimeDebug,
	};
	//use frame_support::sp_runtime::traits::Hash;

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	const HTTP_REMOTE_REQUEST_ASSET: &str = "http://localhost:8080/assettoken";
	const HTTP_REMOTE_REQUEST_ASSET_VALID: &str = "http://localhost:8080/assettoken";
	const HTTP_REMOTE_REQUEST_ASSET_SERVICE: &str = "http://localhost:9090/assetservicetoken";
	const HTTP_REMOTE_REQUEST_ASSET_SERVICE_VALID: &str = "http://localhost:9090/assetservicetoken";
	const FETCH_TIMEOUT_PERIOD: u64 = 3000; // in milli-seconds
	const LOCK_TIMEOUT_EXPIRATION: u64 = FETCH_TIMEOUT_PERIOD + 1000; // in milli-seconds
	const LOCK_BLOCK_EXPIRATION: u32 = 3; // in block number

	/* 	//Admin Metadata
	#[derive(Debug, Clone, Encode, Decode, PartialEq, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct MetaData<T: Config> {
		pub issuance: BalanceOf<T>,
		pub minter: T::AccountId,
		pub burner: T::AccountId,
	} */

	// Struct for Asset to be Data Bundle or 2) Data Service
	// Derive macros of helper traits
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Asset<T: Config> {
		//pub id: T::Hash,
		pub price: BalanceOf<T>,
		//assettype: AssetType,
		pub asset_criteria: u64,
		pub asset_validation: u64,
		pub priceThreshold: BalanceOf<T>,
		pub asset_privacy: u64,
	}

	// Helper Functions
	impl<T: Config> Asset<T> {
		pub fn assettype(typehash: T::Hash) -> AssetType {
			if typehash.as_ref()[0] % 2 == 0 {
				AssetType::Radar_Data
			} else {
				AssetType::Radar_Road_Signature
			}
		}
	}

	// Struct for Asset Type to be Data Service
	// Derive macros of helper traits
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct AssetService<T: Config> {
		//pub id: T::Hash,
		pub price: BalanceOf<T>,
		//assettype: AssetType,
		pub assetservice_criteria: u64,
		pub assetservice_validation: u64,
		pub priceThreshold: BalanceOf<T>,
		pub assetservice_privacy: u64,
	}

	// Derive macros of helper traits
	#[derive(Encode, Decode, Debug, Clone, PartialEq)]
	pub enum AssetType {
		Radar_Data,
		Radar_Road_Signature,
	}

	// Function to configure Default Value for AssetType as Radar data
	impl Default for AssetType {
		fn default() -> Self {
			AssetType::Radar_Data
		}
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: Currency<Self::AccountId>;

		//Randomness trait from frame_support requires specifying with H256
		// Random type need to be implemented in the runtime for configuration
		type AssetRandomness: Randomness<Self::Hash, Self::BlockNumber>;

		type AssetServiceRandomness: Randomness<Self::Hash, Self::BlockNumber>;

		//#[pallet::constant]
		//type SpecialAccountId: Get<Self::AccountId>;

		// The type used to store balances.
		//type Balanced: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;
	}

	//Errors
	#[pallet::error]
	pub enum Error<T> {
		// Nonce overflowed past bit limit
		NonceOverflow,
		HttpFetchingError,
		// Token Errors
		// An account would go below the minimum balance if the operation were executed.
		BelowMinBalance,
		// The origin account does not have the required permission for the operation.
		NoPermission,
		/// An operation would lead to an overflow.
		Overflow,
		/// An operation would lead to an underflow.
		Underflow,
		// Cannot burn the balance of a non-existent account.
		CannotBurnEmpty,
		// There is not enough balance in the sender's account for the transfer.
		InsufficientBalance,
	}

	#[pallet::event]
	//#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Create an Asset
		CreatedAsset(T::AccountId, T::Hash),
		//Price Set for an Asset
		PriceSetAsset(T::AccountId, T::Hash, BalanceOf<T>),
		//Transfer the asset
		TransferredAsset(T::AccountId, T::AccountId, T::Hash),
		//Buy the Asset
		BoughtAsset(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),

		// Will allow the client to call external API to retrieve the vehicle Data
		PrepareAsset(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>, u64),

		CreatedAssetService(T::AccountId, T::Hash),
		//Price Set for an Asset
		PriceSetAssetService(T::AccountId, T::Hash, BalanceOf<T>),
		//Transfer the asset
		TransferredAssetService(T::AccountId, T::AccountId, T::Hash),
		//Buy the Asset
		BoughtAssetService(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>),
		// Will allow the client to call external API to retrieve the vehicle Data
		PrepareAssetService(T::AccountId, T::AccountId, T::Hash, BalanceOf<T>, u64),

		SubmittedAssetServiceReview(T::AccountId, T::Hash),

		// Token Events
		/// New token supply was minted.
		MintedNewSupply(T::AccountId),
		/// Tokens were successfully transferred between accounts. [from, to, value]
		Transferred(T::AccountId, T::AccountId, BalanceOf<T>),

		//Added Admin Member
		AddedAssetAdmin(T::AccountId),

		AddedEscrowAccount(T::AccountId),
		//Added Admin Member
		AddedAssetServiceAdmin(T::AccountId),
		//Added Vehicle Member
		AddedVehicle(T::AccountId),

		NewAssetDemand(u64, BalanceOf<T>, u64),
		BroadcastAssetService(T::AccountId, T::Hash, u64),

		SubmittedAssetReview(T::AccountId, T::Hash),
	}

	// Storage for the Different Asset
	//Keep track of the nonce for randomness
	//pub (super) --> Keep visibility for the whole parent module
	// Ref: https://substrate.dev/docs/en/knowledgebase/runtime/storage
	//Storage Value or Map or Double Map or N Map

	// Store the Asset Admins
	// Storage can be of type Config as in Genesis Config
	// Key -> Hash of Asset, Value -> Account Owner
	#[pallet::storage]
	#[pallet::getter(fn escrowaccount)]
	pub(super) type EscrowAdmin<T: Config> =
		StorageMap<_, Twox64Concat, u64, T::AccountId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn assetadmins)]
	pub(super) type AssetAdmin<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, u64, ValueQuery>;

	// Store the Asset Service Admins
	#[pallet::storage]
	#[pallet::getter(fn assetserviceadmins)]
	// Storage can be of type Config as in Genesis Config
	// Key -> Hash of Asset, Value -> Account Owner
	pub(super) type AssetServiceAdmin<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, u64, ValueQuery>;

	//Store the Vehicle Relation with Asset Admin (Vehicle OEM)
	// Key: Vehicle Id,Vehicle OEM; (IF present)
	#[pallet::storage]
	#[pallet::getter(fn vehicles)]
	pub(super) type Vehicle<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, T::AccountId, ValueQuery>;

	#[pallet::storage]
	//To implement the method get_nonce
	#[pallet::getter(fn get_nonce)]
	pub(super) type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	//Storage for Outstanding Assets
	#[pallet::getter(fn get_outstandingassetindexes)]
	pub(super) type OutstandingAssetIndex<T: Config> = StorageValue<_, u64, ValueQuery>;
	#[pallet::storage]
	//Storage for Outstanding AssetServices
	#[pallet::getter(fn get_outstandingassetserviceindexes)]
	pub(super) type OutstandingAssetServiceIndex<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	//To implement the method get_nonce
	#[pallet::getter(fn get_assetservicenonce)]
	pub(super) type AssetServiceNonce<T: Config> = StorageValue<_, u64, ValueQuery>;

	// Store an Asset: its property and price
	#[pallet::storage]
	#[pallet::getter(fn assets)]
	// Storage for the Asset Objects as Key, Value
	// Key --> Hash, Value --> Asset Object Struct
	// HashType Twox64Concat
	pub(super) type Assets<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Asset<T>>;

	// Store Asset Service
	#[pallet::storage]
	#[pallet::getter(fn assetservices)]
	// Storage for the Asset Objects as Key, Value
	// Key --> Hash, Value --> Asset Object Struct
	// HashType Twox64Concat
	pub(super) type AssetServices<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, AssetService<T>>;

	// Store the Owner of the Asset
	#[pallet::storage]
	#[pallet::getter(fn owner_of)]
	// Storage can be of type Config as in Genesis Config
	// Key -> Hash of Asset, Value -> Account Owner
	pub(super) type AssetOwner<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, Option<T::AccountId>, ValueQuery>;

	// Store the Owner of the AssetService
	#[pallet::storage]
	#[pallet::getter(fn owner_of_assetservice)]
	// Storage can be of type Config as in Genesis Config
	// Key -> Hash of Asset, Value -> Account Owner
	pub(super) type AssetServiceOwner<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, Option<T::AccountId>, ValueQuery>;

	// Index for the Assets
	#[pallet::storage]
	#[pallet::getter(fn asset_by_index)]
	// Key -> U64 index, Value --> Hash of Asset
	pub(super) type AllAssetsArray<T: Config> =
		StorageMap<_, Twox64Concat, u64, T::Hash, ValueQuery>;
	// Index for the Assets
	#[pallet::storage]
	#[pallet::getter(fn asset_service_by_index)]
	// Key -> U64 index, Value --> Hash of Asset
	pub(super) type AllAssetServicesArray<T: Config> =
		StorageMap<_, Twox64Concat, u64, T::Hash, ValueQuery>;

	//Total Assets
	#[pallet::storage]
	#[pallet::getter(fn all_asset_count)]
	pub(super) type AllAssetsCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	//Total Asset Services
	#[pallet::storage]
	#[pallet::getter(fn all_asset_service_count)]
	pub(super) type AllAssetServicesCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	//Keeps Track of all the asset
	#[pallet::storage]
	// Key-> Hash of Asset, Value --> Index
	pub(super) type AllAssetsIndex<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, u64, ValueQuery>;

	//Keeps Track of all the asset
	#[pallet::storage]
	// Key-> Hash of Asset, Value --> Index
	pub(super) type AllAssetServicesIndex<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, u64, ValueQuery>;

	// Keep track of who owns asset by index
	#[pallet::storage]
	#[pallet::getter(fn asset_of_owner_by_index)]
	// Double Key and Value Map
	// Key: AccountId and Index of Asset | Value: Hash of Asset
	pub(super) type OwnedAssetsArray<T: Config> =
		StorageMap<_, Twox64Concat, (T::AccountId, u64), T::Hash, ValueQuery>;
	// Keep track of who owns asset by index
	#[pallet::storage]
	#[pallet::getter(fn asset_service_of_owner_by_index)]
	// Double Key and Value Map
	// Key: AccountId and Index of Asset | Value: Hash of Asset
	pub(super) type OwnedAssetServicesArray<T: Config> =
		StorageMap<_, Twox64Concat, (T::AccountId, u64), T::Hash, ValueQuery>;

	// Keep Track of the Assets owned by a particular account
	#[pallet::storage]
	#[pallet::getter(fn owned_asset_count)]
	// Key: AccountId | Value: Asset Index
	pub(super) type OwnedAssetsCount<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, u64, ValueQuery>;
	// Keep Track of the Assets owned by a particular account
	#[pallet::storage]
	#[pallet::getter(fn owned_asset_service_count)]
	// Key: AccountId | Value: Asset Index
	pub(super) type OwnedAssetServicesCount<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, u64, ValueQuery>;

	//Keep track of all owned Assets by index
	#[pallet::storage]
	//Key: Asset Hash , Value: Index of Asset
	pub(super) type OwnedAssetsIndex<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, u64, ValueQuery>;
	//Keep track of all owned Assets by index
	#[pallet::storage]
	//Key: Asset Hash , Value: Index of Asset
	pub(super) type OwnedAssetServicesIndex<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, u64, ValueQuery>;

	//Proof
	//Key: Asset Hash Id, Index of the Proof || Account of the Vehicle; Value:  Data Hash
	#[pallet::storage]
	#[pallet::getter(fn get_assetproof)]
	pub(super) type AssetProof<T: Config> =
		StorageMap<_, Twox64Concat, (T::Hash, u64), (T::AccountId, T::Hash), ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_assetproofcounter)]
	pub(super) type AssetProofCounter<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, u64, ValueQuery>;

	//Proof
	//Key: Asset Hash Id, Index of the Proof || Account of the Vehicle; Value:  Data Hash
	#[pallet::storage]
	#[pallet::getter(fn get_assetprooffinal)]
	pub(super) type AssetProofFinal<T: Config> =
		StorageMap<_, Twox64Concat, (T::Hash, u64), (T::AccountId, T::Hash), ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_assetprooffinalcounter)]
	pub(super) type AssetProofFinalCounter<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, u64, ValueQuery>;

	//Proof
	//Key: Asset Service Hash Id, Index of the proof || Account of the Manufacturer; Value:  Data Hash
	#[pallet::storage]
	#[pallet::getter(fn get_assetserviceproof)]
	pub(super) type AssetServiceProof<T: Config> =
		StorageMap<_, Twox64Concat, (T::Hash, u64), (T::AccountId, T::Hash), ValueQuery>;
	//Proof Counter
	//Key: Asset Service Hash Id,|| Proof Counter
	#[pallet::storage]
	#[pallet::getter(fn get_assetserviceproofcounter)]
	pub(super) type AssetServiceProofCounter<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_assetserviceprooffinalcounter)]
	pub(super) type AssetServiceProofFinalCounter<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, u64, ValueQuery>;
	#[pallet::storage]
	#[pallet::getter(fn get_assetserviceprooffinal)]
	pub(super) type AssetServiceProofFinal<T: Config> =
		StorageMap<_, Twox64Concat, (T::Hash, u64), (T::AccountId, T::Hash), ValueQuery>;

	//First Asset Bidding
	// Key: AssetID,BidIndex ; Value : Account of the Bidder, Bid Value, Encrypted Bid Value
	#[pallet::storage]
	#[pallet::getter(fn get_firstassetbid)]
	pub(super) type FirstAssetBid<T: Config> = StorageMap<
		_,
		Twox64Concat,
		(T::Hash, u64),
		(T::AccountId, BalanceOf<T>, Vec<u8>),
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_firstassetbidcounter)]
	pub(super) type FirstAssetBidCounter<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, u64, ValueQuery>;

	//First Asset Bidding
	// Key: AssetID,VehicleOEM,Counter ; Value : Account of the Vehicle Intereseted, Price Value of its interest
	#[pallet::storage]
	#[pallet::getter(fn get_serviceinterest)]
	pub(super) type ServiceInterest<T: Config> = StorageMap<
		_,
		Twox64Concat,
		(T::Hash, T::AccountId, u64),
		(T::AccountId, BalanceOf<T>),
		ValueQuery,
	>;

	// key:{AssetID, Vehicle OEM ID} Value:{Service Counter, Total Price of all Service Interest}
	#[pallet::storage]
	#[pallet::getter(fn get_serviceinterestcounter)]
	pub(super) type ServiceInterestCounter<T: Config> =
		StorageMap<_, Twox64Concat, (T::Hash, T::AccountId), (u64, BalanceOf<T>), ValueQuery>;

	//First Asset Service Bidding
	// Key: AssetID,BidIndex ; Value : Account of the Bidder, Bid Value, Encrypted Bid Value
	#[pallet::storage]
	#[pallet::getter(fn get_firstassetservicebid)]
	pub(super) type FirstAssetServiceBid<T: Config> = StorageMap<
		_,
		Twox64Concat,
		(T::Hash, u64),
		(T::AccountId, BalanceOf<T>, Vec<u8>),
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_firstassetservicebidcounter)]
	pub(super) type FirstAssetServiceBidCounter<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, u64, ValueQuery>;

	//Oauth Part
	//Finalised Asset Counter
	#[pallet::storage]
	//To implement the method get_nonce
	#[pallet::getter(fn get_finalisedassetcounter)]
	pub(super) type FinalisedAssetCounter<T: Config> = StorageValue<_, u64, ValueQuery>;

	//Oauth Part
	//Finalised Asset Counter for Offchain Check
	//To Compare against the above FinalisedAssetCounter
	#[pallet::storage]
	//To implement the method get_nonce
	#[pallet::getter(fn get_finalisedassetcounteroffchain)]
	pub(super) type FinalisedAssetCounterOffchain<T: Config> = StorageValue<_, u64, ValueQuery>;

	//Finalised Asset Ids
	#[pallet::storage]
	//To implement the method get_nonce
	//Key: Finalised Asset Counter
	//Id of Asset; AccountId: Id of New Owner
	#[pallet::getter(fn get_finalisedassetid)]
	pub(super) type FinalisedAssetId<T: Config> =
		StorageMap<_, Twox64Concat, u64, (T::Hash, T::AccountId), ValueQuery>;

	//Storage for Oauth Assets
	//Key: AssetId, Value: (Asset Owner, OauthToken)
	#[pallet::storage]
	#[pallet::getter(fn get_oauthasset)]
	pub(super) type OauthAsset<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, (T::AccountId, Vec<u8>), ValueQuery>;

	//Finalised Asset Service Counter
	#[pallet::storage]
	//To implement the method get_nonce
	#[pallet::getter(fn get_finalisedassetservicecounter)]
	pub(super) type FinalisedAssetServiceCounter<T: Config> = StorageValue<_, u64, ValueQuery>;

	//Finalised Asset Service Counter
	#[pallet::storage]
	//To implement the method get_nonce
	#[pallet::getter(fn get_finalisedassetservicecounteroffchain)]
	pub(super) type FinalisedAssetServiceCounterOffchain<T: Config> =
		StorageValue<_, u64, ValueQuery>;

	//Finalised Asset Ids
	#[pallet::storage]
	//To implement the method get_nonce
	#[pallet::getter(fn get_finalisedassetserviceid)]
	pub(super) type FinalisedAssetServiceId<T: Config> =
		StorageMap<_, Twox64Concat, u64, (T::Hash, T::AccountId), ValueQuery>;
	//Storage for Oauth Assets
	//Key: AssetId, Value: (Asset Owner, OauthToken)
	#[pallet::storage]
	#[pallet::getter(fn get_oauthassetservice)]
	pub(super) type OauthAssetService<T: Config> =
		StorageMap<_, Twox64Concat, (T::Hash, T::AccountId), Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_assetreview)]
	//Key: Asset Id, Value: (Total Review, Total Reviewers)
	pub(super) type AssetReview<T: Config> =
		StorageMap<_, Twox64Concat, (T::Hash), (u64, u64), ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_assetreviewer)]
	//Key: (Asset Id, Reviewer AccountID) Value: (Filler as 1)
	pub(super) type AssetReviewer<T: Config> =
		StorageMap<_, Twox64Concat, (T::Hash, T::AccountId), u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_assetservicereview)]
	//Key: Asset Service Id, Value: (Total Review, Total Reviewers, Reviewer AccountID)
	pub(super) type AssetServiceReview<T: Config> =
		StorageMap<_, Twox64Concat, (T::Hash), (u64, u64), ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_assetservicereviewer)]
	//Key: (Asset Service Id, Reviewer AccountID) Value: (Filler as 1)
	pub(super) type AssetServiceReviewer<T: Config> =
		StorageMap<_, Twox64Concat, (T::Hash, T::AccountId), u64, ValueQuery>;

	//Privacy
	#[pallet::storage]
	#[pallet::getter(fn get_assetpubkey)]
	//Key: Asset Id, Value: Public Key to Encrypt
	pub(super) type AssetKey<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Vec<u8>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_assetservicepubkey)]
	//Key: Asset Service Id, Value: Public Key to Encrypt
	pub(super) type AssetServiceKey<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, Vec<u8>, ValueQuery>;

	// Token Part
	/// Storage item for balances to accounts mapping.
	//#[pallet::storage]
	//#[pallet::getter(fn get_balance)]
	//pub(super) type BalanceToAccount<T: Config> =
	//StorageMap<_, Blake2_128Concat, T::AccountId, T::Balanced, ValueQuery>;

	/* #[pallet::storage]
	#[pallet::getter(fn meta_data)]
	pub(super) type MetaDataStore<T: Config> =
		StorageValue<_, MetaData<T::AccountId, BalanceOf<T>>>; */
	// End Token Storage

	// Pallet Genesis Config
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub assets: Vec<(T::AccountId, T::Hash, BalanceOf<T>, u64, u64, BalanceOf<T>, u64)>,
		pub assetservices: Vec<(T::AccountId, T::Hash, BalanceOf<T>, u64, u64, BalanceOf<T>, u64)>,
		pub admin: T::AccountId,
	}

	// Required to implement default for GenesisConfig<T>
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig { assets: vec![], assetservices: vec![], admin: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			/* for &(ref acct, hash, balance, criteria, pricethreshold) in &self.assets {
				let ast = Asset {
					id: hash,
					price: balance,
					asset_criteria: criteria,
					priceThreshold: pricethreshold,
				};
				let _ = <Module<T>>::mintasset(acct.clone(), hash, ast);
			}
			for &(ref acct, hash, balance, criteria, pricethreshold) in &self.assetservices {
				let ast_service = AssetService {
					id: hash,
					price: balance,
					assetservice_criteria: criteria,
					priceThreshold: pricethreshold,
				};
				let _ = <Module<T>>::mintassetservice(acct.clone(), hash, ast_service);
			} */
			/* MetaDataStore::<T>::put(MetaData {
				issuance: Zero::zero(),
				minter: self.admin.clone(),
				burner: self.admin.clone(),
			}); */
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// By implementing `fn offchain_worker` you declare a new offchain worker.
		/// This function will be called when the node is fully synced and a new best block is
		/// succesfuly imported.
		/// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
		/// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
		/// so the code should be able to handle that.
		/// You can use `Local Storage` API to coordinate runs of the worker.
		fn offchain_worker(block_number: T::BlockNumber) {
			// Note that having logs compiled to WASM may cause the size of the blob to increase
			// significantly. You can use `RuntimeDebug` custom derive to hide details of the types
			// in WASM. The `sp-api` crate also provides a feature `disable-logging` to disable
			// all logging and thus, remove any logging from the WASM.
			log::info!("Hello World from offchain workers!");

			// Since off-chain workers are just part of the runtime code, they have direct access
			// to the storage and other included pallets.
			//
			// We can easily import `frame_system` and retrieve a block hash of the parent block.
			let parent_hash = <system::Pallet<T>>::block_fhash(block_number - 1u32.into());
			log::debug!("Current block: {:?} (parent hash: {:?})", block_number, parent_hash);

			// It's a good practice to keep `fn offchain_worker()` function minimal, and move most
			// of the code to separate `impl` block.
			// Here we call a helper function to calculate current average price.
			// This function reads storage entries of the current state.

			////////////////////////////////------ASSET FINALISATION------------////////////////////////////////////
			//Conditions to check
			// Iterate from finalised counter
			//1) Check counter of finalised counter against finalised counter offchain
			//2) Get the asset id based on finalised counter +1
			//3) Check if there is proof added in Asset Proof at the vehicle level atleast 3
			//4) Then check the final Proof of the OEM added
			//5) Calculate escrow amount as: asset_amount / no of vehicles who submitted proof +1
			//5) Then transfer from escrow to all the the accounts who submitted proof
			//5) Transfer the asset to the owner
			//6) Generate the Token and emit an event

			//TODO: SEND CRITERIA
			// Should Expect HASH of JWT and store it
			let outstanding_asset_ind = Self::get_outstandingassetindexes();
			if (outstanding_asset_ind > 0) {
				//TODO: SEND CRITERIA
				// Should Expect HASH of JWT and store it
				let oauth_token: Vec<u8> = Self::fetch_from_remote_post_asset().unwrap();
				//let oauth_hash = hashing::keccak_256(oauth_token.as_bytes());
			}												
			/////////////////////////////////////:::::::::::::::::::::::////////////////////////////////////

			////////////////////////////////------ASSET SERVICE FINALISATION------------////////////////////////////////////
			//Conditions to check
			// Iterate from finalised counter
			//1) Check counter of finalised counter against finalised counter offchain
			//2) Get the asset id based on finalised counter +1
			//3) Check if there is proof added in Asset Proof at the vehicle level atleast 3
			//4) Then check the final Proof of the OEM added
			//5) Calculate escrow amount as: asset_amount / no of vehicles who submitted proof +1
			//5) Then transfer from escrow to all the the accounts who submitted proof
			//5) Transfer the asset to the owner
			//6) Generate the Token and emit an event

			let outstanding_asset_service_ind = Self::get_outstandingassetserviceindexes();
			if (outstanding_asset_service_ind > 0) {
				//TODO: SEND ASSET SERVICE CRITERIA &&
				// Should Expect HASH of JWT and store it
				let oauth_token: Vec<u8> = Self::fetch_from_remote_post_asset_service().unwrap();
				//let oauth_hash = hashing::keccak_256(oauth_token.as_bytes());
			}
				}
			}
		}
	}

	//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

	// Define the dispatchable functions as extrinsics which are analagous to transactions and return dispatch result
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//////////////////////////////////////////////////////////////////
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn addEscrow(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			// Adding Rights only for Minter Admin
			//let mut meta = Self::meta_data();
			//ensure!(sender == Self::escrowaccount(0), Error::<T>::NoPermission);
			// Update storage.
			<EscrowAdmin<T>>::insert(0, &sender);
			// Emit an event.
			Self::deposit_event(Event::AddedEscrowAccount(sender));
			// Return a successful DispatchResultWithPostInfo.
			Ok(().into())
		}

		//////////////////////////////////////////////////////////////////
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn addAssetAdmin(
			origin: OriginFor<T>,
			assetAdmin: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			// Adding Rights only for Minter Admin
			//let mut meta = Self::meta_data();
			ensure!(sender == Self::escrowaccount(0), Error::<T>::NoPermission);
			// Update storage.
			<AssetAdmin<T>>::insert(assetAdmin.clone(), 1);
			// Emit an event.
			Self::deposit_event(Event::AddedAssetAdmin(assetAdmin));
			// Return a successful DispatchResultWithPostInfo.
			Ok(().into())
		}
		//////////////////////////////////////////////////////////////////
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn addAssetServiceAdmin(
			origin: OriginFor<T>,
			assetServiceAdmin: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			// Adding Rights only for Minter Admin
			//let mut meta = Self::meta_data();
			ensure!(sender == Self::escrowaccount(0), Error::<T>::NoPermission);
			// Update storage.
			<AssetServiceAdmin<T>>::insert(assetServiceAdmin.clone(), 1);
			// Emit an event.
			Self::deposit_event(Event::AddedAssetServiceAdmin(assetServiceAdmin));
			// Return a successful DispatchResultWithPostInfo.
			Ok(().into())
		}

		//////////////////////////////////////////////////////////////////
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn addVehicle(
			origin: OriginFor<T>,
			vehicle: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			// Minting Rights only for Minter Admin
			//let mut meta = Self::meta_data();
			ensure!(<AssetAdmin<T>>::contains_key(sender.clone()), "Admin does not exist");

			ensure!(!<Vehicle<T>>::contains_key(vehicle.clone()), "Vehicle already added");
			// Update storage.
			// Inserted Vehicles against its OEM
			<Vehicle<T>>::insert(vehicle.clone(), sender);
			// Emit an event.
			Self::deposit_event(Event::AddedVehicle(vehicle));
			// Return a successful DispatchResultWithPostInfo.
			Ok(().into())
		}

		//////////////////////////////////////////////////////////////////
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn createAssetDemand(
			origin: OriginFor<T>,
			criteria: u64,
			validation: u64,
			price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			// Adding Rights only for Minter Admin
			ensure!(<AssetServiceAdmin<T>>::contains_key(sender), "Asset Admin does not exist");
			// Emit an event.
			// Validation: 0 => No | 1=> Yes Pre-emptive
			Self::deposit_event(Event::NewAssetDemand(criteria, price, validation));
			// Return a successful DispatchResultWithPostInfo.
			Ok(().into())
		}

		///////////////////////////////////////////////////////
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn mint(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			// Minting Rights only for Minter Admin
			//let mut meta = Self::meta_data();
			ensure!(sender == Self::escrowaccount(0), Error::<T>::NoPermission);
			// Update storage.
			let escrow_account = Self::escrowaccount(0);

			T::Currency::deposit_into_existing(&escrow_account, amount)?;

			// Emit an event.
			Self::deposit_event(Event::MintedNewSupply(sender));
			// Return a successful DispatchResultWithPostInfo.
			Ok(().into())
		}
		///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn transferfromescrow(
			origin: OriginFor<T>,
			to: T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			// Minting Rights only for Minter Admin
			//let mut meta = Self::meta_data();
			ensure!(sender == Self::escrowaccount(0), Error::<T>::NoPermission);
			// Update storage.
			let escrow_account = Self::escrowaccount(0);

			// Updating balances by currency trait
			T::Currency::transfer(&sender, &to, amount, ExistenceRequirement::KeepAlive)?;

			// Emit an event.
			Self::deposit_event(Event::Transferred(sender, to, amount));
			// Return a successful DispatchResultWithPostInfo.
			Ok(().into())
		}
		///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		/* 		/// Allow minting account to transfer a given balance to another account.
		///
		/// Parameters:
		/// - `to`: The account to receive the transfer.
		/// - `amount`: The amount of balance to transfer.
		///
		/// Emits `Transferred` event when successful.
		///
		/// TODO: Add checks on minimum balance required and maximum transferrable balance.
		/// Weight: `O(1)`
		#[pallet::weight(100)]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			#[pallet::compact] amount: T::Balanced,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let sender_balance = Self::get_balance(&sender);
			let receiver_balance = Self::get_balance(&to);
			// TODO:: Check Balance

			// Minting Rights only for Minter Admin
			let mut meta = Self::meta_data();
			ensure!(sender == meta.minter, Error::<T>::NoPermission);

			// Calculate new balances.
			let update_sender = sender_balance.saturating_sub(amount);
			let update_to = receiver_balance.saturating_add(amount);

			// Update both accounts storage.
			<BalanceToAccount<T>>::insert(&sender, update_sender);
			<BalanceToAccount<T>>::insert(&to, update_to);

			// Emit event.
			Self::deposit_event(Event::Transferred(sender, to, amount));
			Ok(().into())
		} */
		/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn create_asset(
			origin: OriginFor<T>,
			first_price: BalanceOf<T>,
			first_criteria: u64,
			validation: u64,
			price_threshold: BalanceOf<T>,
			privacy: u64,
			asset_pub_key: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let asset_creator = sender.clone();
			let asset_creator_event = sender.clone();
			ensure!(<AssetAdmin<T>>::contains_key(asset_creator), "Asset Admin does not exist");
			// Hashing the sender
			let random_hash = Self::random_hash(&sender);

			let new_asset = Asset::<T> {
				//id: random_hash,
				price: first_price,
				asset_criteria: first_criteria,
				asset_validation: validation,
				priceThreshold: price_threshold,
				asset_privacy: privacy,
				//assettype: Asset::<T, T>::assettype(random_hash),
			};
			if (privacy == 1) {
				//Insert the public key for an asset
				<AssetKey<T>>::insert(random_hash, asset_pub_key);
			}
			Self::mintasset(sender, random_hash, new_asset)?;
			Self::increment_nonce()?;
			//let bodie = Bod { username: "renault", password: "leat" };CreatedAsset
			//let encoded: Vec<u8> = vec!["username: renault", "password: leat"];
			// Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
			// Emit event.
			Self::deposit_event(Event::CreatedAsset(asset_creator_event, random_hash));
			Ok(().into())
		}
		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// Set the price of an asset
		/// Weight: `O(1)`
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn set_asset_price(
			origin: OriginFor<T>,
			asset_id: T::Hash,
			new_price: BalanceOf<T>,
			threshold_price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let asset_price_sender = sender.clone();
			let asset_price_sender_event = sender.clone();
			ensure!(<AssetAdmin<T>>::contains_key(sender), "Asset Admin does not exist");
			ensure!(<Assets<T>>::contains_key(asset_id), "Asset does not exist");
			//Get the owner of an asset
			let owner = Self::owner_of(asset_id).ok_or("No owner for this asset")?;

			ensure!(owner == asset_price_sender, "You are not the owner of the asset");

			let mut asset = Self::assets(asset_id).unwrap();
			asset.price = new_price;
			asset.priceThreshold = threshold_price;

			//Update the asset information to storage
			<Assets<T>>::insert(asset_id, asset);

			//Deposit a "PriceSet" event
			Self::deposit_event(Event::PriceSetAsset(
				asset_price_sender_event,
				asset_id,
				new_price,
			));
			Ok(().into())
		}
		////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// Buy an asset
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn buy_asset(
			origin: OriginFor<T>,
			asset_id: T::Hash,
			ask_price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let buy_asset_sender = sender.clone();
			let buy_asset_sender_event = sender.clone();
			ensure!(
				<AssetServiceAdmin<T>>::contains_key(sender),
				"Asset Sevice Admin does not exist"
			);
			ensure!(<Assets<T>>::contains_key(asset_id), "This asset does not exist");

			let owner = Self::owner_of(asset_id).ok_or("No owner for this asset")?;

			//Check that account buying the asset doesnt already own the asset
			ensure!(owner != buy_asset_sender, "You are the owner of the asset already");
			// Get the existing quoted price of the asset by the Radar-manufacturer or OEM
			let mut asset = Self::assets(asset_id).unwrap();
			let asset_price = asset.price;

			let asset_criteria = asset.asset_criteria;

			let asset_threshold_price = asset.priceThreshold;
			// Check if the asset is for sale
			ensure!(!asset_price.is_zero(), "This asset is not for sale!");
			// TO DO
			//ensure!(ask_price < , "This bid is lower");

			// Asset Threshold is usual higher
			ensure!(
				ask_price >= asset_threshold_price,
				"Price is not within threshold Too Low, Consider Bidding"
			);
			// Updating balances by currency trait
			// Transfer To Escrow Account
			//let mut meta = Self::meta_data();
			let escrow_account = Self::escrowaccount(0);
			// Updating balances by currency trait
			T::Currency::transfer(
				&buy_asset_sender,
				&escrow_account,
				ask_price,
				ExistenceRequirement::KeepAlive,
			)?;
			//Set the price of the asset to the new price it was sold
			asset.price = ask_price.into();
			<Assets<T>>::insert(asset_id, asset);
			//Self::deposit_event(Event::BoughtAsset(sender, owner, asset_id, asset_price));
			// Increment the asset counter
			let final_asset_counter = Self::get_finalisedassetcounter();

			// Put the transferred asset along with the sender
			<FinalisedAssetId<T>>::insert(final_asset_counter, (asset_id, &buy_asset_sender));

			let new_final_asset_counter =
				final_asset_counter.checked_add(1).ok_or("Incremented the Final Asset")?;
			<FinalisedAssetCounter<T>>::put(new_final_asset_counter);

			Self::deposit_event(Event::PrepareAsset(
				buy_asset_sender_event,
				owner,
				asset_id,
				ask_price,
				asset_criteria,
			));
			Ok(().into())
		}
		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// Buy an asset
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn submit_asset_proof(
			origin: OriginFor<T>,
			asset_id: T::Hash,
			proof: T::Hash,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let asset_proof_sender = sender.clone();
			let asset_proof_sender_event = sender.clone();
			ensure!(<Vehicle<T>>::contains_key(sender), "Vehicle does not exist");
			ensure!(<Assets<T>>::contains_key(asset_id), "This asset does not exist");
			let asset_proof_counter = Self::get_assetproofcounter(asset_id.clone());
			let new_asset_proof_counter =
				asset_proof_counter.checked_add(1).ok_or("Incremented the Proof Counter")?;
			<AssetProofCounter<T>>::insert(asset_id.clone(), new_asset_proof_counter);

			//Insert the actual proof
			<AssetProof<T>>::insert((asset_id, asset_proof_counter), (asset_proof_sender, proof));
			//<AssetProof<T>>::decode_len(asset_id);

			//ensure!(!(<AssetProof<T>>::contains_key(asset_id)::contains_key(sender)), "This asset does not exist");

			//<AssetProof<T>>::insert(asset_id, (sender, proof));

			Ok(().into())
		}

		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// Submit proof an asset
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn submit_asset_proof_final(
			origin: OriginFor<T>,
			asset_id: T::Hash,
			proof: T::Hash,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let submit_asset_proof_sender = sender.clone();
			ensure!(<AssetAdmin<T>>::contains_key(sender), "Asset Admin does not exist");
			ensure!(<Assets<T>>::contains_key(asset_id), "This asset does not exist");
			let asset_proof_counter = Self::get_assetprooffinalcounter(asset_id);
			let new_asset_proof_counter =
				asset_proof_counter.checked_add(1).ok_or("Incremented the Proof Counter")?;
			<AssetProofFinalCounter<T>>::insert(asset_id, new_asset_proof_counter);

			//Insert the actual proof
			<AssetProofFinal<T>>::insert(
				(asset_id, asset_proof_counter),
				(submit_asset_proof_sender, proof),
			);
			//<AssetProof<T>>::decode_len(asset_id);

			//ensure!(!(<AssetProof<T>>::contains_key(asset_id)::contains_key(sender)), "This asset does not exist");

			//<AssetProof<T>>::insert(asset_id, (sender, proof));

			Ok(().into())
		}

		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn submit_asset_review(
			origin: OriginFor<T>,
			asset_id: T::Hash,
			review: u64,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let submit_reviewer = sender.clone();
			ensure!(
				<AssetServiceAdmin<T>>::contains_key(sender.clone()),
				"Radar OEM does not exist"
			);
			ensure!(<Assets<T>>::contains_key(asset_id), "This asset does not exist");

			let radar_oem = Self::owner_of(asset_id).ok_or("No owner for this asset ")?;

			ensure!(sender.clone() == radar_oem, "You do not belong to the owner Radar OEM");
			let (Asset_Total_Review, Asset_TotalReviewers) = Self::get_assetreview(asset_id);

			if (!<AssetReviewer<T>>::contains_key((asset_id, &submit_reviewer))) {
				let new_total_review = Asset_Total_Review
					.checked_add(review)
					.ok_or("Incremented the Total Reviews")?;
				let new_Asset_TotalReviewers =
					Asset_TotalReviewers.checked_add(1).ok_or("Incremented the Total Reviewers")?;

				<AssetReviewer<T>>::insert((asset_id, &submit_reviewer), 1);

				<AssetReview<T>>::insert(asset_id, (new_total_review, new_Asset_TotalReviewers));

				let mut asset = Self::assets(asset_id).unwrap();
				let asset_price = asset.price;
				// Reward 50 currency
				let reward: u32 = 50;
				let escrow_account = Self::escrowaccount(0);
				T::Currency::transfer(
					&escrow_account,
					&sender,
					reward.try_into().ok().unwrap(),
					ExistenceRequirement::KeepAlive,
				)?;
				Self::deposit_event(Event::SubmittedAssetReview(sender.clone(), asset_id));
			}
			Ok(().into())
		}

		///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		///
		// Bid for An Asset
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn bidAsset(
			origin: OriginFor<T>,
			to: T::AccountId,
			asset_id: T::Hash,
			ask_price: BalanceOf<T>,
			encrypted_price: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let bid_asset_sender = sender.clone();
			ensure!(
				<AssetServiceAdmin<T>>::contains_key(sender),
				"Asset Sevice Admin does not exist"
			);
			//let owner = Self::owner_of(asset_id).ok_or("No owner for this asset")?;

			//ensure!(owner == sender, "Not the owner of the particular asset");
			let mut asset = Self::assets(asset_id).unwrap();

			let firstAssetBidCounter = Self::get_firstassetbidcounter(asset_id);
			let new_first_asset_bid_counter =
				firstAssetBidCounter.checked_add(1).ok_or("Incremented the Final Asset")?;
			<FirstAssetBidCounter<T>>::insert(asset_id, new_first_asset_bid_counter);
			// Put the transferred asset along with the sender
			<FirstAssetBid<T>>::insert(
				(asset_id, firstAssetBidCounter),
				(&bid_asset_sender, ask_price, encrypted_price),
			);

			Ok(().into())
		}
		////////////////////////////////////////////////////////////////////////:
		///
		// Buy an asset
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn accept_bid(
			origin: OriginFor<T>,
			asset_id: T::Hash,
			bid_threshold_price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let accept_bid_sender = sender.clone();
			ensure!(<AssetAdmin<T>>::contains_key(sender), "Asset Admin does not exist");
			ensure!(<Assets<T>>::contains_key(asset_id), "This asset does not exist");

			let owner = Self::owner_of(asset_id).ok_or("No owner for this asset")?;

			//Check that account buying the asset doesnt already own the asset
			ensure!(owner == accept_bid_sender, "You are not the owner of the asset");
			// Get the existing quoted price of the asset by the Radar-manufacturer or OEM
			//Retrieve the counter
			//Retrive the bids of the asset
			let FirstAssetBidCounter = Self::get_firstassetbidcounter(asset_id);
			//let new_first_asset_bid_counter =
			//FirstAssetBidCounter.checked_add(1).ok_or("Incremented the Final Asset")?;
			//<FirstAssetBidCounter<T>>::insert(asset_id, new_first_asset_bid_counter);
			for j in 0..FirstAssetBidCounter {
				let ((Bidder, Bid_Amount, Encrypted_Bid)) = Self::get_firstassetbid((asset_id, j));
				if (Bid_Amount >= bid_threshold_price) {
					let mut asset = Self::assets(asset_id).unwrap();

					let asset_price = asset.price;

					let asset_criteria = asset.asset_criteria;

					// Check if the asset is for sale
					ensure!(!asset_price.is_zero(), "This asset is not for sale!");
					// TO DO
					// Updating balances by currency trait
					// Transfer To Escrow Account
					//let mut meta = Self::meta_data();
					let escrow_account = Self::escrowaccount(0);
					let asset_owner = Self::owner_of(asset_id).ok_or("No owner for this asset")?;
					// Updating balances by currency trait
					T::Currency::transfer(
						&asset_owner,
						&escrow_account,
						Bid_Amount,
						ExistenceRequirement::KeepAlive,
					)?;

					//Set the price of the asset to the new price it was sold
					asset.price = Bid_Amount.into();
					<Assets<T>>::insert(asset_id, asset);
					//Self::deposit_event(Event::BoughtAsset(sender, owner, asset_id, asset_price));
					// Increment the asset counter
					let final_asset_counter = Self::get_finalisedassetcounter();
					let new_final_asset_counter =
						final_asset_counter.checked_add(1).ok_or("Incremented the Final Asset")?;
					<FinalisedAssetCounter<T>>::put(new_final_asset_counter);
					// Put the transferred asset along with the sender
					<FinalisedAssetId<T>>::insert(final_asset_counter, (asset_id, &asset_owner));

					Self::deposit_event(Event::PrepareAsset(
						asset_owner,
						Bidder,
						asset_id,
						Bid_Amount,
						asset_criteria,
					));
				}
			}
			Ok(().into())
		}

		//Function to Acept a bid in privacy mode
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn accept_bid_privacy(
			origin: OriginFor<T>,
			asset_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let accept_bid_sender = sender.clone();
			ensure!(<AssetAdmin<T>>::contains_key(sender), "Asset Admin does not exist");
			ensure!(<Assets<T>>::contains_key(asset_id), "This asset does not exist");

			let owner = Self::owner_of(asset_id).ok_or("No owner for this asset")?;

			//Check that account buying the asset doesnt already own the asset
			ensure!(owner == accept_bid_sender, "You are not the owner of the asset");
			// Get the existing quoted price of the asset by the Radar-manufacturer or OEM
			//Retrieve the counter
			//Retrive the bids of the asset
			let FirstAssetBidCounter = Self::get_firstassetbidcounter(asset_id);
			//let new_first_asset_bid_counter =
			//FirstAssetBidCounter.checked_add(1).ok_or("Incremented the Final Asset")?;
			//<FirstAssetBidCounter<T>>::insert(asset_id, new_first_asset_bid_counter);
			for j in 0..FirstAssetBidCounter {
				let ((Bidder, Bid_Amount,Encrypted_Bid)) = Self::get_firstassetbid((asset_id, j));
				let mut asset = Self::assets(asset_id).unwrap();

				let asset_price = asset.price;

				let asset_criteria = asset.asset_criteria;

				// Check if the asset is for sale
				ensure!(!asset_price.is_zero(), "This asset is not for sale!");
				// TO DO
				// Updating balances by currency trait
				// Transfer To Escrow Account
				//let mut meta = Self::meta_data();
				let escrow_account = Self::escrowaccount(0);
				let asset_owner = Self::owner_of(asset_id).ok_or("No owner for this asset")?;
				// Updating balances by currency trait
				T::Currency::transfer(
					&asset_owner,
					&escrow_account,
					Bid_Amount,
					ExistenceRequirement::KeepAlive,
				)?;

				//Set the price of the asset to the new price it was sold
				asset.price = Bid_Amount.into();
				<Assets<T>>::insert(asset_id, asset);
				//Self::deposit_event(Event::BoughtAsset(sender, owner, asset_id, asset_price));
				// Increment the asset counter
				let final_asset_counter = Self::get_finalisedassetcounter();
				let new_final_asset_counter =
					final_asset_counter.checked_add(1).ok_or("Incremented the Final Asset")?;
				<FinalisedAssetCounter<T>>::put(new_final_asset_counter);
				// Put the transferred asset along with the sender
				<FinalisedAssetId<T>>::insert(final_asset_counter, (asset_id, &asset_owner));

				Self::deposit_event(Event::PrepareAsset(
					asset_owner,
					Bidder,
					asset_id,
					Bid_Amount,
					asset_criteria,
				));
			}

			//Removing the biddding elements
			/*
			for j in 1..FirstAssetBidCounter {
			FirstAssetServiceBid<T>::remove((asset_id,j))
			}
			//Removing the bid counter
			FirstAssetBidCounter<T>::remove(asset_id);
			*/

			Ok(().into())
		}
		//////////////////////////////////////////////////////////////////////////////////////
		// Create a new Asset Service
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn create_asset_service(
			origin: OriginFor<T>,
			first_price: BalanceOf<T>,
			first_criteria: u64,
			price_threshold: BalanceOf<T>,
			assetservicein_validation: u64,
			privacy: u64,
			assetservice_pub_key: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let create_asset_sender = sender.clone();
			ensure!(
				<AssetServiceAdmin<T>>::contains_key(sender),
				"Asset Service Admin does not exist"
			);
			// Hashing the sender
			let random_hash = Self::random_service_hash(&create_asset_sender);
			let new_asset_service = AssetService {
				//id: random_hash,
				price: first_price,
				assetservice_criteria: first_criteria,
				assetservice_validation: assetservicein_validation,
				priceThreshold: price_threshold,
				assetservice_privacy:privacy,
			};
			if (privacy == 1) {
				//Insert the public key for an asset service
				<AssetServiceKey<T>>::insert(random_hash, assetservice_pub_key);
			}
			Self::mintassetservice(create_asset_sender, random_hash, new_asset_service)?;
			Self::increment_service_nonce()?;
			Ok(().into())
		}

		//Broadcast an asset service for interest
		//////////////////////////////////////////////////////////////////////////////////////
		// Create a new Asset Service
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn broadcast_asset_service(
			origin: OriginFor<T>,
			asset_service_id: T::Hash,
			criteria: u64,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let broadcast_asset_sender = sender.clone();
			ensure!(
				<AssetServiceAdmin<T>>::contains_key(sender),
				"Asset Sevice Admin does not exist"
			);
			// Hashing the sender
			Self::deposit_event(Event::BroadcastAssetService(
				broadcast_asset_sender,
				asset_service_id,
				criteria,
			));
			Ok(().into())
		}
		////////////////////////////////////////////////////////////////////////////////////
		///
		// Submit for An Asset Service by  a Vehicle
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn assetServiceInterest(
			origin: OriginFor<T>,
			asset_id: T::Hash,
			ask_price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let asset_service_interest_sender = sender.clone();
			ensure!(<Vehicle<T>>::contains_key(sender.clone()), "Vehicle does not exist");
			//let owner = Self::owner_of(asset_id).ok_or("No owner for this asset")?;

			//ensure!(owner == sender, "Not the owner of the particular asset");
			// Get my vehicle OEM Id
			let vehicleoem_id = Self::vehicles(sender.clone());

			let (serviceInterestCounter, exisitingtotalPrice) =
				Self::get_serviceinterestcounter((asset_id, vehicleoem_id.clone()));
			let newprice = exisitingtotalPrice.checked_add(&ask_price).unwrap();
			let new_service_interest_counter =
				serviceInterestCounter.checked_add(1).ok_or("Incremented the Final Asset")?;
			<ServiceInterestCounter<T>>::insert(
				(asset_id, vehicleoem_id.clone()),
				(new_service_interest_counter, newprice),
			);
			// Put the transferred asset along with the sender
			<ServiceInterest<T>>::insert(
				(asset_id, vehicleoem_id.clone(), new_service_interest_counter),
				(&asset_service_interest_sender, ask_price),
			);
			Ok(().into())
		}

		/////////////////////////////////////////////////////////////////////////////////
		// Set the price of an asset
		/// Weight: `O(1)`
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn set_asset_service_price(
			origin: OriginFor<T>,
			asset_service_id: T::Hash,
			new_price: BalanceOf<T>,
			threshold_price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let asset_service_price_sender = sender.clone();
			ensure!(
				<AssetServiceAdmin<T>>::contains_key(sender),
				"Asset Sevice Admin does not exist"
			);
			ensure!(
				<AssetServices<T>>::contains_key(asset_service_id),
				"Asset Service does not exist"
			);
			//Get the owner of an asset
			let owner = Self::owner_of_assetservice(asset_service_id)
				.ok_or("No owner for this asset service")?;

			ensure!(
				owner == asset_service_price_sender,
				"You are not the owner of the asset service"
			);

			let mut asset = Self::assetservices(asset_service_id).unwrap();
			asset.price = new_price;
			asset.priceThreshold = threshold_price;
			//Update the asset information to storage
			<AssetServices<T>>::insert(asset_service_id, asset);

			//Deposit a "PriceSet" event
			Self::deposit_event(Event::PriceSetAssetService(
				asset_service_price_sender,
				asset_service_id,
				new_price,
			));
			Ok(().into())
		}
		/* ////////////////////////////////////////////////////////////////
		///
		// Buy an asset
		#[pallet::weight(100)]
		pub fn buy_asset_service(
			origin: OriginFor<T>,
			asset_id: T::Hash,
			ask_price: T::Balance,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(<AssetServices<T>>::contains_key(asset_id), "This asset does not exist");

			let owner = Self::owner_of(asset_id).ok_or("No owner for this asset")?;

			//Check that account buying the asset doesnt already own the asset
			ensure!(owner != sender, "You are the owner of the asset already");
			// Get the existing quoted price of the asset by the Radar-manufacturer or OEM
			let mut asset = Self::assetservice(asset_id);

			let asset_price = asset.price;

			let asset_criteria = asset.asset_criteria;

			let asset_threshold_price = asset.priceThreshold;
			// Check if the asset is for sale
			ensure!(!asset_price.is_zero(), "This asset is not for sale!");
			// TO DO
			//ensure!(ask_price < , "This bid is lower");

			// Asset Threshold is usual higher
			ensure!(
				ask_price <= asset_threshold_price,
				"Price is not within threshold Too Low, Consider Bidding"
			);

			// Updating balances by currency trait
			// Transfer To Escrow Account
			let mut meta = Self::meta_data();
			let escrow_account = meta.minter;

			// Updating balances by currency trait
			<pallet_balances::Pallet<T> as Currency<_>>::transfer(
				&sender,
				&escrow_account,
				ask_price,
				ExistenceRequirement::KeepAlive,
			)?;

			//Set the price of the asset to the new price it was sold
			asset.price = ask_price.into();
			<AssetServices<T>>::insert(asset_id, asset);
			//Self::deposit_event(Event::BoughtAsset(sender, owner, asset_id, asset_price));
			// Increment the asset counter
			let final_asset_counter = Self::get_finalisedassetservicecounter();
			let new_final_asset_counter =
				final_asset_counter.checked_add(1).ok_or("Incremented the Final Asset")?;
			<FinalisedAssetServiceCounter<T>>::put(new_final_asset_counter);
			// Put the transferred asset along with the sender
			<FinalisedAssetServiceId<T>>::insert(new_final_asset_counter, (asset_id, &sender));

			Self::deposit_event(Event::PrepareAssetService(
				sender,
				owner,
				asset_id,
				ask_price,
				asset_criteria,
			));
			Ok(().into())
		} */

		//////////////////////////////////////////////////////////
		///
		// Buy an asset
		// Not Needed as of now Like in Asset
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn submit_asset_service_proof(
			origin: OriginFor<T>,
			asset_id: T::Hash,
			proof: T::Hash,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let submit_asset_service_proof_sender = sender.clone();

			ensure!(<AssetServices<T>>::contains_key(asset_id), "This asset does not exist");
			let asset_proof_counter = Self::get_assetserviceproofcounter(asset_id);
			let new_asset_proof_counter =
				asset_proof_counter.checked_add(1).ok_or("Incremented the Proof Counter")?;
			<AssetServiceProofCounter<T>>::insert(asset_id, new_asset_proof_counter);

			//Insert the actual proof
			<AssetServiceProof<T>>::insert((asset_id, asset_proof_counter), (sender, proof));
			//<AssetProof<T>>::decode_len(asset_id);

			//ensure!(!(<AssetProof<T>>::contains_key(asset_id)::contains_key(sender)), "This asset does not exist");

			//<AssetProof<T>>::insert(asset_id, (sender, proof));

			Ok(().into())
		}
		////////////////////////////////////////////////////////////////
		///
		//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
		// Buy an asset
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn submit_asset_service_proof_final(
			origin: OriginFor<T>,
			asset_id: T::Hash,
			proof: T::Hash,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let submit_asset_service_proof_sender = sender.clone();

			ensure!(<AssetServices<T>>::contains_key(asset_id), "This asset does not exist");
			let asset_proof_counter = Self::get_assetserviceprooffinalcounter(asset_id);
			let new_asset_proof_counter =
				asset_proof_counter.checked_add(1).ok_or("Incremented the Proof Counter")?;
			<AssetServiceProofFinalCounter<T>>::insert(asset_id, new_asset_proof_counter);

			//Insert the actual proof
			<AssetServiceProofFinal<T>>::insert((asset_id, asset_proof_counter), (sender, proof));
			//<AssetProof<T>>::decode_len(asset_id);

			//ensure!(!(<AssetProof<T>>::contains_key(asset_id)::contains_key(sender)), "This asset does not exist");

			//<AssetProof<T>>::insert(asset_id, (sender, proof));

			Ok(().into())
		}

		// Submit Review an asset
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn submit_asset_service_review(
			origin: OriginFor<T>,
			asset_service_id: T::Hash,
			review: u64,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let submit_asset_service_reviewer = sender.clone();
			ensure!(
				<AssetServices<T>>::contains_key(asset_service_id),
				"This asset service does not exist"
			);
			ensure!(
				<Vehicle<T>>::contains_key(submit_asset_service_reviewer.clone()),
				"You are not a valid vehicle"
			);
			let vehicle_oem = Self::owner_of_assetservice(asset_service_id)
				.ok_or("No owner for this asset service")?;

			ensure!(
				vehicle_oem == Self::vehicles(submit_asset_service_reviewer.clone()),
				"You do not belong to the owner Vehicle OEM"
			);
			let ((Asset_Total_Review, Asset_TotalReviewers)) =
				Self::get_assetservicereview(asset_service_id);

			if (!<AssetServiceReviewer<T>>::contains_key((
				asset_service_id,
				&submit_asset_service_reviewer,
			))) {
				let new_total_review = Asset_Total_Review
					.checked_add(review)
					.ok_or("Incremented the Total Reviews")?;
				let new_Asset_TotalReviewers =
					Asset_TotalReviewers.checked_add(1).ok_or("Incremented the Total Reviewers")?;

				<AssetServiceReviewer<T>>::insert(
					(asset_service_id, submit_asset_service_reviewer),
					1,
				);

				<AssetServiceReview<T>>::insert(
					asset_service_id,
					(new_total_review, new_Asset_TotalReviewers),
				);

				let mut asset = Self::assetservices(asset_service_id).unwrap();
				let asset_price = asset.price;
				// Reward 20
				let reward: u32 = 20;
				let escrow_account = Self::escrowaccount(0);
				T::Currency::transfer(
					&escrow_account,
					&sender,
					reward.try_into().ok().unwrap(),
					ExistenceRequirement::KeepAlive,
				)?;
				Self::deposit_event(Event::SubmittedAssetServiceReview(
					sender.clone(),
					asset_service_id,
				));
			}
			Ok(().into())
		}
		///////////////////////////////////////////////////////
		///

		//Finalise an asset
		// Transfer an asset between Radar Manuacturer and OEM
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn transferAssetService(
			origin: OriginFor<T>,
			to: T::AccountId,
			asset_service_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let transfer_asset_service_sender = sender.clone();

			let owner = Self::owner_of_assetservice(asset_service_id)
				.ok_or("No owner for this asset service")?;

			ensure!(owner == sender, "Not the owner of the particular asset service");
			let asset_service_index = Self::all_assets_services_index(asset_service_id);
			//Increment by 1
			let newindex = asset_service_index.checked_add(1).unwrap();
			<OutstandingAssetServiceIndex<T>>::put(newindex);
			let asset_service_finalise_id = asset_service_id;
			let new_asset_service_owner = to;

			let mut asset = Self::assetservices(asset_service_finalise_id).unwrap();
                    //let no_proofs_asset =
                    //Self::get_assetserviceproofcounter(asset_service_finalise_id);
                    if (Self::get_assetserviceprooffinalcounter(asset_service_finalise_id) >= 1) {
                        let (ServiceInterestCounter, TotalServiceInterestprice) =
                            Self::get_serviceinterestcounter((
                                asset_service_finalise_id,
                                new_asset_service_owner.clone(),
                            ));
                        //https://stackoverflow.com/questions/56081117/how-do-you-convert-between-substrate-specific-types-and-rust-primitive-types
                        let assetprice_u64 = TryInto::<u64>::try_into(asset.price).ok().unwrap();
                        // Get the fidelity amount by dividing the asset price by the total vehicles
                        let division_amount = assetprice_u64 / ServiceInterestCounter;
                        let asset_oldowner =
                            Self::owner_of_assetservice(asset_service_finalise_id).unwrap();
                        let fidelity_amount_balance = division_amount.try_into().ok().unwrap();
                        // Get the reamining asset price: total asset price - fidlity amount
                        let assetprice_afterfidelitydecrement = assetprice_u64
                            .checked_sub(division_amount)
                            .unwrap()
                            .try_into()
                            .ok()
                            .unwrap();
                        //let mut meta = Self::meta_data();
                        // Transfer from Escrow to the old asset owner
                        let escrow_account = Self::escrowaccount(0);
                        T::Currency::transfer(
                            &escrow_account,
                            &asset_oldowner,
                            assetprice_afterfidelitydecrement,
                            ExistenceRequirement::KeepAlive,
                        );
                        T::Currency::transfer(
                            &escrow_account,
                            &new_asset_service_owner,
                            fidelity_amount_balance,
                            ExistenceRequirement::KeepAlive,
                        );
                        log::info!("Asset Service ");
                        // Transfer ownership of asset service from Radar OEM to Vehicle OEM
                        Self::transfer_asset_service_from(
                            asset_oldowner.clone(),
                            new_asset_service_owner.clone(),
                            asset_service_finalise_id,
                        )
                        .expect("asset transferred from owner to sender");
                        //TODO: SEND ASSET SERVICE CRITERIA &&
                        // Should Expect HASH of JWT and store it
                        Self::deposit_event(Event::BoughtAssetService(
                            asset_oldowner,
                            new_asset_service_owner,
                            asset_service_finalise_id,
                            asset.price,
                        ));
                    }

			Ok(().into())
		}
		////////////////////////////////////////////////////////////////////////////////////
		///
		// Bid for An Asset
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn bidAssetService(
			origin: OriginFor<T>,
			to: T::AccountId,
			asset_id: T::Hash,
			ask_price: BalanceOf<T>,
			encrypted_price: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let bid_asset_service_sender = sender.clone();

			//let owner = Self::owner_of(asset_id).ok_or("No owner for this asset")?;

			//ensure!(owner == sender, "Not the owner of the particular asset");

			let firstAssetBidCounter = Self::get_firstassetservicebidcounter(asset_id);
			let new_first_asset_bid_counter =
				firstAssetBidCounter.checked_add(1).ok_or("Incremented the Final Asset")?;
			<FirstAssetServiceBidCounter<T>>::insert(asset_id, new_first_asset_bid_counter);
			// Put the transferred asset along with the sender
			<FirstAssetServiceBid<T>>::insert(
				(asset_id, firstAssetBidCounter),
				(&sender, ask_price, encrypted_price),
			);

			Ok(().into())
		}
		////////////////////////////////////////////////////////////////////////:
		///
		// Buy an asset
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn accept_bid_service(
			origin: OriginFor<T>,
			asset_id: T::Hash,
			bid_threshold_price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let accept_bid_service_sender = sender.clone();
			ensure!(<AssetServices<T>>::contains_key(asset_id), "This asset does not exist");

			let owner = Self::owner_of_assetservice(asset_id).ok_or("No owner for this asset")?;

			//Check that account buying the asset doesnt already own the asset
			ensure!(owner == sender, "You are not the owner of the asset");
			// Get the existing quoted price of the asset by the Radar-manufacturer or OEM
			//Retrieve the counter
			//Retrive the bids of the asset
			let FirstAssetBidCounter = Self::get_firstassetservicebidcounter(asset_id);
			let new_first_asset_bid_counter =
				FirstAssetBidCounter.checked_add(1).ok_or("Incremented the Final Asset")?;
			<FirstAssetServiceBidCounter<T>>::insert(asset_id, new_first_asset_bid_counter);
			for j in 0..FirstAssetBidCounter {
				let ((Bidder, Bid_Amount,Encrypted_Bid)) = Self::get_firstassetservicebid((asset_id, j));
				if (Bid_Amount >= bid_threshold_price) {
					let mut asset = Self::assetservices(asset_id).unwrap();

					let asset_price = asset.price;

					let asset_criteria = asset.assetservice_criteria;

					// Check if the asset is for sale
					ensure!(!asset_price.is_zero(), "This asset is not for sale!");
					// TO DO
					// Updating balances by currency trait
					// Transfer To Escrow Account
					//let mut meta = Self::meta_data();
					let escrow_account = Self::escrowaccount(0);
					let asset_owner =
						Self::owner_of_assetservice(asset_id).ok_or("No owner for this asset")?;

					let (ServiceInterestCounter, TotalPrice) =
						Self::get_serviceinterestcounter((asset_id, Bidder.clone()));
					//https://stackoverflow.com/questions/56081117/how-do-you-convert-between-substrate-specific-types-and-rust-primitive-types
					let assetprice_u64 = TryInto::<u64>::try_into(asset_price).ok().unwrap();

					// Get the fidelity amount by dividing the asset price by the total vehicles
					let division_amount = Bid_Amount / ServiceInterestCounter.saturated_into();
					for j in 0..ServiceInterestCounter {
						let (vehicle_account, price) =
							Self::get_serviceinterest((asset_id, Bidder.clone(), j));
						T::Currency::transfer(
							&sender,
							&escrow_account,
							division_amount,
							ExistenceRequirement::KeepAlive,
						)?;
					}
					//Set the price of the asset to the new price it was sold
					asset.price = Bid_Amount.into();
					<AssetServices<T>>::insert(asset_id, asset);
					//Self::deposit_event(Event::BoughtAsset(sender, owner, asset_id, asset_price));
					// Increment the asset counter
					let final_asset_counter = Self::get_finalisedassetservicecounter();
					// Put the transferred asset along with the sender
					<FinalisedAssetServiceId<T>>::insert(
						final_asset_counter,
						(asset_id, Bidder.clone()),
					);
					let new_final_asset_counter =
						final_asset_counter.checked_add(1).ok_or("Incremented the Final Asset")?;
					<FinalisedAssetServiceCounter<T>>::put(new_final_asset_counter);

					Self::deposit_event(Event::PrepareAssetService(
						asset_owner,
						Bidder.clone(),
						asset_id,
						Bid_Amount,
						asset_criteria,
					));
				}
			}
			Ok(().into())
		}

		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn accept_bid_service_privacy(
			origin: OriginFor<T>,
			asset_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let accept_bid_service_sender = sender.clone();
			ensure!(<AssetServices<T>>::contains_key(asset_id), "This asset does not exist");

			let owner = Self::owner_of_assetservice(asset_id).ok_or("No owner for this asset")?;

			//Check that account buying the asset doesnt already own the asset
			ensure!(owner == sender, "You are not the owner of the asset");
			// Get the existing quoted price of the asset by the Radar-manufacturer or OEM
			//Retrieve the counter
			//Retrive the bids of the asset
			let FirstAssetBidCounter = Self::get_firstassetservicebidcounter(asset_id);
			let new_first_asset_bid_counter =
				FirstAssetBidCounter.checked_add(1).ok_or("Incremented the Final Asset")?;
			<FirstAssetServiceBidCounter<T>>::insert(asset_id, new_first_asset_bid_counter);
			for j in 0..FirstAssetBidCounter {
				let ((Bidder, Bid_Amount,Encrypted_BidAmount)) = Self::get_firstassetservicebid((asset_id, j));
				let mut asset = Self::assetservices(asset_id).unwrap();

				let asset_price = asset.price;

				let asset_criteria = asset.assetservice_criteria;

				// Check if the asset is for sale
				ensure!(!asset_price.is_zero(), "This asset is not for sale!");
				// TO DO
				// Updating balances by currency trait
				// Transfer To Escrow Account
				//let mut meta = Self::meta_data();
				let escrow_account = Self::escrowaccount(0);
				let asset_owner =
					Self::owner_of_assetservice(asset_id).ok_or("No owner for this asset")?;

				let (ServiceInterestCounter, TotalPrice) =
					Self::get_serviceinterestcounter((asset_id, Bidder.clone()));
				//https://stackoverflow.com/questions/56081117/how-do-you-convert-between-substrate-specific-types-and-rust-primitive-types
				let assetprice_u64 = TryInto::<u64>::try_into(asset_price).ok().unwrap();

				// Get the fidelity amount by dividing the asset price by the total vehicles
				let division_amount = Bid_Amount / ServiceInterestCounter.saturated_into();
				for j in 0..ServiceInterestCounter {
					let (vehicle_account, price) =
						Self::get_serviceinterest((asset_id, Bidder.clone(), j));
					T::Currency::transfer(
						&sender,
						&escrow_account,
						division_amount,
						ExistenceRequirement::KeepAlive,
					)?;
				}
				//Set the price of the asset to the new price it was sold
				asset.price = Bid_Amount.into();
				<AssetServices<T>>::insert(asset_id, asset);
				//Self::deposit_event(Event::BoughtAsset(sender, owner, asset_id, asset_price));
				// Increment the asset counter
				let final_asset_counter = Self::get_finalisedassetservicecounter();
				// Put the transferred asset along with the sender
				<FinalisedAssetServiceId<T>>::insert(
					final_asset_counter,
					(asset_id, Bidder.clone()),
				);
				let new_final_asset_counter =
					final_asset_counter.checked_add(1).ok_or("Incremented the Final Asset")?;
				<FinalisedAssetServiceCounter<T>>::put(new_final_asset_counter);

				Self::deposit_event(Event::PrepareAssetService(
					asset_owner,
					Bidder.clone(),
					asset_id,
					Bid_Amount,
					asset_criteria,
				));
			}
			Ok(().into())
		}

		/////////////////////////////////////////////////////////////////
		// Buy an asset
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn buy_asset_service(
			origin: OriginFor<T>,
			asset_service_id: T::Hash,
			ask_price: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			ensure!(
				<AssetServices<T>>::contains_key(asset_service_id),
				"This asset service does not exist"
			);
			let owner = Self::owner_of_assetservice(asset_service_id)
				.ok_or("No owner for this asset service")?;
			// Only
			ensure!(
				<AssetAdmin<T>>::contains_key(sender.clone()),
				"Asset Admin can only buy the data"
			);
			//Check that account buying the asset doesnt already own the asset
			ensure!(owner != sender.clone(), "You are the owner of the asset service already");
			// Get the existing quoted price of the asset by the Radar-manufacturer or OEM
			let mut assetservice = Self::assetservices(asset_service_id).unwrap();
			let asset_service_threshold_price = assetservice.priceThreshold;

			let asset_criteria = assetservice.assetservice_criteria;

			let asset_service_price = assetservice.price;
			// Check if the asset is for sale
			ensure!(!asset_service_price.is_zero(), "This asset is not for sale!");
			// TO DO
			ensure!(
				ask_price >= asset_service_threshold_price,
				"Price is not within threshold Too Low, Consider Bidding"
			);

			// Updating balances by currency trait
			// Transfer To Escrow Account
			//let mut meta = Self::meta_data();
			let escrow_account = Self::escrowaccount(0);

			let (ServiceInterestCounter, totalprice) =
				Self::get_serviceinterestcounter((asset_service_id, sender.clone()));
			//https://stackoverflow.com/questions/56081117/how-do-you-convert-between-substrate-specific-types-and-rust-primitive-types
			let assetprice_u64 = TryInto::<u64>::try_into(asset_service_price).ok().unwrap();
			// Get the fidelity amount by dividing the asset price by the total vehicles
			let division_amount = assetprice_u64 / ServiceInterestCounter;
			let iter_servicecounter = ServiceInterestCounter.checked_add(1).unwrap();
			for j in 1..iter_servicecounter {
				let ((vehicle_account, price)) =
					Self::get_serviceinterest((asset_service_id, sender.clone(), j));
				T::Currency::transfer(
					&vehicle_account,
					&escrow_account,
					division_amount.saturated_into(),
					ExistenceRequirement::KeepAlive,
				)?;
			}
			//Set the price of the asset to the new price it was sold
			assetservice.price = ask_price.into();
			<AssetServices<T>>::insert(asset_service_id, assetservice);

			let final_asset_counter = Self::get_finalisedassetservicecounter();
			// Put the transferred asset along with the sender
			<FinalisedAssetServiceId<T>>::insert(final_asset_counter, (asset_service_id, &sender));
			let new_final_asset_counter =
				final_asset_counter.checked_add(1).ok_or("Incremented the Final Asset")?;
			<FinalisedAssetServiceCounter<T>>::put(new_final_asset_counter);

			Self::deposit_event(Event::PrepareAssetService(
				sender,
				owner,
				asset_service_id,
				asset_service_price,
				asset_criteria,
			));
			Ok(().into())
		}

		//////////////////////////////////////////////////////////////////////////////////////////////////////////
		///
		// Buy an asset
		/* 		#[pallet::weight(100)]
		pub fn submit_asset_service_proof(
			origin: OriginFor<T>,
			asset_id: T::Hash,
			proof: T::Hash,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(<AssetServices<T>>::contains_key(asset_id), "This asset does not exist");
			let asset_proof_counter = Self::get_assetserviceproofcounter(asset_id);
			let new_asset_proof_counter =
				asset_proof_counter.checked_add(1).ok_or("Incremented the Proof Counter")?;
			<AssetServiceProofCounter<T>>::insert(asset_id, new_asset_proof_counter);

			//Insert the actual proof
			<AssetServiceProof<T>>::insert((asset_id, new_asset_proof_counter), (sender, proof));
			//<AssetProof<T>>::decode_len(asset_id);

			//ensure!(!(<AssetProof<T>>::contains_key(asset_id)::contains_key(sender)), "This asset does not exist");

			//<AssetProof<T>>::insert(asset_id, (sender, proof));

			Ok(().into())
		} */
		//////////////////////////////////////////////////////////////////////////////////////////////////////////////
		//Finalise an asset Service
		// Transfer an asset between Radar Manuacturer and OEM
		#[pallet::weight((0, DispatchClass::Normal, Pays::No))]
		pub fn transferAsset(
			origin: OriginFor<T>,
			to: T::AccountId,
			asset_id: T::Hash,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let owner = Self::owner_of(asset_id).ok_or("No owner for this asset")?;

			ensure!(owner == sender, "Not the owner of the particular asset");
			let asset_index = Self::all_assets_index(asset_id);
			//Increment by 1
			let newindex = asset_index.checked_add(1).unwrap();
			<OutstandingAssetIndex<T>>::put(newindex);
			let asset_finalise_id = asset_id;
			let new_asset_owner = to;
			let asset_index = Self::all_assets_index(asset_id);
			//Increment by 1
			let newindex = asset_index.checked_add(1).unwrap();
			<OutstandingAssetIndex<T>>::put(newindex);
			let asset_finalise_id = asset_id;
			let new_asset_owner = to;
			
			let mut asset = Self::assets(asset_finalise_id).unwrap();
                    let no_proofs_asset = Self::get_assetproofcounter(asset_finalise_id);

                    if (no_proofs_asset >= 1
                        && Self::get_assetprooffinalcounter(asset_finalise_id) >= 1)
                    {
                        //https://stackoverflow.com/questions/56081117/how-do-you-convert-between-substrate-specific-types-and-rust-primitive-types
                        let assetprice_u64 = TryInto::<u64>::try_into(asset.price).ok().unwrap();
                        // Get the fidelity amount by dividing the asset price by the total vehicles
                        let fidelity_amount =
                            assetprice_u64 / no_proofs_asset.checked_add(1).unwrap();
                        let asset_oldowner = Self::owner_of(asset_finalise_id).unwrap();
                        let fidelity_amount_balance = fidelity_amount.try_into().ok().unwrap();
                        //let mut meta = Self::meta_data();
                        let escrow_account = Self::escrowaccount(0);
                        let mut uniquevehicles = BTreeSet::new();
                        for j in 1..no_proofs_asset {
                            let ((vehicle_id, hashproof)) =
                                Self::get_assetprooffinal((asset_finalise_id, j));
                            //Validation check
                            if (!uniquevehicles.contains(&vehicle_id)) {
                                uniquevehicles.insert(vehicle_id.clone());
                                // Updating balances by currency trait
                                T::Currency::transfer(
                                    &escrow_account,
                                    &vehicle_id,
                                    fidelity_amount_balance,
                                    ExistenceRequirement::KeepAlive,
                                );
                            } else {
                                log::info!("Duplicate Asset Submission");
                            }
                        }
                        T::Currency::transfer(
                            &escrow_account,
                            &asset_oldowner,
                            fidelity_amount_balance,
                            ExistenceRequirement::KeepAlive,
                        );
                        // Transfer ownership of asset
                        Self::transfer_asset_from(
                            asset_oldowner.clone(),
                            new_asset_owner.clone(),
                            asset_finalise_id,
                        )
                        .expect("asset transferred from owner to sender");
                        //TODO: SEND CRITERIA
                        // Should Expect HASH of JWT and store it
                        
                        Self::deposit_event(Event::BoughtAsset(
                            asset_oldowner,
                            new_asset_owner,
                            asset_finalise_id,
                            asset.price,
                        ));

			Ok(().into())
		}
	}
	impl<T: Config> Pallet<T> {
		fn increment_nonce() -> DispatchResult {
			<Nonce<T>>::try_mutate(|nonce| {
				let next = nonce.checked_add(1).ok_or(Error::<T>::NonceOverflow)?;
				*nonce = next;
				Ok(().into())
			})
		}

		fn increment_service_nonce() -> DispatchResult {
			<AssetServiceNonce<T>>::try_mutate(|nonce| {
				let next = nonce.checked_add(1).ok_or(Error::<T>::NonceOverflow)?;
				*nonce = next;
				Ok(().into())
			})
		}

		fn random_hash(sender: &T::AccountId) -> T::Hash {
			let nonce = <Nonce<T>>::get();
			let seed = T::AssetRandomness::random_seed();

			T::Hashing::hash_of(&(seed, &sender, nonce))
		}

		fn random_service_hash(sender: &T::AccountId) -> T::Hash {
			let nonce = <AssetServiceNonce<T>>::get();
			let seed = T::AssetServiceRandomness::random_seed();

			T::Hashing::hash_of(&(seed, &sender, nonce))
		}
		// Mint an asset
		fn mintasset(to: T::AccountId, asset_id: T::Hash, new_asset: Asset<T>) -> DispatchResult {
			ensure!(!<AssetOwner<T>>::contains_key(asset_id), " Asset Already contains key");

			// Update total asset counts
			let owned_asset_count = Self::owned_asset_count(&to);
			let new_owned_asset_count =
				owned_asset_count.checked_add(1).ok_or("Overflow added a new asset")?;

			let all_asset_count = Self::all_asset_count();
			let new_all_asset_count =
				all_asset_count.checked_add(1).ok_or("Overflow added a new asset to total")?;

			// Update Storage with new asset

			<Assets<T>>::insert(asset_id, new_asset);
			<AssetOwner<T>>::insert(asset_id, Some(&to));

			// Write Asset counting information to storage
			<AllAssetsArray<T>>::insert(all_asset_count, asset_id);
			<AllAssetsCount<T>>::put(new_all_asset_count);
			<AllAssetsIndex<T>>::insert(asset_id, all_asset_count);

			//Write Asset information
			<OwnedAssetsArray<T>>::insert((to.clone(), all_asset_count), asset_id);
			<OwnedAssetsCount<T>>::insert(&to, new_owned_asset_count);
			<OwnedAssetsIndex<T>>::insert(asset_id, all_asset_count);

			//Deposit our "Created event"
			Self::deposit_event(Event::CreatedAsset(to, asset_id));

			Ok(())
		}

		// Mint an asset service
		fn mintassetservice(
			to: T::AccountId,
			asset_service_id: T::Hash,
			new_asset_service: AssetService<T>,
		) -> DispatchResult {
			ensure!(
				!<AssetServiceOwner<T>>::contains_key(asset_service_id),
				" Asset Service Already contains key"
			);

			// Increment asset index count for a particular account
			let owned_asset_service_count = Self::owned_asset_service_count(&to);
			let new_owned_asset_service_count = owned_asset_service_count
				.checked_add(1)
				.ok_or("Overflow added a new asset service")?;
			// Update Asset Service Count
			let all_asset_service_count = Self::all_asset_service_count();
			let new_all_asset_service_count = all_asset_service_count
				.checked_add(1)
				.ok_or("Overflow added a new asset to total")?;

			// Update Storage with new asset

			<AssetServices<T>>::insert(asset_service_id, new_asset_service);
			<AssetServiceOwner<T>>::insert(asset_service_id, Some(&to));

			// Write Asset counting information to storage

			<AllAssetServicesArray<T>>::insert(all_asset_service_count, asset_service_id);
			<AllAssetServicesCount<T>>::put(new_all_asset_service_count);
			<AllAssetServicesIndex<T>>::insert(asset_service_id, all_asset_service_count);

			//Write Asset information
			<OwnedAssetServicesArray<T>>::insert(
				(to.clone(), all_asset_service_count),
				asset_service_id,
			);
			<OwnedAssetServicesCount<T>>::insert(&to, new_owned_asset_service_count);
			<OwnedAssetServicesIndex<T>>::insert(asset_service_id, all_asset_service_count);

			//Deposit our "Created event"
			Self::deposit_event(Event::CreatedAssetService(to, asset_service_id));

			Ok(())
		}

		// Transfer Asset from one account to Another
		fn transfer_asset_from(
			from: T::AccountId,
			to: T::AccountId,
			asset_id: T::Hash,
		) -> DispatchResult {
			// Verify owner if rightful owner of this asset
			let owner = Self::owner_of(asset_id).ok_or("No owner of this asset")?;

			ensure!(owner == from, "You are not owner of this asset");

			// Address to send from
			let owned_asset_count_from = Self::owned_asset_count(&from);

			// Address to send to
			let owned_asset_count_to = Self::owned_asset_count(&to);

			// Increment the amount of owned assets by 1
			let new_owned_asset_count_to =
				owned_asset_count_to.checked_add(1).ok_or("Overflow error of asset balance ")?;

			//Decrement the amount of owned assets by 1
			let new_owned_asset_count_from = owned_asset_count_from
				.checked_sub(1)
				.ok_or("Underflow error of asset balance ")?;

			// Get current asset index
			let asset_index = <OwnedAssetsIndex<T>>::get(asset_id);

			//Update storage items that required updated asset
			if asset_index != new_owned_asset_count_from {
				let last_asset_id =
					<OwnedAssetsArray<T>>::get((from.clone(), new_owned_asset_count_from));
				<OwnedAssetsArray<T>>::insert((from.clone(), asset_index), last_asset_id);
				<OwnedAssetsIndex<T>>::insert(last_asset_id, asset_index);
			}

			// Write new asset ownership to storage items

			<AssetOwner<T>>::insert(&asset_id, Some(&to));
			<OwnedAssetsIndex<T>>::insert(asset_id, owned_asset_count_to);

			// Remove the asset from its owner
			<OwnedAssetsArray<T>>::remove((from.clone(), new_owned_asset_count_from));
			// Add the asset to the recipient
			<OwnedAssetsArray<T>>::insert((to.clone(), owned_asset_count_to), asset_id);
			// Update the Asset Count
			<OwnedAssetsCount<T>>::insert(&from, new_owned_asset_count_from);
			<OwnedAssetsCount<T>>::insert(&to, new_owned_asset_count_to);
			// Emit an event for Transfer
			Self::deposit_event(Event::TransferredAsset(from, to, asset_id));

			Ok(())
		}

		// Transfer Asset from one account to Another
		fn transfer_asset_service_from(
			from: T::AccountId,
			to: T::AccountId,
			asset_service_id: T::Hash,
		) -> DispatchResult {
			// Verify owner if rightful owner of this asset
			let owner = Self::owner_of_assetservice(asset_service_id)
				.ok_or("No owner of this asset service")?;

			ensure!(owner == from, "You are not owner of this asset");

			// Address to send from
			let owned_asset_service_count_from = Self::owned_asset_service_count(&from);

			// Address to send to
			let owned_asset_service_count_to = Self::owned_asset_service_count(&to);

			// Increment the amount of owned assets by 1
			let new_owned_asset_service_count_to = owned_asset_service_count_to
				.checked_add(1)
				.ok_or("Overflow error of asset service balance ")?;

			//Decrement the amount of owned assets by 1
			let new_owned_asset_service_count_from = owned_asset_service_count_from
				.checked_sub(1)
				.ok_or("Underflow error of asset balance ")?;

			// Get current asset index
			let asset_service_index = <OwnedAssetServicesIndex<T>>::get(asset_service_id);

			//Update storage items that required updated asset
			if asset_service_index != new_owned_asset_service_count_from {
				let last_asset_service_id = <OwnedAssetServicesArray<T>>::get((
					from.clone(),
					new_owned_asset_service_count_from,
				));
				<OwnedAssetServicesArray<T>>::insert(
					(from.clone(), asset_service_index),
					last_asset_service_id,
				);
				<OwnedAssetsIndex<T>>::insert(last_asset_service_id, asset_service_index);
			}

			// Write new asset ownership to storage items

			<AssetOwner<T>>::insert(&asset_service_id, Some(&to));
			<OwnedAssetsIndex<T>>::insert(asset_service_id, owned_asset_service_count_to);

			// Remove the asset from its owner
			<OwnedAssetServicesArray<T>>::remove((
				from.clone(),
				new_owned_asset_service_count_from,
			));
			// Add the asset to the recipient
			<OwnedAssetServicesArray<T>>::insert(
				(to.clone(), owned_asset_service_count_to),
				asset_service_id,
			);
			// Update the Asset Count
			<OwnedAssetServicesCount<T>>::insert(&from, new_owned_asset_service_count_from);
			<OwnedAssetServicesCount<T>>::insert(&to, new_owned_asset_service_count_to);
			// Emit an event for Transfer
			Self::deposit_event(Event::TransferredAssetService(from, to, asset_service_id));

			Ok(())
		}

		//https://docs.rs/sp-runtime/2.0.0-rc4/src/sp_runtime/offchain/http.rs.html#198-201
		pub fn fetch_from_remote_post_asset() -> Result<Vec<u8>> {
			// Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
			let pending = rt_offchain::http::Request::default()
				.method(rt_offchain::http::Method::Post)
				.url(HTTP_REMOTE_REQUEST_ASSET)
				.body(vec![b"username=vehicle_oem&password=leat_vehicle"])
				.add_header("Content-Type", "application/x-www-form-urlencoded")
				.send()
				.unwrap();
			let mut response = pending.wait().unwrap();
			//let mut headers = response.headers().into_iter();
			//let body = response.body();
			let body = response.body().collect::<Vec<u8>>();

			// Create a str slice from the body.
			let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
				log::warn!("No UTF8 body");
				rt_offchain::http::Error::Unknown
			});
			let body_stri = sp_std::str::from_utf8(&body).unwrap();
			/* let token_resp = match Self::parse_token(body_stri) {
				Some(token_resp) => Ok(token_resp),
				None => {
					log::warn!("Unable to extract price from the response: {:?}", body_stri);
					Err(rt_offchain::http::Error::Unknown)
				}
			}; */
			let v: Value = serde_json::from_str(body_stri)?;
			log::info!("Current block: {} )", v["jwt"]);

			//Ok(v["jwt"].as_str().unwrap().to_string().as_bytes().to_vec())
			Ok(serde_json::to_vec(&v["jwt"]).unwrap())
			// Next we fully read the response body and collect it to a vector of bytes.
		}

		pub fn fetch_from_remote_post_asset_service() -> Result<Vec<u8>> {
			// Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
			let pending = rt_offchain::http::Request::default()
				.method(rt_offchain::http::Method::Post)
				.url(HTTP_REMOTE_REQUEST_ASSET_SERVICE)
				.body(vec![b"username=radar_oem&password=leat_radar"])
				.add_header("Content-Type", "application/x-www-form-urlencoded")
				.send()
				.unwrap();
			let mut response = pending.wait().unwrap();
			//let mut headers = response.headers().into_iter();
			//let body = response.body();
			let body = response.body().collect::<Vec<u8>>();

			// Create a str slice from the body.
			let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
				log::warn!("No UTF8 body");
				rt_offchain::http::Error::Unknown
			});
			let body_stri = sp_std::str::from_utf8(&body).unwrap();
			/* let token_resp = match Self::parse_token(body_stri) {
				Some(token_resp) => Ok(token_resp),
				None => {
					log::warn!("Unable to extract price from the response: {:?}", body_stri);
					Err(rt_offchain::http::Error::Unknown)
				}
			}; */
			let v: Value = serde_json::from_str(body_stri)?;
			log::info!("Current block: {} )", v["jwt"]);

			//Ok(v["jwt"].as_str().unwrap().to_string().as_bytes().to_vec())
			Ok(serde_json::to_vec(&v["jwt"]).unwrap())
			// Next we fully read the response body and collect it to a vector of bytes.
		}

		pub fn validate_proof_asset(asset_id: T::Hash) -> Result<BTreeSet<T::AccountId>> {
			// Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
			let no_proofs_asset = Self::get_assetproofcounter(asset_id);
            let mut invalidvehicles = BTreeSet::<T::AccountId>::new();
			let mut validation = true;
			for i in 1..no_proofs_asset+1 {
				let (vehicle_id,hash_proof) = Self::get_assetproof((asset_id,i));

			let pending = rt_offchain::http::Request::default()
				.method(rt_offchain::http::Method::Post)
				.url(HTTP_REMOTE_REQUEST_ASSET_VALID)
				.body(vec![b"username=radar_oem&password=leat_radar"])
				.add_header("Content-Type", "application/x-www-form-urlencoded")
				.send()
				.unwrap();
			let mut response = pending.wait().unwrap();
			//let mut headers = response.headers().into_iter();
			//let body = response.body();
			let body = response.body().collect::<Vec<u8>>();

			// Create a str slice from the body.
			let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
				log::warn!("No UTF8 body");
				rt_offchain::http::Error::Unknown
			});
			let body_stri = sp_std::str::from_utf8(&body).unwrap();
			/* let token_resp = match Self::parse_token(body_stri) {
				Some(token_resp) => Ok(token_resp),
				None => {
					log::warn!("Unable to extract price from the response: {:?}", body_stri);
					Err(rt_offchain::http::Error::Unknown)
				}
			}; */


			let vehicledata: Value = serde_json::from_str(body_stri)?;
			let calculated_hash = hashing::sha2_256(vehicledata["jwt"].as_str().unwrap().as_bytes());
			if(calculated_hash != hash_proof.encode().as_slice()){
				invalidvehicles.insert(vehicle_id);
			}
			log::info!("Current block: {} )", vehicledata["jwt"]);

			}
          
			// Get the data
			//Calculate Hash
			// Verify the hash of the blockchain


			//Ok(v["jwt"].as_str().unwrap().to_string().as_bytes().to_vec())
			Ok((invalidvehicles))
			// Next we fully read the response body and collect it to a vector of bytes.
		}
	}
}
