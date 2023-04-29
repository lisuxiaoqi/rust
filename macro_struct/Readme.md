下面是一段调用macro的代码，请把其中的parse_tx宏补全，目标是将下面的代码解析为一个结构体。其中的hex字符串需要使用hex crate解析成Vec<u8>类型。
```
fn main(){
	
	parseTx!({
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
				capacity:10000
			}
		],
		outputs:[
			{
				index:0,
				capacity:10000
			},
			{
				index:1,
				capacity:10000
			},
		],
		digest:"01bee5c80a6bd74440f0f96c983b1107f1a419e028bef7b33e77e8f968cbfa"
	});
}
```

这段宏的关键，在于通过宏的调用。宏调用的时候，传递的参数应该为tt类型。才能继续展开

