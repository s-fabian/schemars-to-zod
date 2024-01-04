z.object({
  admin: z.boolean(),
  age: z.number().int().nullable().optional(),
}).array();
