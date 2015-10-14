pub mod cqpapi;

/// Converts `UTF-8` str to `GBK` *const i8.
///
/// Check `CQ_sendPrivateMsg` for examples.
///
#[macro_export]
macro_rules! gbk {

	( $x: expr ) => (CString::new(GBK.encode($x, EncoderTrap::Ignore).unwrap()).unwrap().as_ptr());
	
}

/// Converts `GBK` *const i8 to `UTF-8` str.
///
/// An opposite macro against `gbk!`.
///
#[macro_export]
macro_rules! utf8 {
	
	( $x: expr ) => (&GBK.decode(CStr::from_ptr($x).to_bytes(), DecoderTrap::Ignore).unwrap()[..]);
	
}

