

int string_compare(char *s, char *t, cell m[MAXLEN + 1][MAXLEN + 1]) {
  int i, j, k; /* counters */
  int opt[3];  /* cost of the three options */

  for (i = 0; i <= MAXLEN; i++) {
    row_init(i, m);
    column_init(i, m);
  }

  for (i = 1; i < strlen(s); i++) {
    for (j = 1; j < strlen(t); j++) {

      opt[MATCH] = m[i - 1][j - 1].cost + match(s[i], t[j]);
      opt[INSERT] = m[i][j - 1].cost + indel(t[j]);
      opt[DELETE] = m[i - 1][j].cost + indel(s[i]);

      m[i][j].cost = opt[MATCH];
      m[i][j].parent = MATCH;

      for (k = INSERT; k <= DELETE; k++) {
        if (opt[k] < m[i][j].cost) {
          m[i][j].cost = opt[k];
          m[i][j].parent = k;
        }
      }
    }
  }

  goal_cell(s, t, &i, &j);
  return (m[i][j].cost);
}
