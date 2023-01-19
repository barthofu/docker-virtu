import { publicProcedure, router } from '@server/trpc'
import { TRPCError } from '@trpc/server'
import { z } from 'zod'

export const randomRouter = router({

    getRandom: publicProcedure
        .query(async () => {
            
            try {
                
                const res = await fetch(process.env['API_URL'] + '/random')
                const data = await res.json()

                if (!data) {
                    throw new TRPCError({
                        code: 'INTERNAL_SERVER_ERROR',
                        message: 'invalid name!'
                    })    
                }
                
                return data

            } catch (error) {

                console.error(error)

                throw new TRPCError({
                    code: 'INTERNAL_SERVER_ERROR',
                    message: 'invalid name!'
                })
            }
        })

})