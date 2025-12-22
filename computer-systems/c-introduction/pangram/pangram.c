#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

bool ispangram(char *s) {
    size_t len = strlen(s);
    char unique_letters[len] = {};

    while (*s != '\n') {
        char c = tolower(*s);
        if (strchr(unique_letters, c) == NULL) {
            // Safe to append since character is not present in buffer
            size_t pos = strlen(unique_letters);
            if (isalpha(*s)) {
                unique_letters[pos] = c;
                unique_letters[pos + 1] = '\0';
            }
        }
        s++; // go to next character
    }

  return strlen(unique_letters) == 26;
}

int main() {
  size_t len;
  ssize_t read;
  char *line = NULL;
  while ((read = getline(&line, &len, stdin)) != -1) {
    if (ispangram(line))
      printf("%s", line);
  }

  if (ferror(stdin))
    fprintf(stderr, "Error reading from stdin");

  free(line);
  fprintf(stderr, "ok\n");
}
