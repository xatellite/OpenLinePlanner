# build stage
FROM node:16-alpine as build-stage
WORKDIR /app
COPY . .
RUN yarn
RUN yarn build

# production stage
FROM nginx:stable-alpine as production-stage
COPY --from=build-stage /app/dist /srv/www
COPY ./nginx.conf /etc/nginx/custom.conf
EXPOSE 80
CMD ["nginx", "-c", "/etc/nginx/custom.conf", "-g", "daemon off;"]