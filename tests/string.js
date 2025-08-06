z.object({
  birthday: z.date(),
  createdAt: z.date(),
  name: z.string(),
  userId: z.uuid(),
});
