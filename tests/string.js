z.object({
  birthday: z.coerce.date(),
  createdAt: z.coerce.date(),
  name: z.string(),
  userId: z.guid(),
});
