z.union([
  z.object({ int: z.number().int(), prop: z.string() }),
  z.object({ name: z.string(), prop: z.number().int() }),
]);
