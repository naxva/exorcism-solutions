#include "protein_translation.h"
#include <string.h>

protein_t protein(const char *const rna) {
  protein_t result = {.valid = true, .count = 0};

  if (!rna) {
    result.valid = false;
    return result;
  }

  const char *ptr = rna;

  while (*ptr != '\0') {
    if (strlen(ptr) < 3) {
      result.valid = false;
      return result;
    }

    if (strncmp(ptr, "AUG", 3) == 0) {
      if (result.count < MAX_AMINO_ACIDS) {
        result.amino_acids[result.count++] = Methionine;
      }
    } else if (strncmp(ptr, "UUU", 3) == 0 || strncmp(ptr, "UUC", 3) == 0) {
      if (result.count < MAX_AMINO_ACIDS) {
        result.amino_acids[result.count++] = Phenylalanine;
      }
    } else if (strncmp(ptr, "UUA", 3) == 0 || strncmp(ptr, "UUG", 3) == 0) {
      if (result.count < MAX_AMINO_ACIDS) {
        result.amino_acids[result.count++] = Leucine;
      }
    } else if (strncmp(ptr, "UCU", 3) == 0 || strncmp(ptr, "UCC", 3) == 0 ||
               strncmp(ptr, "UCA", 3) == 0 || strncmp(ptr, "UCG", 3) == 0) {
      if (result.count < MAX_AMINO_ACIDS) {
        result.amino_acids[result.count++] = Serine;
      }
    } else if (strncmp(ptr, "UAU", 3) == 0 || strncmp(ptr, "UAC", 3) == 0) {
      if (result.count < MAX_AMINO_ACIDS) {
        result.amino_acids[result.count++] = Tyrosine;
      }
    } else if (strncmp(ptr, "UGU", 3) == 0 || strncmp(ptr, "UGC", 3) == 0) {
      if (result.count < MAX_AMINO_ACIDS) {
        result.amino_acids[result.count++] = Cysteine;
      }
    } else if (strncmp(ptr, "UGG", 3) == 0) {
      if (result.count < MAX_AMINO_ACIDS) {
        result.amino_acids[result.count++] = Tryptophan;
      }
    } else if (strncmp(ptr, "UAA", 3) == 0 || strncmp(ptr, "UAG", 3) == 0 ||
               strncmp(ptr, "UGA", 3) == 0) {
      break;
    } else {
      result.valid = false;
      return result;
    }

    ptr += 3;
  }

  return result;
}
