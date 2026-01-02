#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#define STARTING_CAPACITY 8

typedef struct DA {
  void** item;
  int length;
  int capacity;
} DA;

DA* DA_new(void) {
  DA* dynamic_array_p = malloc(sizeof(DA));
  if (!dynamic_array_p) {
    fprintf(stderr, "Out of memory\n");
    exit(EXIT_FAILURE);
  }

  dynamic_array_p->capacity = STARTING_CAPACITY;
  dynamic_array_p->length = 0;
  dynamic_array_p->item = malloc(dynamic_array_p->capacity * sizeof(void*));
  if (!dynamic_array_p->item) {
    free(dynamic_array_p);
    fprintf(stderr, "Out of memory\n");
    exit(EXIT_FAILURE);
  }

  return dynamic_array_p;
}

void DA_free(DA* da) {
  if (da->item) {
    free(da->item);
  }
  free(da);
}

int DA_size(DA* da) { return da->length; }

void DA_resize(DA* da) {
  da->capacity *= 2;
  da->item = realloc(da->item, da->capacity * sizeof(void*));
  if (!da->item) {
    free(da->item);
    fprintf(stderr, "Out of memory\n");
    exit(EXIT_FAILURE);
  }
}

void DA_push(DA* da, void* x) {
  if (da->length == da->capacity) DA_resize(da);

  da->item[da->length++] = x;
}

void* DA_pop(DA* da) {
  if (da->length > 0) {
    return da->item[--da->length];
  }
  return NULL;
}

void DA_set(DA* da, void* x, int i) {
  if (i < da->length) da->item[i] = x;
}

void* DA_get(DA* da, int i) {
  if (i < da->length) return da->item[i];
  return NULL;
}

int main() {
  DA* da = DA_new();

  assert(DA_size(da) == 0);

  // basic push and pop test
  int x = 5;
  float y = 12.4;
  DA_push(da, &x);
  DA_push(da, &y);
  assert(DA_size(da) == 2);

  assert(DA_pop(da) == &y);
  assert(DA_size(da) == 1);

  assert(DA_pop(da) == &x);
  assert(DA_size(da) == 0);
  assert(DA_pop(da) == NULL);

  // basic set/get test
  DA_push(da, &x);
  DA_set(da, &y, 0);
  assert(DA_get(da, 0) == &y);
  DA_pop(da);
  assert(DA_size(da) == 0);

  // expansion test
  DA* da2 = DA_new();  // use another DA to show it doesn't get overriden
  DA_push(da2, &x);
  int i, n = 100 * STARTING_CAPACITY, arr[n];
  for (i = 0; i < n; i++) {
    arr[i] = i;
    DA_push(da, &arr[i]);
  }
  assert(DA_size(da) == n);
  for (i = 0; i < n; i++) {
    assert(DA_get(da, i) == &arr[i]);
  }
  for (; n; n--) DA_pop(da);
  assert(DA_size(da) == 0);
  assert(DA_pop(da2) == &x);  // this will fail if da doesn't expand

  DA_free(da);
  DA_free(da2);
  printf("OK\n");
}
