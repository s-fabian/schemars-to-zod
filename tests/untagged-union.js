z.union([
  z.object({ int: z.int32(), prop: z.string() }),
  z.object({ name: z.string(), prop: z.int32() }),
]);
