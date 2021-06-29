/* [[file:nauty.note::*wrapper.c][wrapper.c:1]] */
/* Adopted from nautyex6.c */
#include "lib/nauty.h"

int dwim(int n, int nedges, int *edges_i, int *edges_j, int *lab, int *ptn) {
  /* Select option for canonical labelling */
  static DEFAULTOPTIONS_GRAPH(options);
  options.getcanon = TRUE;
  options.defaultptn = FALSE;

  if (n > 0) {
    /* allocate memory */
    int m = SETWORDSNEEDED(n);
    nauty_check(WORDSIZE, m, n, NAUTYVERSIONID);
    DYNALLSTAT(int, orbits, orbits_sz);
    DYNALLOC1(int, orbits, orbits_sz, n, "malloc");
    DYNALLSTAT(int, map, map_sz);
    DYNALLOC1(int, map, map_sz, n, "malloc");
    DYNALLSTAT(graph, g, g1_sz);
    DYNALLOC2(graph, g, g1_sz, n, m, "malloc");
    DYNALLSTAT(graph, cg, cg1_sz);
    DYNALLOC2(graph, cg, cg1_sz, n, m, "malloc");

    /* start with no edges */
    EMPTYGRAPH(g, m, n);

    /* add edges from edges_i, edges_j arrays */
    for (int i = 0; i < nedges; ++i) {
      int e1 = edges_i[i];
      int e2 = edges_j[i];
      if (e1 < e2) {
        ADDONEEDGE(g, e1, e2, m);
      } else {
        return -2;
      }
    }
    statsblk stats;
    densenauty(g, lab, ptn, orbits, &options, &stats, m, n, cg);
    return 0;
  } else {
    return -1;
  }
}
/* wrapper.c:1 ends here */
