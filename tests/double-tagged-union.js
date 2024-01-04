z.discriminatedUnion('kind', [
  z.object({ kind: z.literal('justTheName') }),
  z.object({
    kind: z.literal('nameAndSingleValue'),
    value: z.number().int(),
  }),
  z.object({
    kind: z.literal('nameAndTuple'),
    value: z.tuple([z.string(), z.string(), z.string()]),
  }),
  z.object({
    kind: z.literal('nameAndObject'),
    value: z.object({
      int: z.number().int(),
      prop: z.string(),
    }),
  }),
]);
