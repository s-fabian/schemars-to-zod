z.intersection(
  z.discriminatedUnion('kind', [
    z.object({
      key: z.string(),
      kind: z.literal('Option1'),
    }),
    z.object({
      key2: z.int32().check(z.minimum(0)),
      kind: z.literal('Option2'),
    }),
  ]),
  z.object({
    a: z.int32(),
    date: z.nullish(z.date()),
    max: z.nullish(z.int32().check(z.minimum(0))),
  }),
);
