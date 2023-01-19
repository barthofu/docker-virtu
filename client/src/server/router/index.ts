import { publicProcedure, router } from '@server/trpc'

import { randomRouter } from './random'

export const appRouter = router({

    random: randomRouter
})
   
export type AppRouter = typeof appRouter