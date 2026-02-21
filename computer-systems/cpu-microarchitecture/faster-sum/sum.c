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
    // Load 16 ints: nums[i..i+15] into a 512-bit vector
    // Example: nums = [1,2,3,...,16,17,18,...]
    //   1st iter: s = [0,...,0] + [1,2,3,...,16]
    //            = [1,2,3,...,16]
    //   2nd iter: s + [17,18,...,32]
    //            = [1+17, 2+18, ..., 16+32]
    // (lane-wise / vertical accumulation, not a horizontal sum)
    s = _mm512_add_epi32(s, _mm512_load_si512((reg*)&nums[i]));

  return horizontal_sum(s);
}
