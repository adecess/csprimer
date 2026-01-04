#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/select.h>
#include <sys/types.h>

#define STARTING_BUCKETS 8
#define MAX_KEY_SIZE 64
typedef uint32_t Hash;

typedef struct ListItem {
  char* key;
  void* value;
  Hash hash;  // use hash instead of key pointer for comparisons
  struct ListItem* next;
} ListItem;

typedef struct Hashmap {
  ListItem** buckets;
  int total_entries;
  int total_buckets;
} Hashmap;

Hash djb2(const char* key) {
  Hash key_hash = 5381;
  char c;
  while ((c = *key++)) {
    key_hash = ((key_hash << 5) + key_hash) + c;
  }

  return key_hash;
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
  hashmap->buckets = calloc(hashmap->total_buckets, sizeof(ListItem*));
  if (!hashmap->buckets) {
    fprintf(stderr, "Out of memory\n");
    exit(EXIT_FAILURE);
  }

  return hashmap;
}

void Hashmap_resize(Hashmap* h) {
  h->total_buckets *= 2;
  h = realloc(h, h->total_buckets);
  if (!h) {
    fprintf(stderr, "Out of memory\n");
    exit(EXIT_FAILURE);
  }
}

void Hashmap_set(Hashmap* h, char* key, void* value) {
  Hash key_hash = djb2(key);
  ListItem* previous_item = NULL;
  ListItem* current_item = h->buckets[key_hash % h->total_buckets];

  for (;;) {
    if (current_item == NULL) {
      ListItem* new_item = malloc(sizeof(ListItem));
      if (!new_item) {
        fprintf(stderr, "Out of memory\n");
        exit(EXIT_FAILURE);
      }

      new_item->key =
          strdup(key);  // avoid assigning the same character pointer
      new_item->value = value;
      new_item->hash = key_hash;
      new_item->next = NULL;

      if (previous_item) {
        previous_item->next = new_item;
      }
      h->buckets[key_hash % h->total_buckets] = new_item;
      h->total_entries += 1;
      if (h->total_entries >= h->total_buckets / 2) Hashmap_resize(h);

      return;
    } else if (current_item->key == key) {
      h->buckets[key_hash % h->total_buckets]->value = value;
      return;
    }

    previous_item = current_item;
    current_item = current_item->next;
  }
}

void* Hashmap_get(Hashmap* h, char* key) {
  Hash key_hash = djb2(key);
  ListItem* current_item = h->buckets[key_hash % h->total_buckets];

  while (current_item != NULL) {
    if (current_item->hash == key_hash) {
      return current_item->value;
    }

    current_item = current_item->next;
  }

  return NULL;
}

void Hashmap_delete(Hashmap* h, char* key) {
  Hash key_hash = djb2(key);

  ListItem* previous_item = NULL;
  ListItem* current_item = h->buckets[key_hash % h->total_buckets];

  for (;;) {
    if (current_item->hash == key_hash) {
      if (previous_item) {
        previous_item->next = current_item->next;
      } else {
        // next item becomes head
        h->buckets[key_hash % h->total_buckets] = current_item->next;
      }

      free(current_item);
      return;
    }

    previous_item = current_item;
    current_item = current_item->next;
  }
}

void Hashmap_free(Hashmap* hashmap) {
  for (int i = 0; i < hashmap->total_buckets; i++) {
    if (hashmap->buckets[i]) {
      ListItem* current_item = hashmap->buckets[i];
      while (current_item != NULL) {
        ListItem* item = current_item;
        current_item = current_item->next;

        free(item);
      }
    }
  }

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
