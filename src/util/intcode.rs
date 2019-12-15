#![allow(unused)]
use arraydeque::ArrayDeque;
use num_traits::{
    AsPrimitive, CheckedAdd, CheckedMul, CheckedSub, FromPrimitive, One, ToPrimitive, Zero,
};
use std::error::Error;
use std::ops::{Add, Div, Index, IndexMut, Mul, Rem};

pub type Address = usize;

#[derive(Debug, Default)]
pub struct Emulator<Word>
where
    Word: Copy,
{
    memory: Memory<Word>,
    instruction_pointer: Address,
    state: State<Word>,
    relative_base_offset: Address,
    input_buffer: ArrayDeque<[Word; 8]>,
}

#[derive(Debug, Copy, Clone)]
enum State<Word>
where
    Word: Copy,
{
    Running,
    Halt,
    RequestingInput(Address),
    HoldingOutput(Word),
}

impl<Word> Default for State<Word>
where
    Word: Copy,
{
    fn default() -> Self {
        Self::Running
    }
}

enum OperandMode {
    Position,
    Immediate,
    Relative,
}

impl<Word> From<Word> for OperandMode
where
    Word: AsPrimitive<usize>,
{
    fn from(value: Word) -> Self {
        match value.as_() {
            0 => OperandMode::Position,
            1 => OperandMode::Immediate,
            2 => OperandMode::Relative,
            _ => panic!("invalid operand mode encountered"),
        }
    }
}

#[derive(Debug)]
pub enum RunResult<Word> {
    Halt,
    InputRequest,
    Output(Word),
}

impl<Word> RunResult<Word> {
    pub fn into_option(self) -> Option<Word> {
        match self {
            RunResult::Output(w) => Some(w),
            _ => None,
        }
    }
}

macro_rules! maybe_pointer_increment {
    ($self:ident, $ip_increment:expr) => {
        $self.instruction_pointer += $ip_increment;
    };
    ($self:ident) => {};
}

macro_rules! match_operand {
    ($self:ident, $name:ident, $instruction:ident, [], $multiplier:expr) => {};
    ($self:ident, $name:ident, $instruction:ident, [ $par_mode_name:ident, $($rest:ident,)* ], $multiplier:expr) => {
        let $par_mode_name = $crate::util::intcode::OperandMode::from(($instruction.as_() / $multiplier) % 10);
        match_operand!($self, $name, $instruction, [ $($rest,)* ], $multiplier * 10);
    };
}

macro_rules! instructions {
    (
        $(
            $opcode:expr => $name:ident
                ([ $($operand_name:ident + $operand_offset:expr),* ],
                    [ $($write_operand_name:ident + $write_operand_offset:expr),* ]
                    $($ip_increment:tt)*
                )
                $code:block
        )*
        => $run_instruction:ident ();
    ) => {
        paste::item!{
            $(
                fn $name( &mut self, $([<__ $operand_name _mode>]: $crate::util::intcode::OperandMode,)* $([<__ $write_operand_name _mode>]: $crate::util::intcode::OperandMode,)* ) {
                    $(let [<$operand_name>] = self.get_operand(self.memory[self.instruction_pointer + $operand_offset], [<__ $operand_name _mode>]);)*
                    $(let [<$write_operand_name>] = self.get_operand_address(self.memory[self.instruction_pointer + $write_operand_offset], [<__ $write_operand_name _mode>]);)*
                    $code;
                    maybe_pointer_increment!(self $($ip_increment)*);
                }
            )*
            fn $run_instruction(&mut self, instruction: Word) {
                let opcode = instruction.as_() % 100;
                match opcode {
                    $(
                        $opcode => {
                            match_operand!(self, $name, instruction, [$([<__ $operand_name _mode>],)* $([<__ $write_operand_name _mode>],)*], 100);
                            self.$name($([<__ $operand_name _mode>],)* $([<__ $write_operand_name _mode>],)*);
                        },
                    )*
                    _ => panic!("invalid opcode encountered"),
                }
            }
        }
    };
}

