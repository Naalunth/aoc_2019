#![allow(unused)]
use arraydeque::ArrayDeque;
use std::error::Error;
use std::ops::{Index, IndexMut};

pub type Word = i64;
pub type Address = usize;

#[derive(Debug, Default)]
pub struct Emulator {
    memory: Memory,
    instruction_pointer: Address,
    state: State,
    relative_base_offset: Address,
    input_buffer: ArrayDeque<[Word; 8]>,
}

#[derive(Debug)]
enum State {
    Running,
    Halt,
    RequestingInput(Address),
    HoldingOutput(Word),
}

trait OperandMode {
    fn load(emulator: &Emulator, value: Word) -> Word;
    unsafe fn load_unsafe(emulator: &Emulator, value: Word) -> Word {
        Self::load(emulator, value)
    }
    fn address(emulator: &Emulator, value: Word) -> Address;
}

struct PositionMode;
impl OperandMode for PositionMode {
    fn load(emulator: &Emulator, value: Word) -> Word {
        emulator.memory[value as Address]
    }
    unsafe fn load_unsafe(emulator: &Emulator, value: Word) -> Word {
        *emulator.memory.get_unchecked(value as Address)
    }

    fn address(emulator: &Emulator, value: i64) -> Address {
        value as Address
    }
}

struct ImmediateMode;
impl OperandMode for ImmediateMode {
    fn load(_: &Emulator, value: Word) -> Word {
        value
    }
    fn address(emulator: &Emulator, value: i64) -> Address {
        panic!("no")
    }
}

struct RelativeMode;
impl OperandMode for RelativeMode {
    fn load(emulator: &Emulator, value: Word) -> Word {
        emulator.memory[(emulator.relative_base_offset as Word + value) as Address]
    }

    fn address(emulator: &Emulator, value: i64) -> Address {
        (emulator.relative_base_offset as Word + value) as Address
    }
}

//enum OperandModeThing {
//    Position,
//    Immediate,
//}
//
//impl From<Word> for OperandModeThing {
//    fn from(value: i64) -> Self {
//        match value {
//            0 => OperandModeThing::Position,
//            1 => OperandModeThing::Immediate,
//            _ => panic!("invalid operand mode encountered"),
//        }
//    }
//}

#[derive(Debug)]
pub enum RunResult {
    Halt,
    InputRequest,
    Output(Word),
}

macro_rules! maybe_pointer_increment {
    ($self:ident, $ip_increment:expr) => {
        $self.instruction_pointer += $ip_increment;
    };
    ($self:ident) => {};
}

macro_rules! match_operand {
    ($self:ident, $name:ident, $instruction:ident, [], $multiplier:expr, [$($op_ty:ty,)*]) => {
        $self.$name::<$($op_ty),*>();
    };
    ($self:ident, $name:ident, $instruction:ident, [ $operand_type:ident, $($rest:ident,)* ], $multiplier:expr, [$($op_ty:ty,)*]) => {
        match ($instruction / $multiplier) % 10 {
            0 => {
                match_operand!($self, $name, $instruction, [$($rest,)*], $multiplier * 10, [$($op_ty,)* $crate::util::intcode::PositionMode,]);
            },
            1 => {
                match_operand!($self, $name, $instruction, [$($rest,)*], $multiplier * 10, [$($op_ty,)* $crate::util::intcode::ImmediateMode,]);
            },
            2 => {
                match_operand!($self, $name, $instruction, [$($rest,)*], $multiplier * 10, [$($op_ty,)* $crate::util::intcode::RelativeMode,]);
            },
            _ => panic!("unexpected operand mode encountered"),
        }
    };
}

macro_rules! instructions {
    (
        $(
            $opcode:expr => $name:ident
                ([ $($operand_name:ident + $operand_offset:expr),* ],
                    [ $($write_operand_name:ident + $write_operand_offset:expr),* ],
                    [ $($manual_operand_name:ident + $manual_operand_offset:expr),* ]
                    $($ip_increment:tt)*
                )
                $code:block
        )*
        fn $run_instruction:ident ();
    ) => {
        paste::item!{
            $(
                fn $name< $( [<Type $operand_name>] : OperandMode, )* $( [<Type $write_operand_name>] : OperandMode, )* > ( &mut self ) {
                    $(let [<$operand_name>] = [<Type $operand_name>] ::load(&self, self.memory[self.instruction_pointer + $operand_offset]);)*
                    $(let [<$write_operand_name>] = [<Type $write_operand_name>] ::address(&self, self.memory[self.instruction_pointer + $write_operand_offset]);)*
                    $(let [<$manual_operand_name>] = self.memory[self.instruction_pointer + $manual_operand_offset] as Address;)*
                    $code;
                    maybe_pointer_increment!(self $($ip_increment)*);
                }
            )*
            fn $run_instruction(&mut self, instruction: Word) {
                let opcode = instruction % 100;
                match opcode {
                    $(
                        $opcode => {
                            match_operand!(self, $name, instruction, [$( [<Type $operand_name>], )* $( [<Type $write_operand_name>], )*], 100, []);
                        },
                    )*
                    _ => panic!("invalid opcode encountered"),
                }
            }
        }
    };
}

