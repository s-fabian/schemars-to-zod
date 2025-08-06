z.union([
  z.object({ kind: z.literal('justTheName') }),
  z.union([
    z.object({ kind: z.literal('nameAndSingleValue') }),
    z.int32(),
  ]),
  z.object({
    int: z.int32(),
    kind: z.literal('nameAndObject'),
    prop: z.string(),
  }),
]);
