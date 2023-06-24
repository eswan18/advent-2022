use crate::instruction::Instruction;

pub struct CpuState {
    pub cycles: usize,
    pub value: i32,
    instructions: Vec<Instruction>,
    // Here we keep track of how many cycles we are into a multiple-cycle instruction.
    instruction_in_progress: Option<(Instruction, usize)>,
}

impl CpuState {
    pub fn new() -> CpuState {
        CpuState {
            cycles: 0,
            value: 1,
            instructions: Vec::new(),
            instruction_in_progress: None,
        }
    }

    pub fn queue_instructions(&mut self, instructions: &mut Vec<Instruction>) {
        self.instructions.append(instructions)
    }

    pub fn tick(&mut self) -> Result<(), String> {
        if self.instructions.is_empty() && self.instruction_in_progress.is_none(){
            return Err("No instructions queued".to_string())
        }
        self.cycles += 1;

        // Handle the case where we're in the middle of a multi-cycle instruction.
        if let Some((instr, cycles)) = self.instruction_in_progress.take() {
            if cycles + 1 == instr.cycles() {
                self.instruction_in_progress = None;
                self.value = instr.transform_value(self.value);
            } else {
                self.instruction_in_progress = Some((instr, cycles + 1));
            }
            return Ok(())
        }

        // Handle a new instruction.
        let next_instr = self.instructions.remove(0);
        if next_instr.cycles() == 1 {
            self.value = next_instr.transform_value(self.value);
        } else {
            self.instruction_in_progress = Some((next_instr, 1));
        }
        Ok(())
    }

    pub fn signal_strength(&self) -> i32 {
        self.value * self.cycles as i32
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let mut ins = vec![
            Instruction::Noop,
            Instruction::AddX(3),
            Instruction::AddX(-5),
        ];
        let mut cpu = CpuState::new();
        cpu.queue_instructions(&mut ins);

        assert_eq!(cpu.cycles, 0);
        // Cycle 1; do Noop
        cpu.tick().unwrap();
        assert_eq!(cpu.cycles, 1);
        assert_eq!(cpu.value, 1);
        // Cycle 2; first half of AddX(3)
        cpu.tick().unwrap();
        assert_eq!(cpu.cycles, 2);
        assert_eq!(cpu.value, 1);
        // Cycle 3; complete AddX(3)
        cpu.tick().unwrap();
        assert_eq!(cpu.cycles, 3);
        assert_eq!(cpu.value, 4);
        // Cycle 4; first half of AddX(-5)
        cpu.tick().unwrap();
        assert_eq!(cpu.cycles, 4);
        assert_eq!(cpu.value, 4);
        // Cycle 5; complete AddX(-5)
        cpu.tick().unwrap();
        assert_eq!(cpu.cycles, 5);
        assert_eq!(cpu.value, -1);
    }
}