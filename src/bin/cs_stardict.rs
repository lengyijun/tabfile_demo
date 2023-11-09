use std::fs::File;
use std::io::prelude::*;

#[rustfmt::skip]
const STR : &str = 
"
WIP\u{0009}Work in progress, do not merge yet.\\n开发中
LGTM\u{0009}Looks good to me.\\n Riview 完别人的 PR ，没有问题
SGTM\u{0009}Sounds Good To Me
PTAL\u{0009} Please take a look.\\n 帮我看下，一般都是请别人 review 自己的 PR
CC\u{0009} Carbon copy\\n 一般代表抄送别人的意思
RFC\u{0009} request for comments.\\n 我觉得这个想法很好, 我们来一起讨论下
IIRC\u{0009} if I recall correctly.  \\n 如果我没记错
IIUC\u{0009} if I understand correctly.\\n 如果我没理解错
ACK\u{0009} acknowledgement.\\n 我确认了或者我接受了,我承认了
NACK/NAK\u{0009} negative acknowledgement.\\n 我不同意
FWIW\u{0009} for what it’s worth\\n 当有人将其放在消息的开头或结尾时，这意味着消息中的信息可能对读者不感兴趣，或者发布消息的人不是专家。\\n 也就是说发件人不确定对方是否真的需要或想要他们发送的信息
IMO\u{0009}in my opinion
IMHO\u{0009}in my humble opinion
AFAIK\u{0009} as far as I know
AFAICT\u{0009} as far as I can tell
RN\u{0009} right now
TBH\u{0009} to be honest
AFK\u{0009} away from keyboard
tl;dr\u{0009} Too Long Didn't Read
FP\u{0009} false positive
iff\u{0009} if and only if
ditto\u{0009} 同上
aka\u{0009} also known as
OBOE, OBO, OB1 , OBOB\u{0009} off-by-one error or off-by-one bug
NB\u{0009} please note\\n used in writing to tell the reader that something is important
FYI\u{0009} for your information\\n It is often used in both personal and business correspondence to show that information is simply being shared and that no immediate action is required or expected.\\n  https://www.techtarget.com/whatis/definition/for-your-information-FYI#:~:text=In%20both%20chat%20acronyms%20and,action%20is%20required%20or%20expected.
JFYI\u{0009} just for your information
FYA\u{0009} for your amusement
MCP\u{0009} major change proposal
STR\u{0009} Short Tandem Repeat\\n 简短且可重现
SSSP\u{0009} 单源最短路算法
SSA\u{0009} Static single assignment
lol\u{0009} Laughing out loud
KEKW\u{0009} laugh
w.r.t.\u{0009} with reference to
GCSE\u{0009}全局的公共子表达式消除
SCCP\u{0009}常量传播并移除控制流图中的死路径
RA\u{0009}可达性分析
CP\u{0009}赋值传播
DCE\u{0009}死代码消除
LA\u{0009}循环分析，可以提取循环不变量等
meh\u{0009}- great\\n - good\\n - ok\\n - meh\\n - yuck
yuck\u{0009}- great\\n - good\\n - ok\\n - meh\\n - yuck
";

fn main() -> std::io::Result<()> {
    let mut file = File::create("cs.tab")?;
    file.write_all(STR.as_bytes())?;
    Ok(())
}
