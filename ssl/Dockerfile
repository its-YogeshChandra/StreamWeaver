#image of the certbot 
FROM certbot/certbot:nightly

#copy the value in the container
COPY renew-entrypoint.sh ./ 

#make the entrypoint file executable
RUN chmod +x renew-entrypoint.sh 

#entrypoint for the file 
ENTRYPOINT [ "./renew-entrypoint.sh" ]



