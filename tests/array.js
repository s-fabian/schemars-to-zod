z.array(
  z.object({
    admin: z.boolean(),
    age: z.nullish(z.int32()),
  }),
);
