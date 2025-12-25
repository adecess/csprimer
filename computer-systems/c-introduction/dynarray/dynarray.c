#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

#define STARTING_CAPACITY 8

typedef struct DA {
    int capacity;
    void **array;
    int length;
} DA;


DA* DA_new (void) {
    DA *dynamic_array_p = malloc(sizeof(DA));
    if(!dynamic_array_p) {
        fprintf(stderr, "Out of memory\n");
        exit(EXIT_FAILURE);
    }

    dynamic_array_p->capacity = STARTING_CAPACITY;
    dynamic_array_p->length = 0;
    dynamic_array_p->array = malloc(dynamic_array_p->capacity * sizeof(void *));
    if(!dynamic_array_p->array) {
        free(dynamic_array_p);
        fprintf(stderr, "Out of memory\n");
        exit(EXIT_FAILURE);
    }

    return dynamic_array_p;
}

int DA_size(DA *da) {
    return da->length;
}

void DA_push (DA* da, void* x) {
  // TODO push to the end
}

void *DA_pop(DA *da) {
  // TODO pop from the end
}

void DA_set(DA *da, void *x, int i) {
  // TODO set at a given index, if possible
}

void *DA_get(DA *da, int i) {
  // TODO get from a given index, if possible
}


void DA_free(DA *da) {
    if (da->array) {
        free(da->array);
    }
    free(da);
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
    DA *da2 = DA_new(); // use another DA to show it doesn't get overriden
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
    for (; n; n--)
      DA_pop(da);
    assert(DA_size(da) == 0);
    assert(DA_pop(da2) == &x); // this will fail if da doesn't expand

    DA_free(da);
    DA_free(da2);
    printf("OK\n");
}
