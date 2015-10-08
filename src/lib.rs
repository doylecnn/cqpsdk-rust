pub mod cqpapi;

#[macro_export]
macro_rules! gbk {

	( $x: expr ) => (CString::new(GBK.encode($x, EncoderTrap::Ignore).unwrap()).unwrap().as_ptr());
	
}

#[macro_export]
macro_rules! utf8 {
	
	( $x: expr ) => (&GBK.decode(CStr::from_ptr($x).to_bytes(), DecoderTrap::Ignore).unwrap()[..]);
	
}

