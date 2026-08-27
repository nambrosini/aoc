#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum OpCode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl OpCode {
    pub const ALL: [OpCode; 16] = [
        OpCode::Addr,
        OpCode::Addi,
        OpCode::Mulr,
        OpCode::Muli,
        OpCode::Banr,
        OpCode::Bani,
        OpCode::Borr,
        OpCode::Bori,
        OpCode::Setr,
        OpCode::Seti,
        OpCode::Gtir,
        OpCode::Gtri,
        OpCode::Gtrr,
        OpCode::Eqir,
        OpCode::Eqri,
        OpCode::Eqrr,
    ];
}

impl From<&str> for OpCode {
    fn from(s: &str) -> Self {
        match s {
            "addr" => Self::Addr,
            "addi" => Self::Addi,
            "mulr" => Self::Mulr,
            "muli" => Self::Muli,
            "banr" => Self::Banr,
            "bani" => Self::Bani,
            "borr" => Self::Borr,
            "bori" => Self::Bori,
            "setr" => Self::Setr,
            "seti" => Self::Seti,
            "gtir" => Self::Gtir,
            "gtri" => Self::Gtri,
            "gtrr" => Self::Gtrr,
            "eqir" => Self::Eqir,
            "eqri" => Self::Eqri,
            "eqrr" => Self::Eqrr,
            _ => unreachable!(),
        }
    }
}

/// Executes one Elfcode instruction against `regs`, returning the resulting
/// register state. `a`/`b` are raw operand values (already resolved to
/// either a register index or an immediate, per the opcode's addressing
/// mode); `c` is always the destination register index.
pub fn execute(op: OpCode, regs: &[usize], a: usize, b: usize, c: usize) -> Vec<usize> {
    let mut res = regs.to_vec();

    res[c] = match op {
        OpCode::Addr => regs[a] + regs[b],
        OpCode::Addi => regs[a] + b,
        OpCode::Mulr => regs[a] * regs[b],
        OpCode::Muli => regs[a] * b,
        OpCode::Banr => regs[a] & regs[b],
        OpCode::Bani => regs[a] & b,
        OpCode::Borr => regs[a] | regs[b],
        OpCode::Bori => regs[a] | b,
        OpCode::Setr => regs[a],
        OpCode::Seti => a,
        OpCode::Gtir => usize::from(a > regs[b]),
        OpCode::Gtri => usize::from(regs[a] > b),
        OpCode::Gtrr => usize::from(regs[a] > regs[b]),
        OpCode::Eqir => usize::from(a == regs[b]),
        OpCode::Eqri => usize::from(regs[a] == b),
        OpCode::Eqrr => usize::from(regs[a] == regs[b]),
    };

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute() {
        assert_eq!(
            execute(OpCode::Mulr, &[3, 2, 1, 1], 2, 1, 2),
            vec![3, 2, 2, 1]
        );
        assert_eq!(
            execute(OpCode::Addi, &[3, 2, 1, 1], 2, 1, 2),
            vec![3, 2, 2, 1]
        );
        assert_eq!(
            execute(OpCode::Seti, &[3, 2, 1, 1], 9, 0, 2),
            vec![3, 2, 9, 1]
        );
    }
}
