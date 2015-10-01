#![feature(const_fn)]
#![feature(libc)]
mod cqpapi;

use std::ffi::{CString, CStr};

extern crate encoding;
use encoding::{Encoding, EncoderTrap};
use encoding::all::GB18030;

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

pub struct CqpApi{
    auth_code :i32,
}

impl CqpApi{
	fn create_cstring(text: &str) -> CString {
		CString::new(GB18030.encode(text,EncoderTrap::Ignore).unwrap()).unwrap()
	}

	pub const fn static_new()->CqpApi{
		CqpApi{auth_code:0}
	}

	pub fn new(auth_code :i32)->CqpApi{
		CqpApi{auth_code:auth_code}
	}

	pub fn get_login_qq(&self)->i64{
		unsafe{
			cqpapi::CQ_getLoginQQ(self.auth_code)
		}
	}

	pub fn send_private_message(&self, qq_number: i64, msg: &str)->i32{
		let msg = CqpApi::create_cstring(msg);
		unsafe{
			cqpapi::CQ_sendPrivateMsg(self.auth_code, qq_number, msg.as_ptr())
		}
	}

	pub fn add_log(&self, priority: LogLevel, t: &str, msg: &str)->i32{
		let t = CqpApi::create_cstring(t);
		let msg = CqpApi::create_cstring(msg);
		unsafe{
			cqpapi::CQ_addLog(self.auth_code, priority.to_int(), t.as_ptr(), msg.as_ptr())
		}
	}

	pub fn send_group_msg(&self, group_number: i64, msg: &str) -> i32{
		let msg = CqpApi::create_cstring(msg);
	    unsafe{
	        cqpapi::CQ_sendGroupMsg(self.auth_code, group_number, msg.as_ptr())
	    }
	}

	pub fn send_discussion_msg(&self, discussion_number: i64, msg: &str) -> i32{
		let msg = CqpApi::create_cstring(msg);
	    unsafe{
	        cqpapi::CQ_sendDiscussMsg(self.auth_code, discussion_number, msg.as_ptr())
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
		let anonymous_name = CqpApi::create_cstring(anonymous_name);
	    unsafe{
	        cqpapi::CQ_setGroupAnonymousBan(self.auth_code, group_number, anonymous_name.as_ptr(), ban_time)
	    }
	}

	pub fn set_group_anonymous(&self, group_number: i64, enable_anonymous: i32) -> i32{
	    unsafe{
	        cqpapi::CQ_setGroupAnonymous(self.auth_code, group_number, enable_anonymous)
	    }
	}

	pub fn set_group_card(&self, group_number: i64, qq_number: i64, nickname: &str) -> i32{
		let nickname = CqpApi::create_cstring(nickname);
	    unsafe{
	        cqpapi::CQ_setGroupCard(self.auth_code, group_number, qq_number, nickname.as_ptr())
	    }
	}

	pub fn set_group_leave(&self, group_number: i64, qq_number: i64, dispose_group: i32) -> i32{
	    unsafe{
	        cqpapi::CQ_setGroupLeave(self.auth_code, group_number, qq_number, dispose_group)
	    }
	}

	pub fn set_group_special_title(&self, group_number: i64, qq_number: i64, title: &str, expire_time: i64) -> i32{
		let title = CqpApi::create_cstring(title);
	    unsafe{
	        cqpapi::CQ_setGroupSpecialTitle(self.auth_code, group_number, qq_number, title.as_ptr(), expire_time)
	    }
	}

	pub fn set_discuss_leave(&self, discussion_number: i64) -> i32{
	    unsafe{
	        cqpapi::CQ_setDiscussLeave(self.auth_code, discussion_number)
	    }
	}

	pub fn set_friend_add_request(&self, response_flag: &str, response_type: i32, comment: &str) -> i32{
		let response_flag = CqpApi::create_cstring(response_flag);
		let comment = CqpApi::create_cstring(comment);
	    unsafe{
	        cqpapi::CQ_setFriendAddRequest(self.auth_code, response_flag.as_ptr(), response_type, comment.as_ptr())
	    }
	}

	pub fn set_group_add_request_v2(&self, response_flag: &str, request_type: i32, response_type: i32, reason: &str) -> i32{
		let response_flag = CqpApi::create_cstring(response_flag);
		let reason = CqpApi::create_cstring(reason);
	    unsafe{
	        cqpapi::CQ_setGroupAddRequestV2(self.auth_code, response_flag.as_ptr(), request_type, response_type, reason.as_ptr())
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
		let function_name = CqpApi::create_cstring(function_name);
	    unsafe{
	        cqpapi::CQ_setFunctionMark(self.auth_code, function_name.as_ptr())
	    }
	}

	pub fn set_fatal(&self, err_msg: &str) -> i32{
		let err_msg = CqpApi::create_cstring(err_msg);
	    unsafe{
	        cqpapi::CQ_setFatal(self.auth_code, err_msg.as_ptr())
	    }
	}
}

