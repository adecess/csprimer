#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/select.h>
#include <sys/types.h>

#define STARTING_BUCKETS 8
#define MAX_KEY_SIZE 64

typedef struct Item {
  char* key;
  void* value;
  struct Item* next;
} Item;

typedef struct Hashmap {
  Item** buckets;
  int total_entries;
  int total_buckets;
} Hashmap;

uint32_t hash(char* key, int total_buckets) {
  uint32_t key_hash = 0;
  for (int i = 0; key[i] != '\0'; i++) {
    key_hash += (unsigned char)key[i];
  }

  return key_hash % total_buckets;
}

Hashmap* Hashmap_new(void) {
  Hashmap* hashmap = malloc(sizeof(Hashmap));
  if (!hashmap) {
    fprintf(stderr, "Out of memory\n");
    exit(EXIT_FAILURE);
  }

  hashmap->total_buckets = STARTING_BUCKETS;
  hashmap->total_entries = 0;
  // initialize underlying array with null pointers
  hashmap->buckets = calloc(hashmap->total_buckets, sizeof(Item*));
  if (!hashmap->buckets) {
    fprintf(stderr, "Out of memory\n");
    exit(EXIT_FAILURE);
  }

  return hashmap;
}

// TODO
void Hashmap_resize(Hashmap* h) {}

void Hashmap_set(Hashmap* h, char* key, void* value) {
  uint32_t key_hash = hash(key, h->total_buckets);
  Item* previous_item = NULL;
  Item* current_item = h->buckets[key_hash];

  for (;;) {
    if (!current_item) {
      Item* new_item = malloc(sizeof(Item));
      if (!new_item) {
        fprintf(stderr, "Out of memory\n");
        exit(EXIT_FAILURE);
      }
      *new_item = (Item){.key = key, .value = value, .next = NULL};

      if (previous_item) {
        previous_item->next = new_item;
      }
      h->buckets[key_hash] = new_item;
      h->total_entries += 1;
      if (h->total_entries >= h->total_buckets / 2) Hashmap_resize(h);

      return;
    } else if (current_item->key == key) {
      h->buckets[key_hash]->value = value;
      return;
    }

    previous_item = current_item;
    current_item = current_item->next;
  }
}

void* Hashmap_get(Hashmap* h, char* key) {
  uint32_t key_hash = hash(key, h->total_buckets);
  Item* current_item = h->buckets[key_hash];

  while (current_item) {
    if (current_item->key == key) {
      return current_item->value;
    }

    current_item = current_item->next;
  }

  return NULL;
}

// TOFIX
void Hashmap_delete(Hashmap* h, char* key) {
  uint32_t key_hash = hash(key, h->total_buckets);

  if (h->buckets[key_hash]) h->buckets[key_hash] = NULL;
}

// TOFIX
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
     maintaing key ordering etc. see
     https://www.youtube.com/watch?v=npw4s1QTmPg for ideas
     */
  printf("ok\n");
}
