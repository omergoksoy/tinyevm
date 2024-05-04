use tinyevm::{
    eval_opcode, lex_bytecode,
    types::{to_uint256, Endian},
    Stack,
};

fn main() {
    let opcodes = lex_bytecode("0x6001600101").unwrap();
    // let opcodes = lex_bytecode("0x60 01 60 01 01").unwrap();
    println!("{:?}", opcodes.clone());
    let (mut stack, _storage, _memory) = eval_opcode(opcodes);
    let (last, rest) = stack.pop();
    assert_eq!(last.unwrap(), to_uint256(&[0x02], Endian::Little));
    assert_eq!(*rest, Stack::EMPTY);
}
