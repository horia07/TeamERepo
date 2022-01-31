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

  // accumulate all instructions that have to be removed
  std::vector<Instruction *> to_remove;

  do {
    to_remove.clear();
    // iterate through all intstructions
    for (BasicBlock &bb : F) {
      for (Instruction &instr : bb) {
        // terminators do not have results so skip them
        if (instr.isTerminator()) {
          continue;
        }

        // skip if the instruction could have side effects 
        // (like store or a function call)
        if (instr.mayHaveSideEffects()) {
          continue;
        }

        unsigned int uses = instr.getNumUses();
        errs() << "uses: " << uses << "\t" << instr << "\n";

        // if there are no uses for the value, add it to the remove list
        if (uses == 0) {
          std::string value_name = instr.getName().str();
          to_remove.push_back(&instr);
        }
      }
    }

    // erase all instructions from the remove list
    for (Instruction *instr : to_remove) {
      instr->eraseFromParent();
    }
  } while (!to_remove.empty());

  return PreservedAnalyses::all();
}
