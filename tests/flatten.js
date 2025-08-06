z.intersection(
  z.discriminatedUnion('kind', [
    z.object({
      key: z.string(),
      kind: z.literal('Option1'),
    }),
    z.object({
      key2: z.uint32(),
      kind: z.literal('Option2'),
    }),
  ]),
  z.object({
    a: z.int32(),
    date: z.nullish(z.iso.date()),
    max: z.nullish(z.uint32()),
  }),
);
