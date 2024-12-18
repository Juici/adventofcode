#![feature(str_lines_remainder)]

use std::sync::LazyLock;

use anyhow::{Context, Result};
use regex::Regex;
use z3::ast::Ast;

#[macro_use]
mod uint3;

use self::uint3::u3;

const INPUT: &str = include_str!("./input");

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

type Int = u64;

#[derive(Clone, Copy, Debug)]
struct Cpu {
    reg_a: Int,
    reg_b: Int,
    reg_c: Int,
    pc: usize,
}

impl Cpu {
    fn combo_operand(&self, operand: u3) -> Int {
        match operand {
            v if v == u3!(4) => self.reg_a,
            v if v == u3!(5) => self.reg_b,
            v if v == u3!(6) => self.reg_c,
            v if v == u3!(7) => panic!("reserved combo operand: {v:#x}"),
            v => Int::from(v.to_u8()),
        }
    }

    fn combo_operand_u3(&self, operand: u3) -> u3 {
        match operand {
            v if v == u3!(4) => u3::from_u8(self.reg_a as u8),
            v if v == u3!(5) => u3::from_u8(self.reg_b as u8),
            v if v == u3!(6) => u3::from_u8(self.reg_c as u8),
            v if v == u3!(7) => panic!("reserved combo operand: {v:#x}"),
            v => v,
        }
    }

    fn run(&mut self, rom: &[u3]) -> Vec<u3> {
        let mut output = Vec::new();

        while let Some(opcode) = rom.get(self.pc) {
            let opcode = opcode.to_opcode();
            let operand = rom[self.pc + 1];

            self.pc += 2;

            match opcode {
                Opcode::Adv => {
                    self.reg_a >>= self.combo_operand(operand);
                }
                Opcode::Bxl => {
                    self.reg_b ^= Int::from(operand.to_u8());
                }
                Opcode::Bst => {
                    self.reg_b = Int::from(self.combo_operand_u3(operand).to_u8());
                }
                Opcode::Jnz => {
                    if self.reg_a == 0 {
                        continue;
                    }
                    self.pc = usize::from(operand.to_u8());
                }
                Opcode::Bxc => {
                    self.reg_b ^= self.reg_c;
                }
                Opcode::Out => {
                    output.push(self.combo_operand_u3(operand));
                }
                Opcode::Bdv => {
                    self.reg_b = self.reg_a >> self.combo_operand(operand);
                }
                Opcode::Cdv => {
                    self.reg_c = self.reg_a >> self.combo_operand(operand);
                }
            }
        }

        output
    }
}

fn main() -> Result<()> {
    let (cpu, rom) = parse_input(INPUT).context("failed to parse input")?;

    println!("part 1: {}", part1(cpu, &rom));
    println!("part 2: {}", part2(cpu, &rom));

    Ok(())
}

fn part1(mut cpu: Cpu, rom: &[u3]) -> String {
    render_values(cpu.run(rom))
}