macro_rules! instructions_unsafe {
    (
        $(
            $opcode:expr => $name:ident
                ([ $($operand_name:ident + $operand_offset:expr),* ],
                    [ $($write_operand_name:ident + $write_operand_offset:expr),* ],
                    [ $($manual_operand_name:ident + $manual_operand_offset:expr),* ]
                    $($ip_increment:tt)*
                )
                $code:block
        )*
        fn $run_instruction:ident ();
    ) => {
        paste::item!{
            $(
                unsafe fn [<$name _unsafe>] < $( [<Type $operand_name>] : OperandMode, )* $( [<Type $write_operand_name>] : OperandMode, )* > ( &mut self ) {
                    $(let [<$operand_name>] = [<Type $operand_name>] ::load_unsafe(&self, *self.memory.get_unchecked(self.instruction_pointer + $operand_offset));)*
                    $(let [<$write_operand_name>] = [<Type $write_operand_name>] ::address(&self, self.memory[self.instruction_pointer + $write_operand_offset]);)*
                    $(let [<$manual_operand_name>] = *self.memory.get_unchecked(self.instruction_pointer + $manual_operand_offset) as Address;)*
                    $code;
                    maybe_pointer_increment!(self $($ip_increment)*);
                }
            )*
            unsafe fn $run_instruction(&mut self, instruction: Word) {
                let opcode = instruction % 100;
                match opcode {
                    $(
                        $opcode => {
                            match_operand!(self, [<$name _unsafe>], instruction, [$( [<Type $operand_name>], )* $( [<Type $write_operand_name>], )*], 100, []);
                        },
                    )*
                    _ => panic!("invalid opcode encountered"),
                }
            }
        }
    };
}

//macro_rules! match_operand {
//    ($self:ident, $name:ident, $instruction:ident, [], $multiplier:expr) => {};
//    ($self:ident, $name:ident, $instruction:ident, [ $par_mode_name:ident, $($rest:ident,)* ], $multiplier:expr) => {
//        let $par_mode_name = $crate::util::intcode::OperandModeThing::from(($instruction / $multiplier) % 10);
//        match_operand!($self, $name, $instruction, [ $($rest,)* ], $multiplier * 10);
//    };
//}
//
//macro_rules! instructions {
//    (
//        $(
//            $opcode:expr => $name:ident
//                ([ $($par_name:ident + $par_offset:expr),* ], [ $($man_par_name:ident + $man_par_offset:expr),* ] $($ip_increment:tt)*)
//                $code:block
//        )*
//        fn $run_instruction:ident ();
//    ) => {
//        paste::item!{
//            $(
//                fn $name( &mut self, $([<__ $par_name _mode>]: $crate::util::intcode::OperandModeThing),* ) {
//                    $(let [<$par_name>] = self.get_operand(self.memory[self.instruction_pointer + $par_offset], [<__ $par_name _mode>]);)*
//                    $(let [<$man_par_name>] = self.memory[self.instruction_pointer + $man_par_offset] as Address;)*
//                    $code;
//                    maybe_pointer_increment!(self $($ip_increment)*);
//                }
//            )*
//            fn $run_instruction(&mut self, instruction: Word) {
//                let opcode = instruction % 100;
//                match opcode {
//                    $(
//                        $opcode => {
//                            match_operand!(self, $name, instruction, [$([<__ $par_name _mode>] , )*], 100);
//                            self.$name($([<__ $par_name _mode>]),*);
//                        },
//                    )*
//                    _ => panic!("invalid opcode encountered"),
//                }
//            }
//        }
//    };
//}

impl Emulator {
    instructions! {
        1 => add ([a + 1, b + 2], [write + 3], [], 4) {
            self.memory[write] = a + b;
        }
        2 => mul ([a + 1, b + 2], [write + 3], [], 4) {
            self.memory[write] = a * b;
        }
        3 => input ([], [write + 1], [], 2) {
            self.state = State::RequestingInput(write as Address);
        }
        4 => output ([read + 1], [], [], 2) {
            self.state = State::HoldingOutput(read);
        }
        5 => jump_if_true ([test + 1, jump + 2], [], []) {
            match test {
                0 => self.instruction_pointer += 3,
                _ => self.instruction_pointer = jump as Address,
            }
        }
        6 => jump_if_false ([test + 1, jump + 2], [], []) {
            match test {
                0 => self.instruction_pointer = jump as Address,
                _ => self.instruction_pointer += 3,
            }
        }
        7 => less_than ([a + 1, b + 2], [write + 3], [], 4) {
            self.memory[write] = if a < b {1} else {0}
        }
        8 => equals ([a + 1, b + 2], [write + 3], [], 4) {
            self.memory[write] = if a == b {1} else {0}
        }
        9 => add_to_relative_base ([rbo + 1], [], [], 2) {
            self.relative_base_offset += rbo as Address;
        }
        99 => halt ([], [], [], 1) {
            self.state = State::Halt;
        }
        fn run_instruction();
    }

