//===-- HelloWorld.cpp - Example Transformations --------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
// test
//===----------------------------------------------------------------------===//

#include "llvm/Transforms/Utils/DeadCodeElimination.h"
#include "llvm/IR/Instruction.h"

#include <unordered_set>

using namespace llvm;

PreservedAnalyses DeadCodeEliminationPass::run(Function &F,
                                               FunctionAnalysisManager &AM) {
  errs() << F.getName() << "\n";

  std::unordered_set<std::string> dead_names;
  int iteration = 0;

  do {
    // clear the names for the next round
    dead_names.clear();

    // iterate through all intstructions
    for (BasicBlock &bb : F) {
      for (Instruction &instr : bb) {

        // iterate through all operands of the instructions
        auto numOperands = instr.getNumOperands();
        for (unsigned int i = 0; i < numOperands; i++) {
          auto v = instr.getOperand(i);
          auto id = v->getNameOrAsOperand();

          // if the remove the operands from the dead_names set
          // because they are alive
          dead_names.erase(id);
        }

        // terminators do not have results so skip them
        if (instr.isTerminator()) {
          continue;
        }

        // skip if the instruction could have side effects (like store or a function call) 
        if (instr.mayHaveSideEffects()) {
            continue;
        }

        std::string value_name = instr.getNameOrAsOperand();
        // insert the result name as a potential dead name
        dead_names.insert(value_name);
      }
    }

    // accumulate all instructions that have to be removed
    std::vector<Instruction *> to_remove;
    for (BasicBlock &bb : F) {
      for (Instruction &i_it : bb) {
        auto name = i_it.getNameOrAsOperand();
        if (dead_names.find(name) != dead_names.end()) {
          errs() << "[" << iteration << "] erasing " << i_it << "\n";
          to_remove.push_back(&i_it);
        }
      }
    }

    for (Instruction *i : to_remove) {
      i->eraseFromParent();
    }

    iteration += 1;
  } while (
      !dead_names.empty()); // repeat while until there are no dead instructions

  return PreservedAnalyses::all();
}
