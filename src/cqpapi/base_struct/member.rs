use base64;

use super::utils;

#[derive(Serialize, Deserialize)]
pub struct Member{
    group_id :i64,
    qq_id :i64,
    nickname :String,
    namecard :String,
    sex :i32,
    age :i32,
    area :String,
    join_time :i32,
    last_active :i32,
    level_name :String,
    permission :i32,
    bad_record :bool,
    special_title :String,
    special_title_express_time :i32,
    allow_modify_namecard :bool
}

impl Member{
    fn from_bytes(bytes: &[u8])->Member{
        let (group_id, offset)  = utils::get_i64(&bytes, 0);
        let (qq_id, offset) = utils::get_i64(&bytes, offset);
        let (nickname, offset) = utils::get_string(&bytes, offset);
        let (namecard, offset) = utils::get_string(&bytes, offset);
        let (sex, offset) = utils::get_i32(&bytes, offset);
        let (age, offset) = utils::get_i32(&bytes, offset);
        let (area, offset) = utils::get_string(&bytes, offset);
        let (join_time, offset) = utils::get_i32(&bytes, offset);
        let (last_active, offset) = utils::get_i32(&bytes, offset);
        let (level_name, offset) = utils::get_string(&bytes, offset);
        let (permission, offset) = utils::get_i32(&bytes, offset);
        let (bad_record, offset) = utils::get_i32(&bytes, offset);
        let bad_record = bad_record == 1;
        let (special_title, offset) = utils::get_string(&bytes, offset);
        let (special_title_express_time, offset) = utils::get_i32(&bytes, offset);
        let (allow_modify_namecard, _) = utils::get_i32(&bytes, offset);
        let allow_modify_namecard = allow_modify_namecard == 1;

        Member{group_id :group_id,
        qq_id :qq_id,
        nickname :nickname,
        namecard :namecard,
        sex :sex,
        age :age,
        area :area,
        join_time :join_time,
        last_active :last_active,
        level_name :level_name,
        permission :permission,
        bad_record :bad_record,
        special_title :special_title,
        special_title_express_time :special_title_express_time,
        allow_modify_namecard :allow_modify_namecard}
    }

    pub fn parse_to_member(rawdata: &str)->Member{
        let bytes = base64::decode(rawdata).unwrap();
        Member::from_bytes(&bytes)
    }

    pub fn parse_to_member_list(rawdata: &str)->Vec<Member>{
        let bytes = base64::decode(rawdata).unwrap();
        
        let mut left_index :usize = 0;
        let mut right_index :usize = 4;
        let (count, _) = utils::get_usize(&bytes, left_index, 4);

        let mut member_list : Vec<Member> = Vec::with_capacity(count);

        for _ in 0..count{
            left_index = right_index;
            let (token_size,left_index) = utils::get_usize(&bytes, left_index, 2);
            
            right_index = left_index + token_size;
            let token = &bytes[left_index..right_index];
            let m = Member::from_bytes(token);

            member_list.push(m);
        }
        member_list
    }
}