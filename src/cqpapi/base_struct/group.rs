use base64;

use super::utils;

#[derive(Serialize, Deserialize, Debug)]
pub struct Group{
    id :i64,
    name :String
}

impl Group{
    fn from_bytes(bytes: &[u8])->Group{
        let (group_id,left_index) = utils::get_i64(&bytes, 0);
        let (group_name,_) = utils::get_string(&bytes, left_index);
        Group{id:group_id, name:group_name}
    }

    pub fn parse_to_group_list(rawdata: &str)->Vec<Group>{
	    let bytes = base64::decode(rawdata).unwrap();
	    
	    let mut left_index :usize = 0;
	    let mut right_index :usize = 4;
	    let (count, _) = utils::get_usize(&bytes, left_index, 4);

	    let mut group_list : Vec<Group> = Vec::with_capacity(count);

	    for _ in 0..count{
	        left_index = right_index;
	        let (token_size,left_index) = utils::get_usize(&bytes, left_index, 2);

	        right_index = left_index + token_size;
	        let token = &bytes[left_index..right_index];
	        
	        let g = Group::from_bytes(&token);
	        group_list.push(g);
	    }
	    group_list	    
	}
}