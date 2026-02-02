#include <assert.h>
#include <stdio.h>

extern int matrix(int* matrix, int rows, int cols, int rindex, int cindex);

int main(void) {
  int matrix1[1][4] = {{1, 2, 3, 4}};
  assert(matrix((int*)matrix1, 1, 4, 0, 2) == 3);

  int matrix2[4][1] = {{1}, {2}, {3}, {4}};
  assert(matrix((int*)matrix2, 4, 1, 1, 0) == 2);

  int matrix3[2][3] = {{1, 2, 3}, {4, 5, 6}};
  assert(matrix((int*)matrix3, 2, 3, 1, 2) == 6);

  printf("OK\n");
}
