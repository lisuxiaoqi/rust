extern crate hex;

#[derive(Debug)]
struct Tx{
    tx_hash:String,
    fee:u32,
    action:TxAction,
    inputs:Vec<Payload>,
    outputs:Vec<Payload>,
    digest:Vec<u8>,
}

#[derive(Debug)]
struct TxAction{
    action:String,
    params:String
}

#[derive(Debug)]
struct Payload{
    index:u8,
    capacity:u64
}

macro_rules! proc_tt{
    // tx_hash
    ($tx:tt, tx_hash, $v:expr)=>{
       $tx.tx_hash = $v.to_string() 
    };

    // fee
    ($tx:tt, fee, $v:expr)=>{
       $tx.fee = $v+1 
    };

    // digest 
    ($tx:tt, digest, $v:expr)=>{
       $tx.digest = hex::decode($v).unwrap()
    };

    //action{..}
    ($tx:tt, action, {$($k:tt : $v:tt),*})=>{
       $(
           proc_tt!($tx, $k, $v);
        )* 
    };

    //action
    ($tx:tt, action, $v:expr)=>{
       $tx.action.action = $v.to_string() 
    };

    //params
    ($tx:tt, params, $v:expr)=>{
       $tx.action.params = $v.to_string() 
    };

    //inputs[..]
    ($tx:tt, inputs, [$($arr:tt),*])=>{{
       $(
           $tx.inputs.push(proc_tt!(payload, $arr));
        )* 
    }};
    
    //outputs[..]
    ($tx:tt, outputs, [$($arr:tt),*])=>{{
       $(
           $tx.outputs.push(proc_tt!(payload, $arr));
        )* 
    }};

    //payload{..}
    (payload, {$($k:tt : $v:tt),*})=>{{
        let mut p = Payload{
            index:0,
            capacity:0
        };
       $(
           proc_tt!(p, $k, $v);
        )* 
       p
    }};
    
    //index
    ($p:tt, index, $v:expr)=>{
       $p.index = $v 
    };
    
    //capacity
    ($p:tt, capacity, $v:expr)=>{
       $p.capacity = $v 
    };

    ($($x:tt)*)=>{};
}

//匹配k,v对，然后把v的内容继续调用proc_tt宏匹配
//注意，此时v必须要为tt类型才能正常调用
macro_rules! foo {
    ({$($key:tt : $val:tt),*}) => {{
       let mut tx = Tx{
            tx_hash:String::new(),
            fee:0,
            action:TxAction{
                action:String::new(),
                params:String::new()
            },
            inputs:Vec::new(),
            outputs:Vec::new(),
            digest:Vec::new()
       };
       $(
           proc_tt!(tx, $key, $val);
        )*
        
        tx
    }};
}

fn main() {
    let t = foo!({
		tx_hash:"01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfa",
		fee:10000,
        action:{
			action:"register",
			params:"0x00"
		},
        inputs:[
			{
				index:0,
				capacity:10000
			},
			{
				index:1,
				capacity:20000
			}
		],
        outputs:[
			{
				index:0,
				capacity:30000
			},
			{
				index:1,
				capacity:40000
			}
		],
        digest:"01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfa"
	});
    
    println!("{:?}", t);
}