fn part2(cpu: Cpu, rom: &[u3]) -> Int {
    use z3::ast::BV;

    let ctx = z3::Context::new(&z3::Config::new());
    let opt = z3::Optimize::new(&ctx);

    let s = BV::new_const(&ctx, "a", 64);

    let mut a = s.clone();
    let mut b = BV::from_u64(&ctx, cpu.reg_b, 64);
    let mut c = BV::from_u64(&ctx, cpu.reg_c, 64);

    let mut pc = 0;

    macro_rules! combo_operand {
        ($operand:expr) => {
            match $operand {
                v if v == u3!(4) => a.clone(),
                v if v == u3!(5) => b.clone(),
                v if v == u3!(6) => c.clone(),
                v if v == u3!(7) => panic!("reserved combo operand: {v:#x}"),
                v => BV::from_u64(&ctx, v.to_u8().into(), 64),
            }
        };
    }

    let mut next_out = rom.iter().copied().peekable();

    while let Some(opcode) = rom.get(pc) {
        let opcode = opcode.to_opcode();
        let operand = rom[pc + 1];

        pc += 2;

        match opcode {
            Opcode::Adv => {
                a = a.bvlshr(&combo_operand!(operand));
            }
            Opcode::Bxl => {
                b ^= BV::from_u64(&ctx, operand.to_u8().into(), 64);
            }
            Opcode::Bst => {
                b = combo_operand!(operand) & BV::from_u64(&ctx, 7, 64);
            }
            Opcode::Jnz => {
                if next_out.peek().is_none() {
                    opt.assert(&a._eq(&BV::from_u64(&ctx, 0, 64)));
                    break;
                }
                pc = usize::from(operand.to_u8());
            }
            Opcode::Bxc => {
                b ^= &c;
            }
            Opcode::Out => {
                let expected = next_out.next().expect("no more rom");
                let out = combo_operand!(operand) & BV::from_u64(&ctx, 7, 64);

                opt.assert(&out._eq(&BV::from_u64(&ctx, expected.to_u8().into(), 64)));
            }
            Opcode::Bdv => {
                b = a.bvlshr(&combo_operand!(operand));
            }
            Opcode::Cdv => {
                c = a.bvlshr(&combo_operand!(operand));
            }
        }
    }

    opt.minimize(&s);

    assert_eq!(next_out.next(), None);
    assert_eq!(opt.check(&[]), z3::SatResult::Sat);

    let a = opt.get_model().unwrap().eval(&s, true).unwrap().as_u64().unwrap();

    a
}

fn render_values(values: impl IntoIterator<Item = u3>) -> String {
    let mut buf = String::new();
    let mut values = values.into_iter().map(|v| v.to_u8());

    let Some(first) = values.next() else { return buf };

    buf.push_str(itoa::Buffer::new().format(first));

    for v in values {
        buf.push(',');
        buf.push_str(itoa::Buffer::new().format(v));
    }

    buf
}

fn parse_input(input: &str) -> Result<(Cpu, Vec<u3>)> {
    let mut lines = input.lines();

    let reg_a = lines.next().context("missing register A").and_then(|l| parse_register(l, "A"))?;
    let reg_b = lines.next().context("missing register B").and_then(|l| parse_register(l, "B"))?;
    let reg_c = lines.next().context("missing register C").and_then(|l| parse_register(l, "C"))?;

    if lines.next() != Some("") {
        anyhow::bail!("missing newline after registers");
    }

    let memory = lines
        .remainder()
        .and_then(|s| s.strip_prefix("Program:"))
        .context("missing instructions")?
        .trim()
        .split(',')
        .enumerate()
        .map(|(i, s)| s.parse::<u3>().with_context(|| format!("invalid data at {i}")))
        .collect::<Result<Vec<_>>>()?;

    Ok((Cpu { reg_a, reg_b, reg_c, pc: 0 }, memory))
}

fn parse_register(line: &str, reg: &str) -> Result<Int> {
    static REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"Register ([^:]+): (\d+)").unwrap());

    let captures = REGEX.captures(line).context("failed to parse register")?;

    let (_, [id, value]) = captures.extract();

    anyhow::ensure!(id == reg, "expected register {reg}, but found {id}");

    value.parse().with_context(|| format!("value does not fit in register: {value}"))
}

#[cfg(test)]
mod example1 {
    const EXAMPLE: &str = include_str!("./example1");

    #[test]
    fn part1() {
        let (cpu, rom) = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part1(cpu, &rom), "4,6,3,5,6,3,5,2,1,0");
    }
}

#[cfg(test)]
mod example2 {
    const EXAMPLE: &str = include_str!("./example2");

    #[test]
    fn part1() {
        let (mut cpu, rom) = super::parse_input(EXAMPLE).unwrap();

        cpu.reg_a = 117440;

        assert_eq!(super::part1(cpu, &rom), super::render_values(rom));
    }

    #[test]
    fn part2() {
        let (cpu, rom) = super::parse_input(EXAMPLE).unwrap();

        assert_eq!(super::part2(cpu, &rom), 117440);
    }
}
