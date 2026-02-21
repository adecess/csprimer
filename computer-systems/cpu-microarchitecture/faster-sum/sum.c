#include <immintrin.h>
#include <x86intrin.h>

typedef __m512i reg;

// sum of all the 16 partial sums
int horizontal_sum(reg x) {
  int t[16], s = 0;
  _mm512_storeu_si512((reg*)t, x);

  for (int i = 0; i < 16; i++) {
    s += t[i];
  }

  return s;
}

int sum(int* nums, int n) {
  // create a accumulator (16 element int vector filled with zeros)
  // it will hold 16 partial sums of the vector
  reg s = _mm512_setzero_si512();

  // vertical sum
  for (int i = 0; i < n; i += 16)
    /*
    Load 16 consecutive int32 values starting at nums[i]
    Example (first iteration, i = 0):
      nums = [1, 2, 3, 4, ..., 16, ...]
      loaded vector = [1, 2, 3, 4, ..., 16]

    Then perform lane-wise addition with the accumulator `s`:
      if s was initially [0, 0, 0, 0, ..., 0]
      result becomes        [1, 2, 3, 4, ..., 16]

    On the next iteration (i = 16), suppose:
      nums[16..31] = [17, 18, 19, 20, ..., 32]
      s becomes:
      [1+17, 2+18, 3+19, 4+20, ..., 16+32]
      = [18, 20, 22, 24, ..., 48]
    */
    s = _mm512_add_epi32(s, _mm512_load_si512((reg*)&nums[i]));

  return horizontal_sum(s);
}
