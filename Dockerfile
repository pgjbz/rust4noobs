FROM docker.io/allfunc/mdbook:0.4.36 AS BUILDER
WORKDIR /book
COPY . .
RUN [ "mdbook", "build" ]

FROM docker.io/httpd:alpine3.19 as RUNNER
COPY --from=BUILDER /book/book /usr/local/apache2/htdocs/
EXPOSE 80