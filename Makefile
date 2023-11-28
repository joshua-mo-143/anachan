up:
	cargo shuttle run
css:
	lessc templates/styles.less templates/styles.css

re:
	make css && make up
deploy:
	cargo shuttle deploy
ddeploy:
	cargo shuttle deploy --no-test --allow-dirty

