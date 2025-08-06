z.object({
  birthday: z.iso.date(),
  createdAt: z.iso.datetime({ offset: true, local: true }),
  name: z.string(),
  userId: z.guid(),
});