impl<Word> Emulator<Word>
where
    Word: Copy
        + Clone
        + AsPrimitive<Address>
        + ToPrimitive
        + FromPrimitive
        + Add<Output = Word>
        + Mul<Output = Word>
        + Div<Output = Word>
        + Rem<Output = Word>
        + Zero
        + One
        + Eq
        + Ord,
{
    instructions! {
        1 => add ([a + 1, b + 2], [write + 3], 4) {
            self.memory[write] = a + b;
        }
        2 => mul ([a + 1, b + 2], [write + 3], 4) {
            self.memory[write] = a * b;
        }
        3 => input ([], [write + 1], 2) {
            self.state = State::RequestingInput(write.as_());
        }
        4 => output ([read + 1], [], 2) {
            self.state = State::HoldingOutput(read);
        }
        5 => jump_if_true ([test + 1, jump + 2], []) {
            match test.as_() {
                0 => self.instruction_pointer += 3,
                _ => self.instruction_pointer = jump.as_(),
            }
        }
        6 => jump_if_false ([test + 1, jump + 2], []) {
            match test.as_() {
                0 => self.instruction_pointer = jump.as_(),
                _ => self.instruction_pointer += 3,
            }
        }
        7 => less_than ([a + 1, b + 2], [write + 3], 4) {
            self.memory[write] = if a < b {Word::one()} else {Word::zero()}
        }
        8 => equals ([a + 1, b + 2], [write + 3], 4) {
            self.memory[write] = if a == b {Word::one()} else {Word::zero()}
        }
        9 => add_to_relative_base ([rbo + 1], [], 2) {
            self.relative_base_offset = (Word::from_usize(self.relative_base_offset).unwrap() + rbo).as_()
        }
        99 => halt ([], [], 1) {
            self.state = State::Halt;
        }
        => run_instruction();
    }

    fn get_operand(&mut self, value: Word, mode: OperandMode) -> Word {
        match mode {
            OperandMode::Position => self.memory[value.as_()].clone(),
            OperandMode::Immediate => value,
            OperandMode::Relative => self.memory
                [(Word::from_usize(self.relative_base_offset).unwrap() + value).as_()]
            .clone(),
        }
    }
    fn get_operand_address(&mut self, value: Word, mode: OperandMode) -> Address {
        match mode {
            OperandMode::Position => value.as_(),
            OperandMode::Immediate => panic!(),
            OperandMode::Relative => {
                (Word::from_usize(self.relative_base_offset).unwrap() + value).as_()
            }
        }
    }

    pub fn new(memory: Vec<Word>) -> Self {
        Self {
            memory: Memory::new(memory),
            instruction_pointer: 0,
            state: State::Running,
            relative_base_offset: 0,
            input_buffer: Default::default(),
        }
    }

    pub fn run(&mut self) -> RunResult<Word> {
        loop {
            match self.state.clone() {
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
            self.run_instruction(self.memory[self.instruction_pointer].clone());
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

#[derive(Debug, Default)]
struct Memory<Word>
where
    Word: Copy,
{
    inner: Vec<Word>,
    zero: Word,
}

impl<Word> Index<Address> for Memory<Word>
where
    Word: Copy,
{
    type Output = Word;

    fn index(&self, index: Address) -> &Self::Output {
        self.inner.get(index).unwrap_or(&self.zero)
    }
}

impl<Word> IndexMut<Address> for Memory<Word>
where
    Word: Copy,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.inner.len() <= index {
            self.inner.resize(index + 1, self.zero.clone());
        }
        self.inner.get_mut(index).unwrap()
    }
}

impl<Word> Memory<Word>
where
    Word: Copy + Zero,
{
    fn new(memory: Vec<Word>) -> Self {
        Self {
            inner: memory,
            zero: Word::zero(),
        }
    }
}

impl<Word> Memory<Word>
where
    Word: Copy,
{
    fn into_inner(self) -> Vec<Word> {
        self.inner
    }
}

pub fn parse_intcode_text<Word>(input: &[u8]) -> Result<Vec<Word>, Box<dyn Error>>
where
    Word: FromPrimitive + Zero + CheckedAdd + CheckedSub + CheckedMul,
{
    use crate::util::parsers::signed_number;
    use nom::{bytes::complete::tag, combinator::all_consuming, multi::separated_list};
    Ok(
        all_consuming(separated_list(tag(b","), signed_number::<Word>))(input)
            .map_err(|err| format!("Parser error: {:x?}", err))?
            .1,
    )
}
