pub fn median(v:&mut Vec<u8>) -> Option<u8> {
   v.sort();
   if v.len() & 1 == 0 {
       let Some(v1) = v.get(v.len()/2) else {return None};
       let Some(v2) = v.get(v.len()/2 - 1) else {return None};
       return Some( ((*v1 as u16 + *v2 as u16) / 2 as u16) as u8 );
   }
   match v.get(v.len()/2) {
       Some(&i) => Some(i),
       _ => None
   }
}
