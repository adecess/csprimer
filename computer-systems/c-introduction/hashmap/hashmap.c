#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/select.h>
#include <sys/types.h>

#define STARTING_BUCKETS 8

typedef struct {
  char* key;
  void* value;
  struct Bucket* next;
} Bucket;

typedef struct {
  Bucket** buckets;
  int size;
  int bucket_capacity;
} Hashmap;

Hashmap* Hashmap_new(void) {
  Hashmap* hashmap = malloc(sizeof(Hashmap));
  if (!hashmap) {
    fprintf(stderr, "Out of memory\n");
    exit(EXIT_FAILURE);
  }

  hashmap->bucket_capacity = STARTING_BUCKETS;
  hashmap->size = 0;
  hashmap->buckets = calloc(
      hashmap->bucket_capacity,
      sizeof(Bucket*));  // initialize underlying array with null pointers
  if (!hashmap->buckets) {
    fprintf(stderr, "Out of memory\n");
    exit(EXIT_FAILURE);
  }

  return hashmap;
}

void Hashmap_set(Hashmap* h, char* key, void* value) {
  uint32_t key_hash = 0;
  for (int i = 0; key[i] != '\0'; i++) {
    key_hash += (unsigned char)key[i];
  }

  Bucket* bucket = malloc(sizeof(Bucket));
  bucket->key = key;
  bucket->value = value;
  bucket->next = NULL;

  int bucket_index = key_hash % h->bucket_capacity;
  h->buckets[bucket_index] = bucket;
}

void* Hashmap_get(Hashmap* h, char* key) {}

void Hashmap_delete(Hashmap* h, char* key) {}

void Hashmap_free(Hashmap* hashmap) {
  if (hashmap->buckets) {
    free(hashmap->buckets);
  }
  free(hashmap);
}

int main() {
  Hashmap* h = Hashmap_new();

  // basic get/set functionality
  int a = 5;
  float b = 7.2;
  Hashmap_set(h, "item a", &a);
  Hashmap_set(h, "item b", &b);
  assert(Hashmap_get(h, "item a") == &a);
  assert(Hashmap_get(h, "item b") == &b);

  // using the same key should override the previous value
  int c = 20;
  Hashmap_set(h, "item a", &c);
  assert(Hashmap_get(h, "item a") == &c);

  // basic delete functionality
  Hashmap_delete(h, "item a");
  assert(Hashmap_get(h, "item a") == NULL);

  // handle collisions correctly
  // note: this doesn't necessarily test expansion
  int i, n = STARTING_BUCKETS * 10, ns[n];
  char key[MAX_KEY_SIZE];
  for (i = 0; i < n; i++) {
    ns[i] = i;
    sprintf(key, "item %d", i);
    Hashmap_set(h, key, &ns[i]);
  }
  for (i = 0; i < n; i++) {
    sprintf(key, "item %d", i);
    assert(Hashmap_get(h, key) == &ns[i]);
  }

  Hashmap_free(h);
  /*
     stretch goals:
     - expand the underlying array if we start to get a lot of collisions
     - support non-string keys
     - try different hash functions
     - switch from chaining to open addressing
     - use a sophisticated rehashing scheme to avoid clustered collisions
     - implement some features from Python dicts, such as reducing space use,
     maintaing key ordering etc. see https://www.youtube.com/watch?v=npw4s1QTmPg
     for ideas
     */
  printf("ok\n");
}
