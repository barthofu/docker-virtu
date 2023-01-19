# ==================
# == dependencies ==
# ==================

FROM node:16-alpine as deps

    WORKDIR /app

    # as alpine is really light, we install libc6-compat to be able to run post-install npm scripts from next-js dependencies 
    RUN apk add --no-cache libc6-compat
    # copy only the package.json and package-lock.json in order to install dependencies
    COPY package.json package-lock.json ./
    # install dependencies
    RUN npm ci

# ==================
# ==== builder =====
# ==================

FROM node:16-alpine as builder

    WORKDIR /app

    # copy the dependencies from the previous stage
    COPY --from=deps /app/node_modules ./node_modules
    # copy the rest of the source code
    COPY . .
    # this ensure next-js will not send telemetry data during build
    ENV NEXT_TELEMETRY_DISABLED 1

    # build the app
    RUN npm run build

# ==================
# ==== runner ======
# ==================

FROM node:16-alpine as runner

    WORKDIR /app

    ENV NODE_ENV production
    # this ensure next-js will not send telemetry data during runtime
    ENV NEXT_TELEMETRY_DISABLED 1

    # create a user and a group to run the app more securely and properly
    RUN addgroup --system --gid 1001 nodejs \
        && adduser --system --uid 1001 nextjs

    # copy all the required files from the previous stage
    COPY --from=builder /app/public ./public
    COPY --from=builder --chown=nextjs:nodejs /app/.next/standalone ./
    COPY --from=builder --chown=nextjs:nodejs /app/.next/static ./.next/static

    # switch to the nextjs user
    USER nextjs

    # start the app
    CMD ["node", "server.js"]