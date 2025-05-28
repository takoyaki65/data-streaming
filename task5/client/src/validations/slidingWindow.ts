import { z } from "zod";

const slidingWindowSchema = z.object({
  window: z.number().positive().finite(),
  slide: z.number().positive().finite(),
});

export { slidingWindowSchema };
export type SlidingWindowSchemaType = z.infer<typeof slidingWindowSchema>;
