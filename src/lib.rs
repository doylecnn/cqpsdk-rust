#![feature(use_extern_macros)]

use std::ffi::{CString, CStr};

extern crate encoding;
use encoding::{Encoding, EncoderTrap, DecoderTrap};
use encoding::all::GB18030;

/// Converts `UTF-8` str to `GB18030` *const i8.
///
/// Check `CQ_sendPrivateMsg` for examples.
///
#[macro_export]
macro_rules! UTF8_STR_TO_GB18030_C_CHAR_PTR {
    ( $x: expr ) => (CString::new(GB18030.encode($x, EncoderTrap::Ignore).unwrap()).unwrap().into_raw());
}

/// Converts `GB18030` *const i8 to `UTF-8` str.
///
/// An opposite macro against `UTF8_STR_TO_GB18030_C_CHAR_PTR!`.
///
#[macro_export]
macro_rules! GB18030_C_CHAR_PRT_TO_UTF8_STR {   
    ( $x: expr ) => (&GB18030.decode(CStr::from_ptr($x).to_bytes(), DecoderTrap::Ignore).unwrap()[..]);
}

pub mod cqpapi;

pub enum LogLevel{
    Debug       = 0,
    Info        = 10,
    InfoSuccess = 11,
    InfoRecv    = 12,
    InfoSend    = 13,
    Warning     = 20,
    Error       = 30,
    Fatal       = 40,
}

impl LogLevel{
    fn to_int(self)->i32{
        match self {
            LogLevel::Debug       => 0,
            LogLevel::Info        => 10,
            LogLevel::InfoSuccess => 11,
            LogLevel::InfoRecv    => 12,
            LogLevel::InfoSend    => 13,
            LogLevel::Warning     => 20,
            LogLevel::Error       => 30,
            LogLevel::Fatal       => 40,
        }
    }
}

pub struct Client{
    auth_code :i32,
    initialized :bool,
    app_name :String,
    api_version :i32
}

impl Client{
    /// Create new Client
    ///
    /// # Examples
    ///
    /// ```
    /// let mut c = cqpsdk::Client::new("app_name");
    /// ```
    ///
    pub fn new<S: Into<String>>(app_name:S) -> Client{
        Client{auth_code:0, initialized:false, app_name:app_name.into(), api_version:cqpapi::API_VERSION}
    }

    /// Initialize
    /// 
    /// # Examples
    ///
    /// ```
    /// let mut c = cqpsdk::Client::new("app_name");
    /// c.initialize(131);
    /// ```
    ///
    pub fn initialize(&mut self, auth_code: i32){
        self.auth_code = auth_code;
        self.initialized = true;
    }

    /// Return appinfo
    /// 
    /// # Examples
    ///
    /// ```
    /// let mut c = cqpsdk::Client::new("app_name");
    /// c.initialize(131);
    /// println!("{}", c.app_info());
    /// ```
    ///
    pub fn app_info(&self) -> String{
        self.api_version.to_string() + "," + &self.app_name
    }

    /// Return login qq no
    /// 
    /// # Examples
    ///
    /// ```
    /// let mut c = cqpsdk::Client::new("app_name");
    /// c.initialize(131);
    /// println!("{}", c.get_login_qq());
    /// ```
    ///
    pub fn get_login_qq(&self)->i64{
        unsafe{
            cqpapi::CQ_getLoginQQ(self.auth_code)
        }
    }

    pub fn send_private_message(&self, qq_number: i64, msg: &str)->i32{
        let msg = UTF8_STR_TO_GB18030_C_CHAR_PTR!(msg);
        unsafe{
            cqpapi::CQ_sendPrivateMsg(self.auth_code, qq_number, msg)
        }
    }

    pub fn add_log(&self, priority: LogLevel, t: &str, msg: &str)->i32{
        let t = UTF8_STR_TO_GB18030_C_CHAR_PTR!(t);
        let msg = UTF8_STR_TO_GB18030_C_CHAR_PTR!(msg);
        unsafe{
            cqpapi::CQ_addLog(self.auth_code, priority.to_int(), t, msg)
        }
    }

    pub fn send_group_msg(&self, group_number: i64, msg: &str) -> i32{
        let msg = UTF8_STR_TO_GB18030_C_CHAR_PTR!(msg);
        unsafe{
            cqpapi::CQ_sendGroupMsg(self.auth_code, group_number, msg)
        }
    }

    pub fn send_discussion_msg(&self, discussion_number: i64, msg: &str) -> i32{
        let msg = UTF8_STR_TO_GB18030_C_CHAR_PTR!(msg);
        unsafe{
            cqpapi::CQ_sendDiscussMsg(self.auth_code, discussion_number, msg)
        }
    }

    pub fn send_like(&self, qq_number: i64) -> i32{
        unsafe{
            cqpapi::CQ_sendLike(self.auth_code, qq_number)
        }
    }

    pub fn set_group_kick(&self, group_number: i64, qq_number: i64, refuse_rejoin: i32) -> i32{
        unsafe{
            cqpapi::CQ_setGroupKick(self.auth_code, group_number, qq_number, refuse_rejoin)
        }
    }

    pub fn set_group_ban(&self, group_number: i64, qq_number: i64, ban_time: i64) -> i32{
        unsafe{
            cqpapi::CQ_setGroupBan(self.auth_code, group_number, qq_number, ban_time)
        }
    }

