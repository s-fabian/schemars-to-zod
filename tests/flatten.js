z.object({
  a: z.number().int(),
  date: z.date().nullable().optional(),
  max: z.number().int().min(0).nullable().optional(),
}).and(
  z.discriminatedUnion('kind', [
    z.object({
      key: z.string(),
      kind: z.literal('Option1'),
    }),
    z.object({
      key2: z.number().int().min(0),
      kind: z.literal('Option2'),
    }),
  ]),
);
