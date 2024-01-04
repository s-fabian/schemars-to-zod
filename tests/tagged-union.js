z.union([
  z.object({ kind: z.literal('justTheName') }),
  z.union([
    z.object({ kind: z.literal('nameAndSingleValue') }),
    z.number().int(),
  ]),
  z.object({
    int: z.number().int(),
    kind: z.literal('nameAndObject'),
    prop: z.string(),
  }),
]);