    pub fn set_group_admin(&self, group_number: i64, qq_number: i64, become_admin: i32) -> i32{
        unsafe{
            cqpapi::CQ_setGroupAdmin(self.auth_code, group_number, qq_number, become_admin)
        }
    }

    pub fn set_group_whole_ban(&self, group_number: i64, enable_ban: i32) -> i32{
        unsafe{
            cqpapi::CQ_setGroupWholeBan(self.auth_code, group_number, enable_ban)
        }
    }

    pub fn set_group_anonymous_ban(&self, group_number: i64, anonymous_name: &str, ban_time: i64) -> i32{
        let anonymous_name = UTF8_STR_TO_GB18030_C_CHAR_PTR!(anonymous_name);
        unsafe{
            cqpapi::CQ_setGroupAnonymousBan(self.auth_code, group_number, anonymous_name, ban_time)
        }
    }

    pub fn set_group_anonymous(&self, group_number: i64, enable_anonymous: i32) -> i32{
        unsafe{
            cqpapi::CQ_setGroupAnonymous(self.auth_code, group_number, enable_anonymous)
        }
    }

    pub fn set_group_card(&self, group_number: i64, qq_number: i64, nickname: &str) -> i32{
        let nickname = UTF8_STR_TO_GB18030_C_CHAR_PTR!(nickname);
        unsafe{
            cqpapi::CQ_setGroupCard(self.auth_code, group_number, qq_number, nickname)
        }
    }

    pub fn set_group_leave(&self, group_number: i64, qq_number: i64, dispose_group: i32) -> i32{
        unsafe{
            cqpapi::CQ_setGroupLeave(self.auth_code, group_number, qq_number, dispose_group)
        }
    }

    pub fn set_group_special_title(&self, group_number: i64, qq_number: i64, title: &str, expire_time: i64) -> i32{
        let title = UTF8_STR_TO_GB18030_C_CHAR_PTR!(title);
        unsafe{
            cqpapi::CQ_setGroupSpecialTitle(self.auth_code, group_number, qq_number, title, expire_time)
        }
    }

    pub fn set_discuss_leave(&self, discussion_number: i64) -> i32{
        unsafe{
            cqpapi::CQ_setDiscussLeave(self.auth_code, discussion_number)
        }
    }

    pub fn set_friend_add_request(&self, response_flag: &str, response_type: i32, comment: &str) -> i32{
        let response_flag = UTF8_STR_TO_GB18030_C_CHAR_PTR!(response_flag);
        let comment = UTF8_STR_TO_GB18030_C_CHAR_PTR!(comment);
        unsafe{
            cqpapi::CQ_setFriendAddRequest(self.auth_code, response_flag, response_type, comment)
        }
    }

    pub fn set_group_add_request_v2(&self, response_flag: &str, request_type: i32, response_type: i32, reason: &str) -> i32{
        let response_flag = UTF8_STR_TO_GB18030_C_CHAR_PTR!(response_flag);
        let reason = UTF8_STR_TO_GB18030_C_CHAR_PTR!(reason);
        unsafe{
            cqpapi::CQ_setGroupAddRequestV2(self.auth_code, response_flag, request_type, response_type, reason)
        }
    }

    pub fn get_group_member_info_v2(&self, group_number: i64, qq_number: i64, use_cache: i32) -> &str{
        unsafe{
            CStr::from_ptr(cqpapi::CQ_getGroupMemberInfoV2(self.auth_code, group_number, qq_number, use_cache)).to_str().unwrap()
        }
    }

    pub fn get_stranger_info(&self, qq_number: i64, use_cache: i32) -> &str{
        unsafe{
            CStr::from_ptr(cqpapi::CQ_getStrangerInfo(self.auth_code, qq_number, use_cache)).to_str().unwrap()
        }
    }

    pub fn get_cookies(&self) -> &str{
        unsafe{
            CStr::from_ptr(cqpapi::CQ_getCookies(self.auth_code)).to_str().unwrap()
        }
    }

    pub fn get_csrf_token(&self) -> i32{
        unsafe{
            cqpapi::CQ_getCsrfToken(self.auth_code)
        }
    }

    pub fn get_login_nick(&self) -> &str{
        unsafe{
            CStr::from_ptr(cqpapi::CQ_getLoginNick(self.auth_code)).to_str().unwrap()
        }
    }

    pub fn get_app_directory(&self) -> &str{
        unsafe{
            CStr::from_ptr(cqpapi::CQ_getAppDirectory(self.auth_code)).to_str().unwrap()
        }
    }

    pub fn set_function_mark(&self, function_name: &str) -> i32{
        let function_name = UTF8_STR_TO_GB18030_C_CHAR_PTR!(function_name);
        unsafe{
            cqpapi::CQ_setFunctionMark(self.auth_code, function_name)
        }
    }

    pub fn set_fatal(&self, err_msg: &str) -> i32{
        let err_msg = UTF8_STR_TO_GB18030_C_CHAR_PTR!(err_msg);
        unsafe{
            cqpapi::CQ_setFatal(self.auth_code, err_msg)
        }
    }
}

#[test]
fn cqpclient_new() {
    let mut c = Client::new("app_name");
    assert_eq!(c.auth_code, 0);
    assert_eq!(c.initialized, false);
    assert_eq!(c.api_version, 9);
    assert_eq!(c.app_name,"app_name");
    assert_eq!(c.app_info(), "9,app_name");
    c.initialize(131);
    assert_eq!(c.auth_code, 131);
    assert_eq!(c.initialized, true);
    assert_eq!(c.app_info(), "9,app_name"); 
}