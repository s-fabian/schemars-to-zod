z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('justTheName') }),
  z.object({
    kind: z.literal('nameAndSingleValue'),
    value: z.int32(),
  }),
  z.object({
    kind: z.literal('nameAndTuple'),
    value: z.tuple([z.string(), z.string(), z.string()]),
  }),
  z.object({
    kind: z.literal('nameAndObject'),
    value: z.object({ int: z.int32(), prop: z.string() }),
  }),
]);