    instructions_unsafe! {
        1 => add ([a + 1, b + 2], [write + 3], [], 4) {
            *self.memory.get_unchecked_mut(write) = a + b;
        }
        2 => mul ([a + 1, b + 2], [write + 3], [], 4) {
            *self.memory.get_unchecked_mut(write) = a * b;
        }
        3 => input ([], [write + 1], [], 2) {
            self.state = State::RequestingInput(write as Address);
        }
        4 => output ([read + 1], [], [], 2) {
            self.state = State::HoldingOutput(read);
        }
        5 => jump_if_true ([test + 1, jump + 2], [], []) {
            match test {
                0 => self.instruction_pointer += 3,
                _ => self.instruction_pointer = jump as Address,
            }
        }
        6 => jump_if_false ([test + 1, jump + 2], [], []) {
            match test {
                0 => self.instruction_pointer = jump as Address,
                _ => self.instruction_pointer += 3,
            }
        }
        7 => less_than ([a + 1, b + 2], [write + 3], [], 4) {
            *self.memory.get_unchecked_mut(write) = if a < b {1} else {0}
        }
        8 => equals ([a + 1, b + 2], [write + 3], [], 4) {
            *self.memory.get_unchecked_mut(write) = if a == b {1} else {0}
        }
        9 => add_to_relative_base ([rbo + 1], [], [], 2) {
            self.relative_base_offset += rbo as Address;
        }
        99 => halt ([], [], [], 1) {
            self.state = State::Halt;
        }
        fn run_instruction_unchecked();
    }

    //    fn get_operand(&mut self, value: Word, mode: OperandModeThing) -> i64 {
    //        match mode {
    //            OperandModeThing::Position => self.memory[value as Address],
    //            OperandModeThing::Immediate => value,
    //        }
    //    }

    pub fn new(memory: Vec<Word>) -> Self {
        Self {
            memory: Memory::new(memory),
            instruction_pointer: 0,
            state: State::Running,
            relative_base_offset: 0,
            input_buffer: Default::default(),
        }
    }

    pub fn run(&mut self) -> RunResult {
        loop {
            match self.state {
                State::HoldingOutput(output) => {
                    self.state = State::Running;
                    return RunResult::Output(output);
                }
                State::RequestingInput(address) => {
                    if let Some(input) = self.input_buffer.pop_front() {
                        self.memory[address] = input;
                        self.state = State::Running;
                    } else {
                        return RunResult::InputRequest;
                    }
                }
                State::Halt => {
                    return RunResult::Halt;
                }
                _ => {}
            }
            self.run_instruction(self.memory[self.instruction_pointer]);
        }
    }

    pub unsafe fn run_unchecked(&mut self) -> RunResult {
        loop {
            match self.state {
                State::HoldingOutput(output) => {
                    self.state = State::Running;
                    return RunResult::Output(output);
                }
                State::RequestingInput(address) => {
                    if let Some(input) = self.input_buffer.pop_front() {
                        *self.memory.get_unchecked_mut(address) = input;
                        self.state = State::Running;
                    } else {
                        return RunResult::InputRequest;
                    }
                }
                State::Halt => {
                    return RunResult::Halt;
                }
                _ => {}
            }
            self.run_instruction_unchecked(*self.memory.get_unchecked(self.instruction_pointer));
        }
    }

    pub fn push_input(&mut self, input: Word) {
        self.input_buffer.push_back(input).unwrap()
    }

    pub fn extend_input(&mut self, input: impl IntoIterator<Item = Word>) {
        self.input_buffer.extend(input);
    }

    pub fn into_memory(self) -> Vec<Word> {
        self.memory.into_inner()
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Running
    }
}

#[derive(Debug, Default)]
struct Memory {
    inner: Vec<Word>,
}

impl Index<Address> for Memory {
    type Output = Word;

    fn index(&self, index: Address) -> &Self::Output {
        self.inner.get(index).unwrap_or(&0)
    }
}

impl IndexMut<Address> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.inner.len() <= index {
            self.inner.resize(index + 1, 0);
        }
        self.inner.get_mut(index).unwrap()
    }
}

impl Memory {
    fn new(memory: Vec<Word>) -> Self {
        Self { inner: memory }
    }
    unsafe fn get_unchecked(&self, index: Address) -> &Word {
        self.inner.get_unchecked(index)
    }
    unsafe fn get_unchecked_mut(&mut self, index: Address) -> &mut Word {
        self.inner.get_unchecked_mut(index)
    }
    fn into_inner(self) -> Vec<Word> {
        self.inner
    }
}

pub fn parse_intcode_text(input: &[u8]) -> Result<Vec<Word>, Box<dyn Error>> {
    use crate::util::parsers::signed_number;
    use nom::{bytes::complete::tag, combinator::all_consuming, multi::separated_list};
    Ok(
        all_consuming(separated_list(tag(b","), signed_number::<Word>))(input)
            .map_err(|err| format!("Parser error: {:x?}", err))?
            .1,
    )
}
