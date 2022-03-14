#[frame_support::pallet]
pub mod pallet {
	// ----------------------------------------------------------------------------------------------------
	//                                       Imports and Dependencies                                      
	// ----------------------------------------------------------------------------------------------------

	// ----------------------------------------------------------------------------------------------------
	//                                    Declaration Of The Pallet Type                                           
	// ----------------------------------------------------------------------------------------------------

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// ----------------------------------------------------------------------------------------------------
	//                                             Config Trait                                            
	// ----------------------------------------------------------------------------------------------------

	// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// ----------------------------------------------------------------------------------------------------
    //                                             Pallet Types                                           
	// ----------------------------------------------------------------------------------------------------

	// ----------------------------------------------------------------------------------------------------
    //                                            Runtime Events                                          
	// ----------------------------------------------------------------------------------------------------

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	}

	// ----------------------------------------------------------------------------------------------------
    //                                           Runtime  Errors                                           
	// ----------------------------------------------------------------------------------------------------

	#[pallet::error]
	pub enum Error<T> {
	}

	// ----------------------------------------------------------------------------------------------------
    //                                           Runtime  Storage                                          
	// ----------------------------------------------------------------------------------------------------

	// ----------------------------------------------------------------------------------------------------
    //                                                Hooks                                                
	// ----------------------------------------------------------------------------------------------------

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	// ----------------------------------------------------------------------------------------------------
    //                                              Extrinsics                                             
	// ----------------------------------------------------------------------------------------------------

	#[pallet::call]
	impl<T: Config> Pallet<T> {
	}
}

// ----------------------------------------------------------------------------------------------------
//                                              Extrinsics                                             
// ----------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
}
